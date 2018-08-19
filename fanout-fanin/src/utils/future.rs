use models::error::RuntimeError;
use tokio::prelude::*;

pub fn cleanup<T>(
    f: impl Future<Item = T, Error = RuntimeError>,
) -> impl Future<Item = (), Error = ()> {
    f.map(|_| ())
        .map_err(|err| println!("Terminated with error: {:?}", err))
}
