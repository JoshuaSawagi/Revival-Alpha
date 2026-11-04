use super::*;
use globals::*;

//=================================================================
//== StatusModule::init_settings
//=================================================================
#[skyline::hook(replace=StatusModule::init_settings)]
unsafe fn init_settings_hook(boma: &mut BattleObjectModuleAccessor, mut situation: smash::app::SituationKind, kinetic_type: i32, ground_correct_kind: u32,
                             ground_cliff_check_kind: smash::app::GroundCliffCheckKind, jostle: bool,
                             keep_flag: i32, keep_int: i32, keep_float: i32, arg10: i32) -> u64 {
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_kind = boma.kind();
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let mut cliff_check_kind = ground_cliff_check_kind;
    let mut kinetic_type = kinetic_type.clone();
    let mut ground_correct_kind = ground_correct_kind.clone();

    // Call edge_slippoffs init_settings
    ground_correct_kind = super::ground::init_settings_edges(boma, situation, kinetic_type, ground_correct_kind, ground_cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10);

    if boma.is_fighter() {
        /*
        if boma.is_prev_situation(*SITUATION_KIND_AIR)
        && ( situation.0 == *SITUATION_KIND_GROUND
            || (boma.is_situation(*SITUATION_KIND_GROUND) && situation.0 == *SITUATION_KIND_NONE) )
        {
            if kinetic_type == *FIGHTER_KINETIC_TYPE_MOTION {
                kinetic_type = *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL;
            }
        }
        // Resets your airtime counter when leaving the below statuses
        // Prevents ECB from shifting on f1 after an "ignored" status (those defined below)
        if boma.is_prev_status_one_of(&[
            *FIGHTER_STATUS_KIND_DEMO,
            *FIGHTER_STATUS_KIND_ENTRY,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_THROWN,
            *FIGHTER_STATUS_KIND_CATCHED_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_REFLET,
            *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
            *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE,
            *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD])
        && situation.0 == *SITUATION_KIND_AIR
        {
            WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR);
        }
        */
        
        // Walk through other fighters
        JostleModule::set_team(boma, 0);

        // Occupy ledge on ledgegrab
        if boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_CLIFF_CATCH,
            *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
            *FIGHTER_STATUS_KIND_CLIFF_WAIT]
        ) {
            let cliff_id = GroundModule::get_cliff_id_uint32(boma);
            VarModule::set_int(boma.object(), vars::common::instance::OCCUPIED_LEDGE_ID, cliff_id as i32);
            VarModule::set_int(boma.object(), vars::common::instance::OCCUPIED_LEDGE_ID_FOR_TETHERS, cliff_id as i32);
        }

    }

    if VarModule::has_var_module(boma.object()) {
        let mut mask = 0;
        if keep_flag != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG {
            mask += VarModule::RESET_STATUS_FLAG;
        }
        if keep_int != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT {
            mask += VarModule::RESET_STATUS_INT;
            mask += VarModule::RESET_STATUS_INT64;
        }
        if keep_float != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT {
            mask += VarModule::RESET_STATUS_FLOAT;
        }
        VarModule::reset(boma.object(), mask);
    }
    original!()(boma, situation, kinetic_type, ground_correct_kind, cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10)
}

#[skyline::hook(replace=StatusModule::change_status_request_from_script)]
unsafe fn change_status_request_from_script_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;
    let mut clear_buffer = arg3;
    
    if boma.is_fighter() {  
        if [
            *FIGHTER_STATUS_KIND_ESCAPE,
            *FIGHTER_STATUS_KIND_ESCAPE_F,
            *FIGHTER_STATUS_KIND_ESCAPE_B,
        ].contains(&next_status) {
            clear_buffer = true;
        }
    }
    original!()(boma, next_status, clear_buffer)
}

pub fn install() {
    skyline::install_hooks!(
        init_settings_hook,
        change_status_request_from_script_hook
    );
}