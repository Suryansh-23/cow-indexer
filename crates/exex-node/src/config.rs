use clap::Parser;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
#[derive(Debug, Parser, Clone)]
pub struct IndexerArgs {
    #[arg(long, default_value_t = 12)]
    pub finality_depth: u64,

    #[arg(long, default_value_t = String::from("cow.db"))]
    pub db_path: String,

    #[arg(long, default_value_t = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5555))]
    pub api_bind: SocketAddr,

    #[arg(long, default_value_t = 100)]
    pub buffer_size: usize,
}
