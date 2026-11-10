use super::*;
use globals::*;

#[skyline::hook(offset = 0x45f440)]
unsafe fn ground_module_ecb_point_calc_hook(ground_module: u64, param_1: *mut *mut Hash40, param_2: *mut f32, param_3: *mut f32, param_4: *mut f32, param_5: *mut f32, param_6: u32) {
    let boma = *((ground_module + 0x20) as *mut *mut BattleObjectModuleAccessor);
    if (*boma).is_fighter() { VarModule::on_flag((*boma).object(), vars::common::instance::IS_GETTING_POSITION_FOR_ECB); }
    call_original!(ground_module, param_1, param_2, param_3, param_4, param_5, 1);
    if (*boma).is_fighter() {
        VarModule::off_flag((*boma).object(), vars::common::instance::IS_GETTING_POSITION_FOR_ECB);
        VarModule::set_float((*boma).object(), vars::common::instance::ECB_BOTTOM_Y_OFFSET, *param_3);
        let ecb_center_y_offset = ((*param_5 - *param_3) / 2.0) + *param_3;
        VarModule::set_float((*boma).object(), vars::common::instance::ECB_CENTER_Y_OFFSET, ecb_center_y_offset);
    }
    if !(*boma).is_fighter()
    || VarModule::is_flag((*boma).object(), vars::common::status::DISABLE_ECB_SHIFT)
    || (*boma).is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEMO,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_THROWN])
    || !(*boma).is_situation(*SITUATION_KIND_AIR)
    || WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) < 9 {
        *param_3 = 0.0;
    }

    if (*boma).is_fighter()
    && !StopModule::is_stop(boma) {
        let total_y_speed = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND);
        if (*boma).is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR) || (
            (*boma).is_status_one_of(&[
                *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
            ]) && (*boma).status_frame() > 1 
        ) {
            if total_y_speed > 0.0 {
                GroundModule::set_passable_check(boma, true);
            } else {
                GroundModule::set_passable_check(boma, false);
            }
        }
    }
}

#[skyline::hook(offset = 0x48fc60)]
unsafe fn model_module_joint_global_position_with_offset_hook(model_module: u64, bone: Hash40, param_3: *mut Vector3f, param_4: *mut Vector3f, param_5: bool) {
    let boma = *(model_module as *mut *mut BattleObjectModuleAccessor).add(1);
    if (*boma).is_fighter()
    && VarModule::is_flag((*boma).object(), vars::common::instance::IS_GETTING_POSITION_FOR_ECB)
    && bone == Hash40::new("trans") {
        return;
    }
    call_original!(model_module, bone, param_3, param_4, param_5);
}

#[skyline::hook(offset = 0x53fe50)]
unsafe fn groundcollision__processgroundcollisioninfo_check_landing(groundcollisioninfo: *mut f32, groundcollision: u64) {
    let groundcollisionline = *((groundcollision + 0x320) as *mut u64) as *mut GroundCollisionLine;
    let groundcollisionline_next = *(groundcollisionline as *mut *mut GroundCollisionLine);
    let vertex_1_y = *(((groundcollisionline_next as u64) + 0x24) as *mut f32);
    let vertex_2_y = *(((groundcollisionline_next as u64) + 0x34) as *mut f32);
    let touch_pos_y = (vertex_1_y + vertex_2_y) / 2.0;
    let flags = *(groundcollisioninfo.add(0x5d8 / 4) as *mut u32);
    let is_fighter = flags >> 0x1b & 1 == 0;
    let is_item = flags >> 0xa & 1 == 0;
    let situation_kind = *(groundcollisioninfo.add(0x5a0 / 4) as *mut i32);
    let prev_pos_y = *groundcollisioninfo.add(0x4c4 / 4);
    let pos_y = *groundcollisioninfo.add(0x634 / 4);
    let prev_ecb_offset_y = *groundcollisioninfo.add(0x424 / 4);
    let ecb_offset_y = *groundcollisioninfo.add(0x3d4 / 4);

    if !is_fighter
    && !is_item
    && situation_kind == 2
    && (prev_ecb_offset_y == 0.0 && ecb_offset_y != 0.0)
    && (prev_pos_y + prev_ecb_offset_y) >= (pos_y + ecb_offset_y)
    && (pos_y + ecb_offset_y) <= touch_pos_y {
        *groundcollisioninfo.add(0x420 / 4) = *groundcollisioninfo.add(0x3d0 / 4);
        *groundcollisioninfo.add(0x424 / 4) = *groundcollisioninfo.add(0x3d4 / 4);
        *((groundcollision + 0x39f) as *mut bool) = true;
    } else {
        *((groundcollision + 0x39f) as *mut bool) = false;
    };

    call_original!(groundcollisioninfo, groundcollision);

    let prev_touch_pos_y = *groundcollisioninfo.add(0x1A4 / 4);
    let touch_pos_y = *groundcollisioninfo.add(0xB4 / 4);
    let ecb_offset_y = *groundcollisioninfo.add(0x3d4 / 4);

    if is_fighter
    && prev_touch_pos_y == 0.0
    && touch_pos_y != 0.0
    && ecb_offset_y != 0.0
    && lua_bind::BattleObjectSlow::is_adjust(utils::singletons::BattleObjectSlow()) {
        let object_kind = *(groundcollisioninfo.add(0x3ac / 4) as *mut i32);
        if object_kind == *WEAPON_KIND_PICKEL_TROLLEY { return };
        *groundcollisioninfo.add(0x634 / 4) = touch_pos_y;
        *groundcollisioninfo.add(0x3d4 / 4) = 0.0;
    }
}

#[skyline::hook(offset = 0x52d920)]
unsafe fn groundcollision__processgroundcollisioninfo_check_landing_sub(groundcollision: u64, arg2: *mut u64, prev_ecb_bottom_pos: *mut Vector2f, ecb_bottom_translation: *mut Vector2f, arg5: u64, arg6: u64, arg7: *mut u64) -> *mut GroundCollisionLine {
    if *((groundcollision + 0x39f) as *mut bool) {
        return 0 as *mut GroundCollisionLine;
    }
    call_original!(groundcollision, arg2, prev_ecb_bottom_pos, ecb_bottom_translation, arg5, arg6, arg7)
}

pub fn install() {
    skyline::install_hooks!(
        groundcollision__processgroundcollisioninfo_check_landing,
        groundcollision__processgroundcollisioninfo_check_landing_sub,
        ground_module_ecb_point_calc_hook,
        model_module_joint_global_position_with_offset_hook
    );
}