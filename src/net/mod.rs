#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod net_client;
pub mod net_dedicated;
pub mod net_defs;
pub mod net_gui;
pub mod net_io;
pub mod net_loop;
pub mod net_packet;
pub mod net_query;
pub mod net_sdl;
pub mod net_server;

pub use net_client::*;
pub use net_dedicated::*;
pub use net_defs::*;
pub use net_gui::*;
pub use net_io::*;
pub use net_loop::*;
pub use net_packet::*;
pub use net_query::*;
pub use net_sdl::*;
pub use net_server::*;

use crate::*;

/////////////////////////////
// NET_* Networking code
/////////////////////////////

pub struct net {}

impl net {
    pub fn new() -> Self {
        Self {}
    }
}
