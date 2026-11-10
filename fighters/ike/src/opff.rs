use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);


unsafe fn quickdraw_jump_attack_cancels(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32) {
    if status_kind != *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH {
        return;
    }
    
    // Wall Jump & ECB correction
    if situation_kind == *SITUATION_KIND_AIR {
        //GroundModule::set_rhombus_offset(boma, &Vector2f::new(0.0, 0.05));
        if  !VarModule::is_flag(boma.object(), vars::common::instance::SPECIAL_WALL_JUMP) {
            let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
            let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
            if touch_left || touch_right {
                if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) {
                    VarModule::on_flag(boma.object(), vars::common::instance::SPECIAL_WALL_JUMP);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                }
            }
        }
    }

    // Jump and Attack cancels
    let pad_flag = ControlModule::get_pad_flag(boma);
    if boma.is_input_jump() && !VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL) {
        if situation_kind == *SITUATION_KIND_GROUND {
            if facing * stick_x < 0.0 {
                PostureModule::reverse_lr(boma);
            }
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
        }
    } else if compare_mask(pad_flag, *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) || compare_mask(pad_flag, *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK, true);
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    quickdraw_jump_attack_cancels(boma, id, status_kind, situation_kind, cat[0], stick_x, facing);
}

pub extern "C" fn ike_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		ike_frame(fighter)
    }
}

pub unsafe fn ike_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, ike_frame_wrapper);
}