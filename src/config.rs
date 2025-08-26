pub const RPC_URL: &str = "wss://pas-rpc.stakeworld.io";
pub type SubXtResult<T> = Result<T, Box<dyn std::error::Error>>;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "./artifacts/paseo.scale")]
pub mod paseo {}