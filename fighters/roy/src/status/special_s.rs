use super::*;

// FIGHTER_ROY_STATUS_KIND_SPECIAL_S4

pub unsafe extern "C" fn special_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    let mot = if VarModule::is_flag(fighter.battle_object, vars::roy::status::SPECIAL_S4_REVERSE) {
        (hash40("special_s4_back"), hash40("special_air_s4_back"))
    }
    else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_LW) {
        (hash40("special_s4_lw"), hash40("special_air_s4_lw"))
    }
    else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_HI) {
        (hash40("special_s4_hi"), hash40("special_air_s4_hi"))
    }
    else {
        (hash40("special_s4_s"), hash40("special_air_s4_s"))
    };
    WorkModule::set_int64(fighter.module_accessor, mot.0, *FIGHTER_ROY_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(fighter.module_accessor, mot.1, *FIGHTER_ROY_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
    VarModule::off_flag(fighter.battle_object, vars::roy::status::SPECIAL_S4_REVERSE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_HI);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_LW);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s4_main_loop as *const () as _))
}

unsafe extern "C" fn special_s4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            special_s_mot_helper(fighter);
        }
    }
    else {
        special_s_mot_helper(fighter);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let sit = fighter.global_table[SITUATION_KIND].get_i32();
        if sit != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn special_s_mot_helper(fighter: &mut L2CFighterCommon) {
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    let correct;
    let mot;
    if sit != *SITUATION_KIND_GROUND {
        correct = *GROUND_CORRECT_KIND_AIR;
        mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
    }
    else {
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
        mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, special_s4_main);
}