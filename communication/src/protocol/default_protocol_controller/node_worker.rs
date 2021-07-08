use super::super::{
    binders::{ReadBinder, WriteBinder},
    config::ProtocolConfig,
    messages::Message,
    protocol_controller::NodeId,
};
use crate::error::{ChannelError, CommunicationError};
use crate::network::network_controller::ConnectionClosureReason;
use models::block::Block;
use std::net::IpAddr;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::timeout;

/// Commands that node worker can manage.
#[derive(Clone, Debug)]
pub enum NodeCommand {
    /// Send given peer list to node.
    SendPeerList(Vec<IpAddr>),
    /// Send that block to node.
    SendBlock(Block),
    /// Send that transation to node.
    SendTransaction(String),
    /// Close the node worker.
    Close,
}

/// Event types that node worker can emit
#[derive(Clone, Debug)]
pub enum NodeEventType {
    /// Node we are conneced to asked for advertized peers
    AskedPeerList,
    /// Node we are conneced to sent peer list
    ReceivedPeerList(Vec<IpAddr>),
    /// Node we are conneced to sent block
    ReceivedBlock(Block),
    /// Node we are conneced to sent transaction
    ReceivedTransaction(String),
    /// Connection with node was shut down for given reason
    Closed(ConnectionClosureReason),
}

/// Events node worker can emit.
/// Events are a tuple linking a node id to an event type
#[derive(Clone, Debug)]
pub struct NodeEvent(pub NodeId, pub NodeEventType);

/// Manages connections
/// One worker per node.
pub struct NodeWorker<ReaderT: 'static, WriterT: 'static>
where
    ReaderT: AsyncRead + Send + Sync + Unpin,
    WriterT: AsyncWrite + Send + Sync + Unpin,
{
    /// Protocol configuration.
    cfg: ProtocolConfig,
    /// Node id associated to that worker.
    node_id: NodeId,
    /// Reader for incomming data.
    socket_reader: ReadBinder<ReaderT>,
    /// Optional writer to send data.
    socket_writer_opt: Option<WriteBinder<WriterT>>,
    /// Channel to receive node commands.
    node_command_rx: Receiver<NodeCommand>,
    /// Channel to send node events.
    node_event_tx: Sender<NodeEvent>,
}

