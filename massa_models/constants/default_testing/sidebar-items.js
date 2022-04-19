initSidebarItems({"constant":[["ADDRESS_SIZE_BYTES","Size of the random bytes array used for the bootstrap, safe to import"],["AMOUNT_DECIMAL_FACTOR","Safe to import"],["BASE_NETWORK_CONTROLLER_IP","our default ip"],["BLOCK_ID_SIZE_BYTES","block id size"],["BLOCK_REWARD","reward for a block"],["BOOTSTRAP_RANDOMNESS_SIZE_BYTES","random bootstrap message size"],["CHANNEL_SIZE","channel size"],["CURSOR_DELAY","normally in `config.toml`, allow execution worker to lag smoothly"],["DELTA_F0","fitness threshold"],["DISABLE_BLOCK_CREATION","normally in `config.toml`, if the node will create blocks"],["ENDORSEMENT_COUNT","target endorsement count"],["ENDORSEMENT_ID_SIZE_BYTES","endorsement id size"],["EVENT_ID_SIZE_BYTES","event id size"],["FINAL_HISTORY_LENGTH","normally in `config.toml`, final history length"],["FORCE_KEEP_FINAL_PERIOD","normally in `config.toml`, forcefully kept periods"],["FUTURE_BLOCK_PROCESSING_MAX_PERIODS","normally in `config.toml`, if slot is after `FUTURE_BLOCK_PROCESSING_MAX_PERIODS`, the block is not processed"],["HANDSHAKE_RANDOMNESS_SIZE_BYTES","random handshake message size"],["INITIAL_DRAW_SEED","initial seed"],["IP_LIST_MAX_SIZE","max bootstrap ips kept size"],["LEDGER_CACHE_CAPACITY","normally in `config.toml`, ledger cache capacity"],["LEDGER_RESET_AT_STARTUP","normally in `config.toml`, if the ledger need a reset at start up"],["MAX_ADVERTISE_LENGTH","max advertised id length"],["MAX_ASK_BLOCKS_PER_MESSAGE","max ask for block per message"],["MAX_ASYNC_GAS","max asynchronous gas"],["MAX_ASYNC_POOL_LENGTH","max asynchronous pool length"],["MAX_BLOCK_SIZE","max block size 3 * 1024 * 1024"],["MAX_BOOTSTRAP_BLOCKS","max bootstrapped blocks"],["MAX_BOOTSTRAP_CHILDREN","max bootstrapped children per block"],["MAX_BOOTSTRAP_CLIQUES","max bootstrapped cliques"],["MAX_BOOTSTRAP_DEPS","max bootstrapped dependencies"],["MAX_BOOTSTRAP_MESSAGE_SIZE","max bootstrap message size"],["MAX_BOOTSTRAP_POS_CYCLES","max bootstrapped proof of take cycles"],["MAX_BOOTSTRAP_POS_ENTRIES","max bootstrapped proof of stake entries"],["MAX_DEPENDENCY_BLOCK","normally in `config.toml`, max unknown dependencies kept"],["MAX_DISCARDED_BLOCKS","normally in `config.toml`, max discarded blocks kept"],["MAX_DUPLEX_BUFFER_SIZE","max duplex buffer size"],["MAX_ENDORSEMENTS_PER_MESSAGE","max endorsements per message"],["MAX_FINAL_EVENTS","normally in `config.toml`, max final events kept"],["MAX_FUTURE_PROCESSING_BLOCK","normally in `config.toml`, max in the future kept blocks"],["MAX_GAS_PER_BLOCK","max gas per block"],["MAX_ITEM_RETURN_COUNT","normally in `config.toml`, max item count returned"],["MAX_MESSAGE_SIZE","max message size 3 * 1024 * 1024"],["MAX_OPERATIONS_PER_BLOCK","max number of operation per block"],["MAX_OPERATIONS_PER_MESSAGE","max number of operation per message"],["MAX_OPERATION_FILL_ATTEMPTS","normally in `config.toml`, max operation fill attempts"],["NODE_SEND_CHANNEL_SIZE","node send channel size"],["OPERATION_BATCH_SIZE","normally in `config.toml`, operation batch size"],["OPERATION_ID_SIZE_BYTES","operation id size"],["OPERATION_VALIDITY_PERIODS","operation validity periods"],["PERIODS_PER_CYCLE","periods per cycle"],["POS_DRAW_CACHED_CYCLE","normally in `config.toml`, proof of stake cached cycle"],["POS_LOCK_CYCLES","proof of stake lock cycles"],["POS_LOOKBACK_CYCLES","proof of stake look back cycle"],["READONLY_QUEUE_LENGTH","normally in `config.toml`, read only queue length"],["ROLL_PRICE","roll price"],["SLOT_KEY_SIZE","serialized slot size"],["T0","period length in milliseconds, sometimes overridden in `config.rs` or `setting.rs`"],["THREAD_COUNT","thread count"]],"struct":[["BLOCK_DB_PRUNE_INTERVAL","blocks are pruned every `BLOCK_DB_PRUNE_INTERVAL` milliseconds"],["END_TIMESTAMP","TESTNET: time when the blockclique is ended."],["GENESIS_KEY","genesis private keys"],["GENESIS_TIMESTAMP","Time in milliseconds when the blockclique started."],["LEDGER_FLUSH_INTERVAL","ledger is saved on disk every `LEDGER_FLUSH_INTERVAL` milliseconds"],["MAX_SEND_WAIT","we wait `MAX_SEND_WAIT` milliseconds to send a message"],["POS_MISS_RATE_DEACTIVATION_THRESHOLD","Be careful: The `GENESIS_TIMESTAMP` shouldn’t be used as it in test because if you start the full test process, the first use is effectively `MassaTime::now().unwrap()` but will be outdated for the latest test. That’s the reason why we choose to reset it each time we get a `ConsensusConfig`."],["STATS_TIMESPAN","stats are considered for `STATS_TIMESPAN` milliseconds"],["VERSION","node version"]]});