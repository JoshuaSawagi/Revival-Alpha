use super::*;
use globals::*;
use interpolation::Lerp;

pub static VECTOR_ZERO : Vector3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };

macro_rules! interrupt {
    () => { return L2CValue::I32(1); };
    ($fighter:ident, $status:expr, $repeat:expr) => {{ $fighter.change_status($status.into(), $repeat.into()); interrupt!(); }}
}
//mod airdodge;
mod cliff;
pub mod dash;
mod dead;
mod guard_damage;
mod guard_off;
mod guard_on;
mod guard;
pub mod jump;
pub mod jumpsquat;
pub mod run;
pub mod turn;
pub mod walk;
pub mod reverse_hits;

pub fn install() {
    //airdodge::install();
    cliff::install();
    //dash::install();
    dead::install();
    guard_damage::install();
    guard_off::install();
    guard_on::install();
    guard::install();
    jump::install();
    jumpsquat::install();
    //turn::install();
    //run::install();
    //walk::install();
    //reverse_hits::install();
}