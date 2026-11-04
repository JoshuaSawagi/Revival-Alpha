use interpolation::Lerp;
use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
        status_pre_EscapeAir,
        status_EscapeAir,
        //status_end_EscapeAir,
        /*sub_escape_air_common,
        sub_escape_air_uniq,
        sub_escape_air_common_main,
        sub_escape_air_common_strans_main*/
        setup_escape_air_slide_common,
        exec_escape_air_slide
        );
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_EscapeAir)]
unsafe fn status_pre_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_EscapeAir)]
unsafe fn status_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_escape_air_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_slide"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air"), 0.0, 1.0, false, 0.0, false, false);
    }
    let mut motion_rate = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_FLOAT_MOTION_RATE_PENALTY);
    let start_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
    if 0 < start_frame {
        let intan_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_XLU_FRAME);
        let add_xlu_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
        motion_rate = 1.0 / ((intan_frame as f32) / ((intan_frame - add_xlu_frame) as f32));
    }
    MotionModule::set_rate(fighter.module_accessor, motion_rate);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_EscapeAir_Main as *const () as _))
}

unsafe extern "C" fn status_EscapeAir_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_setup_escape_air_slide_common)]
unsafe fn setup_escape_air_slide_common(fighter: &mut L2CFighterCommon, stick_x: L2CValue, stick_y: L2CValue) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        return;
    }
    //StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), true);
    let stick_vec = sv_math::vec2_normalize(stick_x.get_f32(), stick_y.get_f32());
    WorkModule::set_float(fighter.module_accessor, stick_vec.x, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
    WorkModule::set_float(fighter.module_accessor, stick_vec.y, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
    let escape_air_slide_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_speed"));
    let escape_air_slide_speed_vec = Vector2f{x: escape_air_slide_speed * stick_vec.x, y: escape_air_slide_speed * stick_vec.y};
    
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_FREE, escape_air_slide_speed_vec.x, escape_air_slide_speed_vec.y, 0.0, 0.0, 0.0);
    app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    app::sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -1.0, -1.0);
    app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    let mut escape_air_slide_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_stiff_frame"));
    let escape_air_slide_u_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_u_stiff_frame"));
    let escape_air_slide_d_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_d_stiff_frame"));
    let escape_air_angle = (stick_vec.y/stick_vec.x.abs()).atan().to_degrees();
    let mut fuck = 0.0;
    if escape_air_angle < 0.0 {
        fuck = (escape_air_angle * -1.0) / 90.0;
        escape_air_slide_stiff_frame = Lerp::lerp(&fuck, &escape_air_slide_d_stiff_frame, &escape_air_slide_stiff_frame);
    }
    else {
        fuck = escape_air_angle / 90.0;
        escape_air_slide_stiff_frame = Lerp::lerp(&fuck, &escape_air_slide_u_stiff_frame, &escape_air_slide_stiff_frame);
    }
    let escape_air_slide_stiff_start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_stiff_start_frame"));
    WorkModule::set_float(fighter.module_accessor, escape_air_slide_stiff_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
    let escape_air_slide_back_end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_back_end_frame"));
    let escape_air_add_xlu_start_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
    WorkModule::set_int(fighter.module_accessor, escape_air_slide_back_end_frame + escape_air_add_xlu_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_BACK_END_FRAME);
    WorkModule::set_float(fighter.module_accessor, escape_air_slide_stiff_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
    
    // EffectModule::req_on_joint(
    //     fighter.module_accessor,
    //     Hash40::new("sys_smash_flash_s"),
    //     Hash40::new("hip"),
    //     &Vector3f{x: 0.0, y: 4.0, z: 8.0},
    //     &Vector3f::zero(),
    //     1.1,
    //     &Vector3f{x: 18.0, y: 12.0, z: 0.0},
    //     &Vector3f::zero(),
    //     false,
    //     0,
    //     0,
    //     0
    // );
}

#[skyline::hook(replace = L2CFighterCommon_sub_escape_air_common_main)]
unsafe extern "C" fn sub_escape_air_common_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let id = VarModule::get_int(fighter.battle_object, vars::common::instance::COSTUME_SLOT_NUMBER) as usize;
    let curr_frame = fighter.global_table[CURRENT_FRAME].get_i32();
    let cancel_frame = (VarModule::get_int(fighter.battle_object, vars::common::status::ESCAPE_AIR_CANCEL_FRAME) - 1) as f32;  // subtract 1 because curr_frame is 0 indexed

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return L2CValue::Bool(true);
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
        || (!fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()){
        if fighter.sub_escape_air_common_strans_main().get_bool() {
            return L2CValue::Bool(true);
        }
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
            let status = if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_KOOPAJR
            && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT) {
                *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_LANDING
            } else {
                *FIGHTER_STATUS_KIND_LANDING
            };
            fighter.change_status(
                L2CValue::I32(status),
                L2CValue::Bool(false)
            );
            return L2CValue::Bool(true);
        }
        if MotionModule::end_frame(fighter.module_accessor) < cancel_frame {
            // If our airdodge animation is shorter than its FAF
            if curr_frame < (cancel_frame as i32) {
                // Stop the animation on its second-to-last frame
                // Continue looping the status until our FAF is reached
                if MotionModule::prev_frame(fighter.module_accessor) < (MotionModule::end_frame(fighter.module_accessor) - 1.0)
                && MotionModule::frame(fighter.module_accessor) >= (MotionModule::end_frame(fighter.module_accessor) - 1.0)
                {
                    MotionModule::set_rate(fighter.module_accessor, 0.0);
                }
                return L2CValue::Bool(false);
            } else {
                // Transition to fall on FAF
                let status = if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_KOOPAJR
                && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT) {
                    *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_FALL
                } else {
                    *FIGHTER_STATUS_KIND_FALL
                };
                fighter.change_status(
                    L2CValue::I32(status),
                    L2CValue::Bool(false)
                );
            }
        }
        else {
            // Vanilla logic
            if !MotionModule::is_end(fighter.module_accessor) {
                return L2CValue::Bool(false);
            } else {
                let status = if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_KOOPAJR
                && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT) {
                    *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_FALL
                } else {
                    *FIGHTER_STATUS_KIND_FALL
                };
                fighter.change_status(
                    L2CValue::I32(status),
                    L2CValue::Bool(false)
                );
            }
        }
    }
    L2CValue::Bool(true)
}


#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_exec_escape_air_slide)]
unsafe fn exec_escape_air_slide(fighter: &mut L2CFighterCommon) {
    let mut step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
    let back_end_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_BACK_END_FRAME);
    if step == 0 {
        let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        if frame >= back_end_frame {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL) {
        return;
    }
    step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
    if step != 0 {
        if step != 1 {
            return;
        }
        if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
        }
        let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        if frame >= 1 {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_KEEP_AIR_TURNED_OFF) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_KEEP_AIR_TURNED_OFF);
            }
            //let speed_mul = ParamModule::get_float(fighter.battle_object, ParamType::Common, "escape_air_slide_speed_mul");
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.9 /*speed_mul*/, 0.9 /*speed_mul*/);
            app::sv_kinetic_energy::mul_speed(fighter.lua_state_agent);
        }
        //let fall_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "escape_air_slide_fall_frame");
        if frame >= 28 /*fall_frame*/ {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);

            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST_NO_CAP, speed_x, 0.0, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            app::sv_kinetic_energy::enable(fighter.lua_state_agent);

            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            app::sv_kinetic_energy::enable(fighter.lua_state_agent);

            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
            WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
            if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_KOOPAJR
            && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}