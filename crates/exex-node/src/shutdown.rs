use tokio_util::sync::CancellationToken;

pub type ShutdownToken = CancellationToken;

pub fn shutdown_token() -> ShutdownToken {
    ShutdownToken::new()
}
