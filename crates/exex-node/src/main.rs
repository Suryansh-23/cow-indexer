use core::types::ChainNotif;
use std::path::Path;

use crate::config::*;
use crate::exex::*;
use crate::shutdown::shutdown_token;
use clap::Parser;
use reth::chainspec::EthereumChainSpecParser;
use reth::cli::Cli;
use reth_node_ethereum::EthereumNode;
use reth_tracing::tracing::{debug, info};
use storage_sqlite::SqliteStore;
use storage_sqlite::dao::R;
use storage_sqlite::dao::Write;
use tokio::pin;

fn main() -> eyre::Result<()> {
    Cli::<EthereumChainSpecParser, IndexerArgs>::parse().run(async move |builder, indexer_args| {
        let _sqlite_store =
            SqliteStore::<Write>::new(Path::new(&indexer_args.db_path).to_path_buf()).unwrap();
        let (tx, rx) = tokio::sync::mpsc::channel::<ChainNotif>(indexer_args.buffer_size);
        let shutdown_token = shutdown_token();

        // Launch the node and get a handle we can await/select on.
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("cow-indexer", async move |ctx| Ok(ExExNode::new(ctx, tx)))
            .launch_with_debug_capabilities()
            .await?;

        // Prepare the node exit future for select! alongside our shutdown token and rx loop.
        let wait_for_exit = handle.wait_for_node_exit();
        pin!(wait_for_exit);

        // We may want to process notifications coming from the ExEx.
        let mut rx = rx;
        let shutdown = shutdown_token.clone();

        loop {
            tokio::select! {
                // Node finished or encountered an error: propagate result.
                _ = &mut wait_for_exit => {
                    break ();
                }
                // External shutdown requested: return Ok(()) so the CliRunner triggers
                // graceful shutdown across all spawned tasks.
                _ = shutdown.cancelled() => {
                    break ();
                }
                // Consume ExEx notifications as needed. Replace with real handling.
                maybe_notif = rx.recv() => {
                    match maybe_notif {
                        Some(_notif) => {
                            info!("Received notification: {:?}", _notif);
                        }
                        None => {
                            // Sender dropped; nothing more to receive.
                            // Continue to wait for node exit or shutdown.
                        }
                    }
                }
            }
        }

        info!("Shutting down COW indexer");
        eyre::Ok(())
    })
}

pub mod config;
pub mod exex;
pub mod shutdown;
