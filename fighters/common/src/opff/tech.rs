use smash::app::lua_bind::*;
use smash::app::BattleObjectModuleAccessor;
use smash::app::{self, lua_bind::*, sv_animcmd, sv_kinetic_energy};
use smash::cpp::root::app::SituationKind;
use smash::hash40;
use smash::lib::{lua_const::*, L2CAgent, L2CValue};
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Hash40;
use smash::phx::{Vector2f, Vector3f, Vector4f};
use utils::{consts::globals::*, consts::*, ext::*, *};
use vars::*;
use super::*;
use globals::*;

// plat drop if you input down during a waveland (airdodge landing lag)
unsafe fn waveland_plat_drop(boma: &mut BattleObjectModuleAccessor, cat2: i32, status_kind: i32) {
    let pass_thresh = boma.get_param_float("common", "pass_stick_y");
    if boma.is_status(*FIGHTER_STATUS_KIND_LANDING)
    && GroundModule::is_passable_ground(boma)
    && ControlModule::get_flick_y(boma) < 2
    && boma.left_stick_y() < pass_thresh
    && boma.is_prev_status_one_of(&[
        *FIGHTER_STATUS_KIND_ESCAPE_AIR,
        *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE
    ]) {
        boma.change_status_req(*FIGHTER_STATUS_KIND_PASS, true);
        return;
    }
}

//=================================================================
//== DASH DROP
//=================================================================
unsafe extern "C" fn dash_drop(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let stick_y = ControlModule::get_stick_y(boma);
        if [*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE,
            * FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
            if GroundModule::is_passable_ground(boma) && stick_y < -0.66 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, false);
            }
        }
    }
}

//=================================================================
//== CROUCH DURING RUN
//=================================================================
unsafe extern "C" fn run_squat(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let stick_y = ControlModule::get_stick_y(boma);
        //let crouch_thresh: f32 = WorkModule::get_param_float(boma, hash40("common"), hash40("pass_stick_y"));
        if status_kind == *FIGHTER_STATUS_KIND_RUN || status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE {
            if stick_y < -0.66 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
            }
        }
    }
}

unsafe fn double_shield_button_airdodge(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    // airdodge with second shield button while holding another shield button
    if boma.is_situation(*SITUATION_KIND_AIR)
    && boma.is_button_trigger(Buttons::GuardHold)
    && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR)
    && boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_PASS,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP3
    ])
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        return;
    }
}

// Teeter cancelling
pub unsafe fn teeter_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if (boma.is_situation(*SITUATION_KIND_GROUND)
        && boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_STATUS_KIND_RUN_BRAKE,
            *FIGHTER_STATUS_KIND_APPEAL,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
            *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
        ])
        && ((KineticModule::get_sum_speed_x(
            fighter.module_accessor,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL,
        ) - KineticModule::get_sum_speed_x(
            fighter.module_accessor,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND,
        ) - KineticModule::get_sum_speed_x(
            fighter.module_accessor,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN,
        )) * PostureModule::lr(boma))
            > 0.0)
    {
        // Conditions for transitioning to teeter animation in sub_ground_check_ottotto
        if (GroundModule::is_ottotto(boma, 1.72) // Original value: 0.86
        && fighter.left_stick_x().abs() < 0.75)
        {
            fighter.change_status(FIGHTER_STATUS_KIND_OTTOTTO.into(), true.into());
        }
    }
}
/*
const METEOR_CANCEL_MIN_FRAME: i32 = 0;

pub unsafe fn meteor_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    unsafe {
        let boma = fighter.module_accessor;
        if StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR {
            return;
        }
        if StopModule::is_stop(boma) {
            return;
        }
        if boma.status_frame() < 0 {
            return;
        }
        let canceled = fighter.sub_transition_group_check_air_special().get_bool() || fighter.sub_transition_group_check_air_jump_aerial().get_bool();

        if canceled {
            KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        }
    }
}
*/

unsafe fn extra_traction(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
    let max_walk = WorkModule::get_param_float(boma, hash40("walk_speed_max"), 0);
    let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);
    let added_traction: smash::phx::Vector3f = smash::phx::Vector3f {x: -0.5 * PostureModule::lr(boma) * ground_brake * speed_x.signum(), y: 0.0, z: 0.0};
    let double_traction_statuses = [
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_JUMP_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_S4_START,
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_LW4
    ];

    if boma.is_status_one_of(&double_traction_statuses) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let motion_accel = smash::app::sv_kinetic_energy::get_accel(fighter.lua_state_agent);

        // reset flag at beginning of any status
        if fighter.global_table[CURRENT_FRAME].get_i32() == 0 {
            VarModule::off_flag(boma.object(), vars::common::instance::IS_MOTION_BASED_ATTACK);
        }
        // if we detect that the current animation is trans-motion-based (shifts your character's position), disable traction for the entire attack 
        if motion_accel.x != 0.0 && !VarModule::is_flag(boma.object(), vars::common::instance::IS_MOTION_BASED_ATTACK) {
            VarModule::on_flag(boma.object(), vars::common::instance::IS_MOTION_BASED_ATTACK);
        }
        if speed_x.abs() > max_walk
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
        && !VarModule::is_flag(boma.object(), vars::common::instance::IS_MOTION_BASED_ATTACK) {
            if boma.is_prev_status_one_of(&double_traction_statuses) {
                KineticModule::add_speed(boma, &added_traction);
            }
            else if fighter.global_table[CURRENT_FRAME].get_i32() > 0 {
                KineticModule::add_speed(boma, &added_traction);
            }
        }
    }
}


pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32, curr_frame: f32) {   
    dash_drop(fighter);
    run_squat(fighter); // Must be done after dash_drop()
    double_shield_button_airdodge(boma, status_kind, situation_kind, cat[0]);
    waveland_plat_drop(boma, cat[1], status_kind);
    teeter_cancel(fighter, boma);
    extra_traction(fighter, boma);
    //meteor_cancel(fighter, boma);
}
