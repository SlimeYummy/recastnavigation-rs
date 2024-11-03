#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]

mod error;
pub use error::*;

pub mod demo;
pub mod detour;
pub mod detour_crowd;
pub mod detour_tile_cache;
pub mod recast;
