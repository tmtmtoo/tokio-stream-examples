use models::error::RuntimeError;
use std::time::Instant;
use tokio::prelude::*;
use utils::stream::connect;

pub fn new() -> (
    impl Sink<SinkItem = Instant, SinkError = RuntimeError>,
    impl Stream<Item = String, Error = RuntimeError>,
) {
    let (input, output) = connect::<Instant>();

    (
        input,
        output.map({
            let mut count = 0u32;

            move |_| {
                count = count + 1;
                format!("count: {}", count)
            }
        }),
    )
}
