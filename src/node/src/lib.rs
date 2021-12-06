pub mod server;
mod peer;
mod connection;

use std::{error, result};

type ServerError = Box<dyn error::Error>;
type ServerResult<T> = result::Result<T, ServerError>;





