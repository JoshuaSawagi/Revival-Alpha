use utils::{
    *,
    ext::*,
    consts::*
};

use utils::consts::*;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;

unsafe fn jump_cancel_grab(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, fighter_kind: i32) {
    // Link can JC grab, except while a bomb is out
    let link_has_bomb = fighter_kind == *FIGHTER_KIND_LINK && ArticleModule::is_exist(boma, *FIGHTER_LINK_GENERATE_ARTICLE_LINKBOMB);
    if !link_has_bomb {
        if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            if boma.is_cat_flag(Cat1::Catch) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
            }
        }
    }
}

unsafe fn dacus(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        if MotionModule::frame(boma) < 10.0 {
            let is_catch = boma.is_cat_flag(Cat1::Catch) || boma.is_button_trigger(Buttons::Catch);

            // Normal smash input or Z with left stick
            if boma.is_cat_flag(Cat1::AttackHi4) || (stick_y >= 0.7 && is_catch) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }

            if boma.is_cat_flag(Cat1::AttackLw4)|| (stick_y <= -0.7 && is_catch) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }

            // Adjust input window of tilts to prevent accidental smashes
            if MotionModule::frame(boma) > 2.0 {
                if boma.is_cat_flag(Cat1::AttackHi3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
                }
                if boma.is_cat_flag(Cat1::AttackLw3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
                }
            }
        }
    }
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    dacus(boma, cat[0], status_kind, stick_y);
    //jump_cancel_grab(boma, cat[0], status_kind, fighter_kind);
}
