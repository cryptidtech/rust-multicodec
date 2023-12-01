build_codec_enum! {
	0x00 => (Identity, "identity"),
	0x01 => (Cidv1, "cidv1"),
	0x02 => (Cidv2, "cidv2"),
	0x03 => (Cidv3, "cidv3"),
	0x04 => (Ip4, "ip4"),
	0x06 => (Tcp, "tcp"),
	0x07 => (Vlad, "vlad"),
	0x08 => (ProvenanceLogEntry, "provenance-log-entry"),
	0x11 => (Sha1, "sha1"),
	0x12 => (Sha2256, "sha2-256"),
	0x13 => (Sha2512, "sha2-512"),
	0x14 => (Sha3512, "sha3-512"),
	0x15 => (Sha3384, "sha3-384"),
	0x16 => (Sha3256, "sha3-256"),
	0x17 => (Sha3224, "sha3-224"),
	0x18 => (Shake128, "shake-128"),
	0x19 => (Shake256, "shake-256"),
	0x1a => (Keccak224, "keccak-224"),
	0x1b => (Keccak256, "keccak-256"),
	0x1c => (Keccak384, "keccak-384"),
	0x1d => (Keccak512, "keccak-512"),
	0x1e => (Blake3, "blake3"),
	0x20 => (Sha2384, "sha2-384"),
	0x21 => (Dccp, "dccp"),
	0x22 => (Murmur3X6464, "murmur3-x64-64"),
	0x23 => (Murmur332, "murmur3-32"),
	0x29 => (Ip6, "ip6"),
	0x2a => (Ip6Zone, "ip6zone"),
	0x2b => (Ipcidr, "ipcidr"),
	0x2f => (Path, "path"),
	0x30 => (Multicodec, "multicodec"),
	0x31 => (Multihash, "multihash"),
	0x32 => (Multiaddr, "multiaddr"),
	0x33 => (Multibase, "multibase"),
	0x34 => (Varsig, "varsig"),
	0x35 => (Dns, "dns"),
	0x36 => (Dns4, "dns4"),
	0x37 => (Dns6, "dns6"),
	0x38 => (Dnsaddr, "dnsaddr"),
	0x39 => (Multisig, "multisig"),
	0x3a => (Multikey, "multikey"),
	0x3b => (Nonce, "nonce"),
	0x50 => (Protobuf, "protobuf"),
	0x51 => (Cbor, "cbor"),
	0x55 => (Raw, "raw"),
	0x56 => (DblSha2256, "dbl-sha2-256"),
	0x60 => (Rlp, "rlp"),
	0x63 => (Bencode, "bencode"),
	0x70 => (DagPb, "dag-pb"),
	0x71 => (DagCbor, "dag-cbor"),
	0x72 => (Libp2PKey, "libp2p-key"),
	0x78 => (GitRaw, "git-raw"),
	0x7b => (TorrentInfo, "torrent-info"),
	0x7c => (TorrentFile, "torrent-file"),
	0x81 => (LeofcoinBlock, "leofcoin-block"),
	0x82 => (LeofcoinTx, "leofcoin-tx"),
	0x83 => (LeofcoinPr, "leofcoin-pr"),
	0x84 => (Sctp, "sctp"),
	0x85 => (DagJose, "dag-jose"),
	0x86 => (DagCose, "dag-cose"),
	0x8c => (Lbry, "lbry"),
	0x90 => (EthBlock, "eth-block"),
	0x91 => (EthBlockList, "eth-block-list"),
	0x92 => (EthTxTrie, "eth-tx-trie"),
	0x93 => (EthTx, "eth-tx"),
	0x94 => (EthTxReceiptTrie, "eth-tx-receipt-trie"),
	0x95 => (EthTxReceipt, "eth-tx-receipt"),
	0x96 => (EthStateTrie, "eth-state-trie"),
	0x97 => (EthAccountSnapshot, "eth-account-snapshot"),
	0x98 => (EthStorageTrie, "eth-storage-trie"),
	0x99 => (EthReceiptLogTrie, "eth-receipt-log-trie"),
	0x9a => (EthReceiptLog, "eth-receipt-log"),
	0xa0 => (Aes128, "aes-128"),
	0xa1 => (Aes192, "aes-192"),
	0xa2 => (Aes256, "aes-256"),
	0xa3 => (Chacha128, "chacha-128"),
	0xa4 => (Chacha256, "chacha-256"),
	0xa5 => (Chacha20Poly1305, "chacha20-poly1305"),
	0xb0 => (BitcoinBlock, "bitcoin-block"),
	0xb1 => (BitcoinTx, "bitcoin-tx"),
	0xb2 => (BitcoinWitnessCommitment, "bitcoin-witness-commitment"),
	0xc0 => (ZcashBlock, "zcash-block"),
	0xc1 => (ZcashTx, "zcash-tx"),
	0xca => (Caip50, "caip-50"),
	0xce => (Streamid, "streamid"),
	0xd0 => (StellarBlock, "stellar-block"),
	0xd1 => (StellarTx, "stellar-tx"),
	0xd4 => (Md4, "md4"),
	0xd5 => (Md5, "md5"),
	0xe0 => (DecredBlock, "decred-block"),
	0xe1 => (DecredTx, "decred-tx"),
	0xe2 => (Ipld, "ipld"),
	0xe3 => (Ipfs, "ipfs"),
	0xe4 => (Swarm, "swarm"),
	0xe5 => (Ipns, "ipns"),
	0xe6 => (Zeronet, "zeronet"),
	0xe7 => (Secp256K1Pub, "secp256k1-pub"),
	0xe8 => (Dnslink, "dnslink"),
	0xea => (Bls12381G1Pub, "bls12_381-g1-pub"),
	0xeb => (Bls12381G2Pub, "bls12_381-g2-pub"),
	0xec => (X25519Pub, "x25519-pub"),
	0xed => (Ed25519Pub, "ed25519-pub"),
	0xee => (Bls12381G1G2Pub, "bls12_381-g1g2-pub"),
	0xef => (Sr25519Pub, "sr25519-pub"),
	0xf0 => (DashBlock, "dash-block"),
	0xf1 => (DashTx, "dash-tx"),
	0xfa => (SwarmManifest, "swarm-manifest"),
	0xfb => (SwarmFeed, "swarm-feed"),
	0xfc => (Beeson, "beeson"),
	0x0111 => (Udp, "udp"),
	0x0113 => (P2PWebrtcStar, "p2p-webrtc-star"),
	0x0114 => (P2PWebrtcDirect, "p2p-webrtc-direct"),
	0x0115 => (P2PStardust, "p2p-stardust"),
	0x0118 => (WebrtcDirect, "webrtc-direct"),
	0x0119 => (Webrtc, "webrtc"),
	0x0122 => (P2PCircuit, "p2p-circuit"),
	0x0129 => (DagJson, "dag-json"),
	0x012d => (Udt, "udt"),
	0x012e => (Utp, "utp"),
	0x0132 => (Crc32, "crc32"),
	0x0164 => (Crc64Ecma, "crc64-ecma"),
	0x0190 => (Unix, "unix"),
	0x0196 => (Thread, "thread"),
	0x01a5 => (P2P, "p2p"),
	0x01bb => (Https, "https"),
	0x01bc => (Onion, "onion"),
	0x01bd => (Onion3, "onion3"),
	0x01be => (Garlic64, "garlic64"),
	0x01bf => (Garlic32, "garlic32"),
	0x01c0 => (Tls, "tls"),
	0x01c1 => (Sni, "sni"),
	0x01c6 => (Noise, "noise"),
	0x01c8 => (Shs, "shs"),
	0x01cc => (Quic, "quic"),
	0x01cd => (QuicV1, "quic-v1"),
	0x01d1 => (Webtransport, "webtransport"),
	0x01d2 => (Certhash, "certhash"),
	0x01dd => (Ws, "ws"),
	0x01de => (Wss, "wss"),
	0x01df => (P2PWebsocketStar, "p2p-websocket-star"),
	0x01e0 => (Http, "http"),
	0x01f0 => (Swhid1Snp, "swhid-1-snp"),
	0x0200 => (Json, "json"),
	0x0201 => (Messagepack, "messagepack"),
	0x0202 => (Car, "car"),
	0x0300 => (IpnsRecord, "ipns-record"),
	0x0301 => (Libp2PPeerRecord, "libp2p-peer-record"),
	0x0302 => (Libp2PRelayRsvp, "libp2p-relay-rsvp"),
	0x0309 => (Memorytransport, "memorytransport"),
	0x0400 => (CarIndexSorted, "car-index-sorted"),
	0x0401 => (CarMultihashIndexSorted, "car-multihash-index-sorted"),
	0x0900 => (TransportBitswap, "transport-bitswap"),
	0x0910 => (TransportGraphsyncFilecoinv1, "transport-graphsync-filecoinv1"),
	0x0920 => (TransportIpfsGatewayHttp, "transport-ipfs-gateway-http"),
	0x0d1d => (Multidid, "multidid"),
	0x1012 => (Sha2256Trunc254Padded, "sha2-256-trunc254-padded"),
	0x1013 => (Sha2224, "sha2-224"),
	0x1014 => (Sha2512224, "sha2-512-224"),
	0x1015 => (Sha2512256, "sha2-512-256"),
	0x1022 => (Murmur3X64128, "murmur3-x64-128"),
	0x1052 => (Ripemd128, "ripemd-128"),
	0x1053 => (Ripemd160, "ripemd-160"),
	0x1054 => (Ripemd256, "ripemd-256"),
	0x1055 => (Ripemd320, "ripemd-320"),
	0x1100 => (X11, "x11"),
	0x1200 => (P256Pub, "p256-pub"),
	0x1201 => (P384Pub, "p384-pub"),
	0x1202 => (P521Pub, "p521-pub"),
	0x1203 => (Ed448Pub, "ed448-pub"),
	0x1204 => (X448Pub, "x448-pub"),
	0x1205 => (RsaPub, "rsa-pub"),
	0x1206 => (Sm2Pub, "sm2-pub"),
	0x1300 => (Ed25519Priv, "ed25519-priv"),
	0x1301 => (Secp256K1Priv, "secp256k1-priv"),
	0x1302 => (X25519Priv, "x25519-priv"),
	0x1303 => (Sr25519Priv, "sr25519-priv"),
	0x1305 => (RsaPriv, "rsa-priv"),
	0x1306 => (P256Priv, "p256-priv"),
	0x1307 => (P384Priv, "p384-priv"),
	0x1308 => (P521Priv, "p521-priv"),
	0x1d01 => (Kangarootwelve, "kangarootwelve"),
	0x2000 => (AesGcm256, "aes-gcm-256"),
	0x3f42 => (Silverpine, "silverpine"),
	0x534d => (Sm3256, "sm3-256"),
	0x7012 => (Sha256A, "sha256a"),
	0xb201 => (Blake2B8, "blake2b-8"),
	0xb202 => (Blake2B16, "blake2b-16"),
	0xb203 => (Blake2B24, "blake2b-24"),
	0xb204 => (Blake2B32, "blake2b-32"),
	0xb205 => (Blake2B40, "blake2b-40"),
	0xb206 => (Blake2B48, "blake2b-48"),
	0xb207 => (Blake2B56, "blake2b-56"),
	0xb208 => (Blake2B64, "blake2b-64"),
	0xb209 => (Blake2B72, "blake2b-72"),
	0xb20a => (Blake2B80, "blake2b-80"),
	0xb20b => (Blake2B88, "blake2b-88"),
	0xb20c => (Blake2B96, "blake2b-96"),
	0xb20d => (Blake2B104, "blake2b-104"),
	0xb20e => (Blake2B112, "blake2b-112"),
	0xb20f => (Blake2B120, "blake2b-120"),
	0xb210 => (Blake2B128, "blake2b-128"),
	0xb211 => (Blake2B136, "blake2b-136"),
	0xb212 => (Blake2B144, "blake2b-144"),
	0xb213 => (Blake2B152, "blake2b-152"),
	0xb214 => (Blake2B160, "blake2b-160"),
	0xb215 => (Blake2B168, "blake2b-168"),
	0xb216 => (Blake2B176, "blake2b-176"),
	0xb217 => (Blake2B184, "blake2b-184"),
	0xb218 => (Blake2B192, "blake2b-192"),
	0xb219 => (Blake2B200, "blake2b-200"),
	0xb21a => (Blake2B208, "blake2b-208"),
	0xb21b => (Blake2B216, "blake2b-216"),
	0xb21c => (Blake2B224, "blake2b-224"),
	0xb21d => (Blake2B232, "blake2b-232"),
	0xb21e => (Blake2B240, "blake2b-240"),
	0xb21f => (Blake2B248, "blake2b-248"),
	0xb220 => (Blake2B256, "blake2b-256"),
	0xb221 => (Blake2B264, "blake2b-264"),
	0xb222 => (Blake2B272, "blake2b-272"),
	0xb223 => (Blake2B280, "blake2b-280"),
	0xb224 => (Blake2B288, "blake2b-288"),
	0xb225 => (Blake2B296, "blake2b-296"),
	0xb226 => (Blake2B304, "blake2b-304"),
	0xb227 => (Blake2B312, "blake2b-312"),
	0xb228 => (Blake2B320, "blake2b-320"),
	0xb229 => (Blake2B328, "blake2b-328"),
	0xb22a => (Blake2B336, "blake2b-336"),
	0xb22b => (Blake2B344, "blake2b-344"),
	0xb22c => (Blake2B352, "blake2b-352"),
	0xb22d => (Blake2B360, "blake2b-360"),
	0xb22e => (Blake2B368, "blake2b-368"),
	0xb22f => (Blake2B376, "blake2b-376"),
	0xb230 => (Blake2B384, "blake2b-384"),
	0xb231 => (Blake2B392, "blake2b-392"),
	0xb232 => (Blake2B400, "blake2b-400"),
	0xb233 => (Blake2B408, "blake2b-408"),
	0xb234 => (Blake2B416, "blake2b-416"),
	0xb235 => (Blake2B424, "blake2b-424"),
	0xb236 => (Blake2B432, "blake2b-432"),
	0xb237 => (Blake2B440, "blake2b-440"),
	0xb238 => (Blake2B448, "blake2b-448"),
	0xb239 => (Blake2B456, "blake2b-456"),
	0xb23a => (Blake2B464, "blake2b-464"),
	0xb23b => (Blake2B472, "blake2b-472"),
	0xb23c => (Blake2B480, "blake2b-480"),
	0xb23d => (Blake2B488, "blake2b-488"),
	0xb23e => (Blake2B496, "blake2b-496"),
	0xb23f => (Blake2B504, "blake2b-504"),
	0xb240 => (Blake2B512, "blake2b-512"),
	0xb241 => (Blake2S8, "blake2s-8"),
	0xb242 => (Blake2S16, "blake2s-16"),
	0xb243 => (Blake2S24, "blake2s-24"),
	0xb244 => (Blake2S32, "blake2s-32"),
	0xb245 => (Blake2S40, "blake2s-40"),
	0xb246 => (Blake2S48, "blake2s-48"),
	0xb247 => (Blake2S56, "blake2s-56"),
	0xb248 => (Blake2S64, "blake2s-64"),
	0xb249 => (Blake2S72, "blake2s-72"),
	0xb24a => (Blake2S80, "blake2s-80"),
	0xb24b => (Blake2S88, "blake2s-88"),
	0xb24c => (Blake2S96, "blake2s-96"),
	0xb24d => (Blake2S104, "blake2s-104"),
	0xb24e => (Blake2S112, "blake2s-112"),
	0xb24f => (Blake2S120, "blake2s-120"),
	0xb250 => (Blake2S128, "blake2s-128"),
	0xb251 => (Blake2S136, "blake2s-136"),
	0xb252 => (Blake2S144, "blake2s-144"),
	0xb253 => (Blake2S152, "blake2s-152"),
	0xb254 => (Blake2S160, "blake2s-160"),
	0xb255 => (Blake2S168, "blake2s-168"),
	0xb256 => (Blake2S176, "blake2s-176"),
	0xb257 => (Blake2S184, "blake2s-184"),
	0xb258 => (Blake2S192, "blake2s-192"),
	0xb259 => (Blake2S200, "blake2s-200"),
	0xb25a => (Blake2S208, "blake2s-208"),
	0xb25b => (Blake2S216, "blake2s-216"),
	0xb25c => (Blake2S224, "blake2s-224"),
	0xb25d => (Blake2S232, "blake2s-232"),
	0xb25e => (Blake2S240, "blake2s-240"),
	0xb25f => (Blake2S248, "blake2s-248"),
	0xb260 => (Blake2S256, "blake2s-256"),
	0xb301 => (Skein2568, "skein256-8"),
	0xb302 => (Skein25616, "skein256-16"),
	0xb303 => (Skein25624, "skein256-24"),
	0xb304 => (Skein25632, "skein256-32"),
	0xb305 => (Skein25640, "skein256-40"),
	0xb306 => (Skein25648, "skein256-48"),
	0xb307 => (Skein25656, "skein256-56"),
	0xb308 => (Skein25664, "skein256-64"),
	0xb309 => (Skein25672, "skein256-72"),
	0xb30a => (Skein25680, "skein256-80"),
	0xb30b => (Skein25688, "skein256-88"),
	0xb30c => (Skein25696, "skein256-96"),
	0xb30d => (Skein256104, "skein256-104"),
	0xb30e => (Skein256112, "skein256-112"),
	0xb30f => (Skein256120, "skein256-120"),
	0xb310 => (Skein256128, "skein256-128"),
	0xb311 => (Skein256136, "skein256-136"),
	0xb312 => (Skein256144, "skein256-144"),
	0xb313 => (Skein256152, "skein256-152"),
	0xb314 => (Skein256160, "skein256-160"),
	0xb315 => (Skein256168, "skein256-168"),
	0xb316 => (Skein256176, "skein256-176"),
	0xb317 => (Skein256184, "skein256-184"),
	0xb318 => (Skein256192, "skein256-192"),
	0xb319 => (Skein256200, "skein256-200"),
	0xb31a => (Skein256208, "skein256-208"),
	0xb31b => (Skein256216, "skein256-216"),
	0xb31c => (Skein256224, "skein256-224"),
	0xb31d => (Skein256232, "skein256-232"),
	0xb31e => (Skein256240, "skein256-240"),
	0xb31f => (Skein256248, "skein256-248"),
	0xb320 => (Skein256256, "skein256-256"),
	0xb321 => (Skein5128, "skein512-8"),
	0xb322 => (Skein51216, "skein512-16"),
	0xb323 => (Skein51224, "skein512-24"),
	0xb324 => (Skein51232, "skein512-32"),
	0xb325 => (Skein51240, "skein512-40"),
	0xb326 => (Skein51248, "skein512-48"),
	0xb327 => (Skein51256, "skein512-56"),
	0xb328 => (Skein51264, "skein512-64"),
	0xb329 => (Skein51272, "skein512-72"),
	0xb32a => (Skein51280, "skein512-80"),
	0xb32b => (Skein51288, "skein512-88"),
	0xb32c => (Skein51296, "skein512-96"),
	0xb32d => (Skein512104, "skein512-104"),
	0xb32e => (Skein512112, "skein512-112"),
	0xb32f => (Skein512120, "skein512-120"),
	0xb330 => (Skein512128, "skein512-128"),
	0xb331 => (Skein512136, "skein512-136"),
	0xb332 => (Skein512144, "skein512-144"),
	0xb333 => (Skein512152, "skein512-152"),
	0xb334 => (Skein512160, "skein512-160"),
	0xb335 => (Skein512168, "skein512-168"),
	0xb336 => (Skein512176, "skein512-176"),
	0xb337 => (Skein512184, "skein512-184"),
	0xb338 => (Skein512192, "skein512-192"),
	0xb339 => (Skein512200, "skein512-200"),
	0xb33a => (Skein512208, "skein512-208"),
	0xb33b => (Skein512216, "skein512-216"),
	0xb33c => (Skein512224, "skein512-224"),
	0xb33d => (Skein512232, "skein512-232"),
	0xb33e => (Skein512240, "skein512-240"),
	0xb33f => (Skein512248, "skein512-248"),
	0xb340 => (Skein512256, "skein512-256"),
	0xb341 => (Skein512264, "skein512-264"),
	0xb342 => (Skein512272, "skein512-272"),
	0xb343 => (Skein512280, "skein512-280"),
	0xb344 => (Skein512288, "skein512-288"),
	0xb345 => (Skein512296, "skein512-296"),
	0xb346 => (Skein512304, "skein512-304"),
	0xb347 => (Skein512312, "skein512-312"),
	0xb348 => (Skein512320, "skein512-320"),
	0xb349 => (Skein512328, "skein512-328"),
	0xb34a => (Skein512336, "skein512-336"),
	0xb34b => (Skein512344, "skein512-344"),
	0xb34c => (Skein512352, "skein512-352"),
	0xb34d => (Skein512360, "skein512-360"),
	0xb34e => (Skein512368, "skein512-368"),
	0xb34f => (Skein512376, "skein512-376"),
	0xb350 => (Skein512384, "skein512-384"),
	0xb351 => (Skein512392, "skein512-392"),
	0xb352 => (Skein512400, "skein512-400"),
	0xb353 => (Skein512408, "skein512-408"),
	0xb354 => (Skein512416, "skein512-416"),
	0xb355 => (Skein512424, "skein512-424"),
	0xb356 => (Skein512432, "skein512-432"),
	0xb357 => (Skein512440, "skein512-440"),
	0xb358 => (Skein512448, "skein512-448"),
	0xb359 => (Skein512456, "skein512-456"),
	0xb35a => (Skein512464, "skein512-464"),
	0xb35b => (Skein512472, "skein512-472"),
	0xb35c => (Skein512480, "skein512-480"),
	0xb35d => (Skein512488, "skein512-488"),
	0xb35e => (Skein512496, "skein512-496"),
	0xb35f => (Skein512504, "skein512-504"),
	0xb360 => (Skein512512, "skein512-512"),
	0xb361 => (Skein10248, "skein1024-8"),
	0xb362 => (Skein102416, "skein1024-16"),
	0xb363 => (Skein102424, "skein1024-24"),
	0xb364 => (Skein102432, "skein1024-32"),
	0xb365 => (Skein102440, "skein1024-40"),
	0xb366 => (Skein102448, "skein1024-48"),
	0xb367 => (Skein102456, "skein1024-56"),
	0xb368 => (Skein102464, "skein1024-64"),
	0xb369 => (Skein102472, "skein1024-72"),
	0xb36a => (Skein102480, "skein1024-80"),
	0xb36b => (Skein102488, "skein1024-88"),
	0xb36c => (Skein102496, "skein1024-96"),
	0xb36d => (Skein1024104, "skein1024-104"),
	0xb36e => (Skein1024112, "skein1024-112"),
	0xb36f => (Skein1024120, "skein1024-120"),
	0xb370 => (Skein1024128, "skein1024-128"),
	0xb371 => (Skein1024136, "skein1024-136"),
	0xb372 => (Skein1024144, "skein1024-144"),
	0xb373 => (Skein1024152, "skein1024-152"),
	0xb374 => (Skein1024160, "skein1024-160"),
	0xb375 => (Skein1024168, "skein1024-168"),
	0xb376 => (Skein1024176, "skein1024-176"),
	0xb377 => (Skein1024184, "skein1024-184"),
	0xb378 => (Skein1024192, "skein1024-192"),
	0xb379 => (Skein1024200, "skein1024-200"),
	0xb37a => (Skein1024208, "skein1024-208"),
	0xb37b => (Skein1024216, "skein1024-216"),
	0xb37c => (Skein1024224, "skein1024-224"),
	0xb37d => (Skein1024232, "skein1024-232"),
	0xb37e => (Skein1024240, "skein1024-240"),
	0xb37f => (Skein1024248, "skein1024-248"),
	0xb380 => (Skein1024256, "skein1024-256"),
	0xb381 => (Skein1024264, "skein1024-264"),
	0xb382 => (Skein1024272, "skein1024-272"),
	0xb383 => (Skein1024280, "skein1024-280"),
	0xb384 => (Skein1024288, "skein1024-288"),
	0xb385 => (Skein1024296, "skein1024-296"),
	0xb386 => (Skein1024304, "skein1024-304"),
	0xb387 => (Skein1024312, "skein1024-312"),
	0xb388 => (Skein1024320, "skein1024-320"),
	0xb389 => (Skein1024328, "skein1024-328"),
	0xb38a => (Skein1024336, "skein1024-336"),
	0xb38b => (Skein1024344, "skein1024-344"),
	0xb38c => (Skein1024352, "skein1024-352"),
	0xb38d => (Skein1024360, "skein1024-360"),
	0xb38e => (Skein1024368, "skein1024-368"),
	0xb38f => (Skein1024376, "skein1024-376"),
	0xb390 => (Skein1024384, "skein1024-384"),
	0xb391 => (Skein1024392, "skein1024-392"),
	0xb392 => (Skein1024400, "skein1024-400"),
	0xb393 => (Skein1024408, "skein1024-408"),
	0xb394 => (Skein1024416, "skein1024-416"),
	0xb395 => (Skein1024424, "skein1024-424"),
	0xb396 => (Skein1024432, "skein1024-432"),
	0xb397 => (Skein1024440, "skein1024-440"),
	0xb398 => (Skein1024448, "skein1024-448"),
	0xb399 => (Skein1024456, "skein1024-456"),
	0xb39a => (Skein1024464, "skein1024-464"),
	0xb39b => (Skein1024472, "skein1024-472"),
	0xb39c => (Skein1024480, "skein1024-480"),
	0xb39d => (Skein1024488, "skein1024-488"),
	0xb39e => (Skein1024496, "skein1024-496"),
	0xb39f => (Skein1024504, "skein1024-504"),
	0xb3a0 => (Skein1024512, "skein1024-512"),
	0xb3a1 => (Skein1024520, "skein1024-520"),
	0xb3a2 => (Skein1024528, "skein1024-528"),
	0xb3a3 => (Skein1024536, "skein1024-536"),
	0xb3a4 => (Skein1024544, "skein1024-544"),
	0xb3a5 => (Skein1024552, "skein1024-552"),
	0xb3a6 => (Skein1024560, "skein1024-560"),
	0xb3a7 => (Skein1024568, "skein1024-568"),
	0xb3a8 => (Skein1024576, "skein1024-576"),
	0xb3a9 => (Skein1024584, "skein1024-584"),
	0xb3aa => (Skein1024592, "skein1024-592"),
	0xb3ab => (Skein1024600, "skein1024-600"),
	0xb3ac => (Skein1024608, "skein1024-608"),
	0xb3ad => (Skein1024616, "skein1024-616"),
	0xb3ae => (Skein1024624, "skein1024-624"),
	0xb3af => (Skein1024632, "skein1024-632"),
	0xb3b0 => (Skein1024640, "skein1024-640"),
	0xb3b1 => (Skein1024648, "skein1024-648"),
	0xb3b2 => (Skein1024656, "skein1024-656"),
	0xb3b3 => (Skein1024664, "skein1024-664"),
	0xb3b4 => (Skein1024672, "skein1024-672"),
	0xb3b5 => (Skein1024680, "skein1024-680"),
	0xb3b6 => (Skein1024688, "skein1024-688"),
	0xb3b7 => (Skein1024696, "skein1024-696"),
	0xb3b8 => (Skein1024704, "skein1024-704"),
	0xb3b9 => (Skein1024712, "skein1024-712"),
	0xb3ba => (Skein1024720, "skein1024-720"),
	0xb3bb => (Skein1024728, "skein1024-728"),
	0xb3bc => (Skein1024736, "skein1024-736"),
	0xb3bd => (Skein1024744, "skein1024-744"),
	0xb3be => (Skein1024752, "skein1024-752"),
	0xb3bf => (Skein1024760, "skein1024-760"),
	0xb3c0 => (Skein1024768, "skein1024-768"),
	0xb3c1 => (Skein1024776, "skein1024-776"),
	0xb3c2 => (Skein1024784, "skein1024-784"),
	0xb3c3 => (Skein1024792, "skein1024-792"),
	0xb3c4 => (Skein1024800, "skein1024-800"),
	0xb3c5 => (Skein1024808, "skein1024-808"),
	0xb3c6 => (Skein1024816, "skein1024-816"),
	0xb3c7 => (Skein1024824, "skein1024-824"),
	0xb3c8 => (Skein1024832, "skein1024-832"),
	0xb3c9 => (Skein1024840, "skein1024-840"),
	0xb3ca => (Skein1024848, "skein1024-848"),
	0xb3cb => (Skein1024856, "skein1024-856"),
	0xb3cc => (Skein1024864, "skein1024-864"),
	0xb3cd => (Skein1024872, "skein1024-872"),
	0xb3ce => (Skein1024880, "skein1024-880"),
	0xb3cf => (Skein1024888, "skein1024-888"),
	0xb3d0 => (Skein1024896, "skein1024-896"),
	0xb3d1 => (Skein1024904, "skein1024-904"),
	0xb3d2 => (Skein1024912, "skein1024-912"),
	0xb3d3 => (Skein1024920, "skein1024-920"),
	0xb3d4 => (Skein1024928, "skein1024-928"),
	0xb3d5 => (Skein1024936, "skein1024-936"),
	0xb3d6 => (Skein1024944, "skein1024-944"),
	0xb3d7 => (Skein1024952, "skein1024-952"),
	0xb3d8 => (Skein1024960, "skein1024-960"),
	0xb3d9 => (Skein1024968, "skein1024-968"),
	0xb3da => (Skein1024976, "skein1024-976"),
	0xb3db => (Skein1024984, "skein1024-984"),
	0xb3dc => (Skein1024992, "skein1024-992"),
	0xb3dd => (Skein10241000, "skein1024-1000"),
	0xb3de => (Skein10241008, "skein1024-1008"),
	0xb3df => (Skein10241016, "skein1024-1016"),
	0xb3e0 => (Skein10241024, "skein1024-1024"),
	0xb3e1 => (Xxh32, "xxh-32"),
	0xb3e2 => (Xxh64, "xxh-64"),
	0xb3e3 => (Xxh364, "xxh3-64"),
	0xb3e4 => (Xxh3128, "xxh3-128"),
	0xb401 => (PoseidonBls12381A2Fc1, "poseidon-bls12_381-a2-fc1"),
	0xb402 => (PoseidonBls12381A2Fc1Sc, "poseidon-bls12_381-a2-fc1-sc"),
	0xb403 => (Urdca2015Canon, "urdca-2015-canon"),
	0xb501 => (Ssz, "ssz"),
	0xb502 => (SszSha2256Bmt, "ssz-sha2-256-bmt"),
	0xb601 => (JsonJcs, "json-jcs"),
	0xcc01 => (Iscc, "iscc"),
	0xce11 => (ZeroxcertImprint256, "zeroxcert-imprint-256"),
	0xd000 => (NonstandardSig, "nonstandard-sig"),
	0xd00d => (BcryptPbkdf, "bcrypt-pbkdf"),
	0xd0e7 => (Es256K, "es256k"),
	0xd0ea => (Bls12381G1Sig, "bls-12381-g1-sig"),
	0xd0eb => (Bls12381G2Sig, "bls-12381-g2-sig"),
	0xd0ed => (Eddsa, "eddsa"),
	0xd191 => (Eip191, "eip-191"),
	0xeb51 => (JwkJcsPub, "jwk_jcs-pub"),
	0xf101 => (FilCommitmentUnsealed, "fil-commitment-unsealed"),
	0xf102 => (FilCommitmentSealed, "fil-commitment-sealed"),
	0x706c61 => (Plaintextv2, "plaintextv2"),
	0x807124 => (HolochainAdrV0, "holochain-adr-v0"),
	0x817124 => (HolochainAdrV1, "holochain-adr-v1"),
	0x947124 => (HolochainKeyV0, "holochain-key-v0"),
	0x957124 => (HolochainKeyV1, "holochain-key-v1"),
	0xa27124 => (HolochainSigV0, "holochain-sig-v0"),
	0xa37124 => (HolochainSigV1, "holochain-sig-v1"),
	0xb19910 => (SkynetNs, "skynet-ns"),
	0xb29910 => (ArweaveNs, "arweave-ns"),
	0xb39910 => (SubspaceNs, "subspace-ns"),
	0xb49910 => (KumandraNs, "kumandra-ns"),
	0xd01200 => (Es256, "es256"),
	0xd01201 => (Es284, "es284"),
	0xd01202 => (Es512, "es512"),
	0xd01205 => (Rs256, "rs256"),
	0xd02000 => (Scion, "scion"),
}
