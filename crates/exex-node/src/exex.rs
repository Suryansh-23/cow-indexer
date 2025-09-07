use core::types::ChainNotif;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, ready},
};

use futures_util::{FutureExt, TryStreamExt};
use reth::{
    api::{FullNodeComponents, NodeTypes},
    primitives::EthPrimitives,
};
use reth_exex::{ExExContext, ExExNotification};
use reth_tracing::tracing::info;
use tokio::sync::mpsc;

pub struct ExExNode<Node: FullNodeComponents> {
    ctx: ExExContext<Node>,
    tx: mpsc::Sender<ChainNotif>,
}

impl<Node: FullNodeComponents<Types: NodeTypes<Primitives = EthPrimitives>>> ExExNode<Node> {
    pub fn new(ctx: ExExContext<Node>, tx: mpsc::Sender<ChainNotif>) -> Self {
        info!("ExExNode initialized");
        Self { ctx, tx }
    }
}

impl<Node: FullNodeComponents<Types: NodeTypes<Primitives = EthPrimitives>>> Future
    for ExExNode<Node>
{
    type Output = eyre::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        info!("ExExNode polling for notifications");

        while let Some(notification) = ready!(this.ctx.notifications.try_next().poll_unpin(cx))? {
            info!("Notification Received: {:?}", notification);
            // if let Err(e) = this.tx.try_send(notification.clone()) {
            //     match e {
            //         mpsc::error::TrySendError::Full(_) => {
            //             // If the channel is full, we can just skip this notification
            //             continue;
            //         }
            //         mpsc::error::TrySendError::Closed(_) => {
            //             return Poll::Ready(Err(eyre::eyre!("Channel closed")));
            //         }
            //     }
            // }

            // if let Some(committed_chain) = notification.committed_chain() {
            //     this.ctx
            //         .events
            //         .send(ExExEvent::FinishedHeight(committed_chain.tip().num_hash()))?;
            // }
            match &notification {
                ExExNotification::ChainCommitted { new } => {
                    info!(committed_chain = ?new.range(), "Received commit");
                }
                ExExNotification::ChainReorged { old, new } => {
                    info!(from_chain = ?old.range(), to_chain = ?new.range(), "Received reorg");
                }
                ExExNotification::ChainReverted { old } => {
                    info!(reverted_chain = ?old.range(), "Received revert");
                }
            };
        }

        info!("ExExNode polling complete");
        Poll::Ready(eyre::Ok(()))
    }
}
