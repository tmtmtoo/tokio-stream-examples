use models::error::RuntimeError;
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio_timer::Interval;

pub fn new(
    basis: Instant,
    interval_millis: u64,
) -> impl Stream<Item = Instant, Error = RuntimeError> {
    Interval::new(basis, Duration::from_millis(interval_millis))
        .map_err(|e| RuntimeError::Timer(Box::new(e)))
}
