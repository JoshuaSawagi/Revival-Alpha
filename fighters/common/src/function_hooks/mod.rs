use super::*;
use crate::globals::*;
use crate::opff::physics::get_player_number;

//pub mod collision;
pub mod controls;
pub mod energy;
pub mod kinetic;
pub mod momentum_transfer;
pub mod transition;
mod lua_bind_hook;
mod game;

pub fn install() {
    controls::install();
    //collision::install();
    energy::install();
    kinetic::install();
    momentum_transfer::install();
    transition::install();
    lua_bind_hook::install();
    game::install();
    
    unsafe {
        // Matches Melee behavior
        skyline::patching::Patch::in_text(0x3e6d08).data(0x14000012u32);
    }
}