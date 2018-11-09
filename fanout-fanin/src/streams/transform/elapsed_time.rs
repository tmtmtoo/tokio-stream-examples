use models::error::RuntimeError;
use std::time::Instant;
use tokio::prelude::*;
use utils::stream::connect;

pub fn new(
    basis: Instant,
) -> (
    impl Sink<SinkItem = Instant, SinkError = RuntimeError>,
    impl Stream<Item = String, Error = RuntimeError>,
) {
    let (input, output) = connect::<Instant>();

    (
        input,
        output.map(move |t: Instant| format!("elapsed time: {:?}", t.duration_since(basis))),
    )
}