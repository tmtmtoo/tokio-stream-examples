extern crate futures;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_fs;
extern crate tokio_timer;

mod models;
mod streams;
mod utils;

use models::error::RuntimeError;
use std::time::Instant;
use streams::{sink, source, transform};
use tokio::prelude::*;
use tokio::runtime::Runtime;
use utils::future::cleanup;

fn main() -> Result<(), RuntimeError> {
    let mut rt = Runtime::new().map_err(|_| RuntimeError::Executor)?;

    let basis = Instant::now();
    let timer = source::timer::new(basis.clone(), 500);
    let (counter_in, counter_out) = transform::counter::new();
    let (elapsed_time_in, elapsed_time_out) = transform::elapsed_time::new(basis);
    let stdout = sink::stdout::new();

    rt.spawn(cleanup(timer.forward(counter_in.fanout(elapsed_time_in))))
        .spawn(cleanup(
            counter_out.select(elapsed_time_out).forward(stdout),
        ));

    rt.shutdown_on_idle()
        .wait()
        .map(|_| ())
        .map_err(|_| RuntimeError::Executor)
}