impl<ReaderT: 'static, WriterT: 'static> NodeWorker<ReaderT, WriterT>
where
    ReaderT: AsyncRead + Send + Sync + Unpin,
    WriterT: AsyncWrite + Send + Sync + Unpin,
{
    /// Creates a new node worker
    ///
    /// # Arguments
    /// * cfg: Protocol configuration.
    /// * node_id: Node id associated to that worker.
    /// * socket_reader: Reader for incomming data.
    /// * socket_writer: Writer for sending data.
    /// * node_command_rx: Channel to receive node commands.
    /// * node_event_tx: Channel to send node events.
    pub fn new(
        cfg: ProtocolConfig,
        node_id: NodeId,
        socket_reader: ReadBinder<ReaderT>,
        socket_writer: WriteBinder<WriterT>,
        node_command_rx: Receiver<NodeCommand>,
        node_event_tx: Sender<NodeEvent>,
    ) -> NodeWorker<ReaderT, WriterT> {
        NodeWorker {
            cfg,
            node_id,
            socket_reader,
            socket_writer_opt: Some(socket_writer),
            node_command_rx,
            node_event_tx,
        }
    }

    /// node event loop. Consumes self.
    pub async fn run_loop(mut self) -> Result<(), CommunicationError> {
        let (writer_command_tx, mut writer_command_rx) = mpsc::channel::<Message>(1024);
        let (writer_event_tx, mut writer_event_rx) = mpsc::channel::<bool>(1);
        let mut socket_writer =
            self.socket_writer_opt
                .take()
                .ok_or(CommunicationError::GeneralProtocolError(
                    "NodeWorker call run_loop more than once".to_string(),
                ))?;
        let write_timeout = self.cfg.message_timeout;
        let node_writer_handle = tokio::spawn(async move {
            let mut clean_exit = true;
            loop {
                match writer_command_rx.recv().await {
                    Some(msg) => {
                        if let Err(_) =
                            timeout(write_timeout.to_duration(), socket_writer.send(&msg)).await
                        {
                            clean_exit = false;
                            break;
                        }
                    }
                    None => break,
                }
            }
            writer_event_tx
                .send(clean_exit)
                .await
                .expect("writer_evt_tx died"); //in a spawned task
        });

        let mut ask_peer_list_interval =
            tokio::time::interval(self.cfg.ask_peer_list_interval.to_duration());
        let mut exit_reason = ConnectionClosureReason::Normal;
        loop {
            tokio::select! {
                // incoming socket data
                res = self.socket_reader.next() => match res {
                    Ok(Some((_, msg))) => {
                        match msg {
                            Message::Block(block) => self.node_event_tx.send(
                                    NodeEvent(self.node_id, NodeEventType::ReceivedBlock(block))
                                ).await.map_err(|err| ChannelError::from(err))?,
                            Message::Transaction(tr) =>  self.node_event_tx.send(
                                    NodeEvent(self.node_id, NodeEventType::ReceivedTransaction(tr))
                                ).await.map_err(|err| ChannelError::from(err))?,
                            Message::PeerList(pl) =>  self.node_event_tx.send(
                                    NodeEvent(self.node_id, NodeEventType::ReceivedPeerList(pl))
                                ).await.map_err(|err| ChannelError::from(err))?,
                            Message::AskPeerList => self.node_event_tx.send(
                                    NodeEvent(self.node_id, NodeEventType::AskedPeerList)
                                ).await.map_err(|err| ChannelError::from(err))?,
                            _ => {  // wrong message
                                exit_reason = ConnectionClosureReason::Failed;
                                break;
                            },
                        }
                    },
                    Ok(None)=> break, // peer closed cleanly
                    Err(_) => {  //stream error
                        exit_reason = ConnectionClosureReason::Failed;
                        break;
                    },
                },

                // node command
                cmd = self.node_command_rx.recv() => match cmd {
                    Some(NodeCommand::Close) => break,
                    Some(NodeCommand::SendPeerList(ip_vec)) => {
                        writer_command_tx.send(Message::PeerList(ip_vec)).await.map_err(|err| ChannelError::from(err))?;
                    }
                    Some(NodeCommand::SendBlock(block)) => {
                        writer_command_tx.send(Message::Block(block)).await.map_err(|err| ChannelError::from(err))?;
                    }
                    Some(NodeCommand::SendTransaction(transaction)) => {
                        writer_command_tx.send(Message::Transaction(transaction)).await.map_err(|err| ChannelError::from(err))?;
                    }
                    None => {
                        return Err(CommunicationError::UnexpectedProtocolControllerClosureError);
                    },
                },

                // writer event
                evt = writer_event_rx.recv() => match evt {
                    Some(s) => {
                        if !s {
                            exit_reason = ConnectionClosureReason::Failed;
                        }
                        break;
                    },
                    None => break
                },

                _ = ask_peer_list_interval.tick() => {
                    debug!("timer-based asking node_id={:?} for peer list", self.node_id);
                    massa_trace!("timer_ask_peer_list", {"node_id": self.node_id});
                    writer_command_tx.send(Message::AskPeerList).await.map_err(|err| ChannelError::from(err))?;
                }
            }
        }

        // close writer
        drop(writer_command_tx);
        while let Some(_) = writer_event_rx.recv().await {}
        node_writer_handle.await?;

        // notify protocol controller of closure
        self.node_event_tx
            .send(NodeEvent(self.node_id, NodeEventType::Closed(exit_reason)))
            .await
            .map_err(|err| ChannelError::from(err))?;
        Ok(())
    }
}
