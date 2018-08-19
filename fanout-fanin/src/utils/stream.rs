use futures::sync::mpsc::unbounded;
use models::error::RuntimeError;
use tokio::prelude::*;

pub fn connect<T>() -> (
    impl Sink<SinkItem = T, SinkError = RuntimeError> + Clone,
    impl Stream<Item = T, Error = RuntimeError>,
) {
    let (sink, stream) = unbounded::<T>();
    (
        sink.sink_map_err(|_| RuntimeError::Channel),
        stream.map_err(|_| RuntimeError::Channel),
    )
}
