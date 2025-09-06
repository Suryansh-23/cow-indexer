use core::types::ChainNotif;
use std::path::Path;

use crate::config::*;
use crate::exex::*;
use clap::Parser;
use reth::{chainspec::EthereumChainSpecParser, cli::Cli};
use reth_node_ethereum::EthereumNode;
use storage_sqlite::dao::Write;
use storage_sqlite::SqliteStore;
use storage_sqlite::dao::R;

fn main() -> eyre::Result<()> {
    Cli::<EthereumChainSpecParser, IndexerArgs>::parse().run(
        async move |builder, indexer_args: IndexerArgs| {
            let sqlite_store =
                SqliteStore::<Write>::new(Path::new(&indexer_args.db_path).to_path_buf()).unwrap();

            let (tx, rx) = tokio::sync::mpsc::channel::<ChainNotif>(indexer_args.buffer_size);

            

            let handle = builder
                .node(EthereumNode::default())
                .install_exex("cow-indexer", async move |ctx| Ok(ExExNode::new(ctx, tx)))
                .launch()
                .await?;

            handle.wait_for_node_exit().await
        },
    )
}

pub mod config;
pub mod exex;
pub mod shutdown;
