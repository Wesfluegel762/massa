initSidebarItems({"constant":[["PRIVATE_KEY_SIZE_BYTES","Size of a private key"],["PUBLIC_KEY_SIZE_BYTES","Size of a public key"],["SIGNATURE_SIZE_BYTES","size of a signature"]],"fn":[["derive_public_key","Derives a `PublicKey` from a `PrivateKey`."],["generate_random_private_key","Generate a random private key from a random draw."],["sign","Returns the `Signature` produced by signing data bytes with a `PrivateKey`."],["verify_signature","Checks if the Signature associated with data bytes was produced with the `PrivateKey` associated to given `PublicKey`"]],"struct":[["PrivateKey","`PrivateKey` used to sign messages Generated using `SignatureEngine`."],["PublicKey","Public key used to check if a message was encoded by the corresponding `PublicKey`. Generated from the `PrivateKey` using `SignatureEngine`"],["Signature","Signature generated from a message and a `PrivateKey`."]]});