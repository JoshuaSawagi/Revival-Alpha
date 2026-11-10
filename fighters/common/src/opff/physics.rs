use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::{Vector2f, Vector3f, Hash40};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;
use smash::app::utility::*;
use vars::*;
use crate::sv_kinetic_energy;
use crate::globals::STICK_X;
use smash::app;
use crate::sv_information;
use smash::lib::{L2CValue, L2CAgent};
use skyline::nro::{self, NroInfo};
use skyline::nn::ro::LookupSymbol;
use smashline::*;
use super::*;
use smash_script::*;

//static mut ECB_Y_OFFSETS: [f32; 9] = [0.0; 9];
pub unsafe fn get_player_number(boma: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
}

//Fixes 2-frame physics delay when wavelanding and wavedashing
unsafe fn waveland_physics_correction(boma: &mut BattleObjectModuleAccessor) {
    if !boma.is_status(*FIGHTER_STATUS_KIND_ESCAPE_AIR)
    || !boma.is_situation(*SITUATION_KIND_AIR) {
        return;
    }
    let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    if speed_y >= 0.0 { return; }

    let pos = Vector2f { x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma) };
    let ray = Vector2f { x: speed_x, y: speed_y };
    let mut hit_pos = Vector2f { x: 0.0, y: 0.0 };

    if GroundModule::ray_check_hit_pos(boma, &pos, &ray, &mut hit_pos, true) {
        PostureModule::set_pos_2d(boma, &hit_pos);
        StatusModule::set_situation_kind(boma, app::SituationKind(*SITUATION_KIND_GROUND), true);
        GroundModule::attach_ground(boma, false);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
    }
}

unsafe fn airdodge_into_special_fall(boma: &mut BattleObjectModuleAccessor) {
    let status = StatusModule::status_kind(boma);
    if status != *FIGHTER_STATUS_KIND_ESCAPE_AIR
        && status != *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE {
        return;
    }
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    // trigger one frame early so vanilla's own "motion ended -> FALL" doesn't win
    if frame >= end_frame - 1.0 {
        StatusModule::change_status_request_from_script(
            boma,
            *FIGHTER_STATUS_KIND_FALL_SPECIAL,
            true,
        );
    }
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32, curr_frame: f32) {
    waveland_physics_correction(boma);
    //airdodge_into_special_fall(boma);
}