//! This is the internal library for the
//! [`when`](https://github.com/mitsuhiko/when) command line utility.
//!
//! Using this crate directly is not recommended as it's not maintained with a stable
//! API interface.  It primarily exists so that it can be compiled to web assembly
//! independently of the CLI tool.
mod location;
mod parser;
mod utils;

pub use self::location::{find_zone, Location, LocationKind, ZoneRef};
pub use self::parser::{InputExpr, TimeAtLocation};
pub use self::utils::{get_time_of_day, TimeOfDay};
