use models::error::RuntimeError;
use tokio::prelude::*;
use tokio_codec::{FramedWrite, LinesCodec};
use tokio_fs::stdout as tokio_stdout;

pub fn new() -> impl Sink<SinkItem = String, SinkError = RuntimeError> {
    FramedWrite::new(tokio_stdout(), LinesCodec::new())
        .sink_map_err(|e| RuntimeError::Stdio(Box::new(e)))
}
