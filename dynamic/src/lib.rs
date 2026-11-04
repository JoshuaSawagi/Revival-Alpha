#![allow(non_upper_case_globals)]
#![allow(integer_to_ptr_transmutes)]
#![allow(unused)]
#![allow(internal_features)]



pub mod offsets;
pub mod util;
pub mod singletons;
pub mod consts;
pub mod ext;
mod modules;
pub mod frame_info;
pub mod se;
pub mod game_modes;
/*
pub mod knockback_calc_context;
pub mod knockback_func;
pub mod rect;
*/
#[macro_use]
extern crate modular_bitfield;

pub use hdr_macros::{
    export,
    import,
    import_noreturn,
    hash40
};

pub use hdr_macros as macros;

pub use modules::*;
pub use frame_info::*;
