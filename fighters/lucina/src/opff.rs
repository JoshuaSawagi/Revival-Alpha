use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

// symbol-based call for the fe characters' common opff
extern "Rust" {
    fn fe_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}


pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //up_special_reverse(boma, status_kind, stick_x, facing, frame);
}

pub extern "C" fn lucina_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		lucina_frame(fighter);
        fe_common(fighter);
    }
}

pub unsafe fn lucina_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, lucina_frame_wrapper);
}