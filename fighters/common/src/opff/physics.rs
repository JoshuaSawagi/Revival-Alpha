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

//HDR-Enviorment edition
unsafe fn waveland_check(boma: &mut BattleObjectModuleAccessor) {
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

unsafe fn ecb_shifts(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    let status = StatusModule::status_kind(boma);
    let prev_status = StatusModule::prev_status_kind(boma, 0);
    let situation = StatusModule::situation_kind(boma);
    let kind = app::utility::get_kind(&mut *boma);

    // Abort if game isn't active
    if !sv_information::is_ready_go() {
        GroundModule::set_rhombus_offset(boma, &Vector2f{x:0.0, y:0.0});
        return;
    }

    // Vanilla state filter
    let vanilla_ecb = [
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_THROWN,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_BURY,
        *FIGHTER_STATUS_KIND_BURY_WAIT,
    ].contains(&status);

    let previous_states = [
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_THROWN,
    ].contains(&prev_status);

    let fighter_low_offset = [
        *FIGHTER_KIND_KIRBY,
        *FIGHTER_KIND_PIKACHU,
        *FIGHTER_KIND_NESS,
        *FIGHTER_KIND_PURIN,
        *FIGHTER_KIND_GAMEWATCH,
        *FIGHTER_KIND_POPO,
        *FIGHTER_KIND_NANA,
        *FIGHTER_KIND_PICHU,
        *FIGHTER_KIND_METAKNIGHT,
        *FIGHTER_KIND_WARIO,
        *FIGHTER_KIND_PZENIGAME,
        *FIGHTER_KIND_PFUSHIGISOU,
        *FIGHTER_KIND_LUCAS,
        *FIGHTER_KIND_PIKMIN,
        *FIGHTER_KIND_TOONLINK,
        *FIGHTER_KIND_DUCKHUNT,
        *FIGHTER_KIND_MURABITO,
        *FIGHTER_KIND_INKLING,
        *FIGHTER_KIND_SHIZUE
    ];

    let fighter_mid_offset = [        
        *FIGHTER_KIND_MARIO,
        *FIGHTER_KIND_YOSHI,
        *FIGHTER_KIND_LUIGI,
        *FIGHTER_KIND_MARIOD,
        *FIGHTER_KIND_YOUNGLINK,
        *FIGHTER_KIND_PLIZARDON,
        *FIGHTER_KIND_DIDDY,
        *FIGHTER_KIND_DEDEDE,
        *FIGHTER_KIND_ROCKMAN,
        *FIGHTER_KIND_GEKKOUGA,
        *FIGHTER_KIND_PACMAN,
        *FIGHTER_KIND_KOOPAJR,
        *FIGHTER_KIND_PACKUN,
        *FIGHTER_KIND_MIIFIGHTER,
        *FIGHTER_KIND_MIISWORDSMAN,
        *FIGHTER_KIND_MIIGUNNER,
        *FIGHTER_KIND_PACKUN,
        *FIGHTER_KIND_BUDDY,
        *FIGHTER_KIND_PICKEL
    ];

    let fighter_high_offset = [
        *FIGHTER_KIND_FOX,
        *FIGHTER_KIND_FALCO,
        *FIGHTER_KIND_DAISY,
        *FIGHTER_KIND_MEWTWO,
        *FIGHTER_KIND_PIT,
        *FIGHTER_KIND_PITB,
        *FIGHTER_KIND_SONIC,
        *FIGHTER_KIND_LUCARIO,
        *FIGHTER_KIND_ROBOT,
        *FIGHTER_KIND_WOLF,
        *FIGHTER_KIND_LITTLEMAC,
        *FIGHTER_KIND_KROOL,
        *FIGHTER_KIND_GAOGAEN
    ];

    let fighter_max_offset = [
        *FIGHTER_KIND_DONKEY,
        *FIGHTER_KIND_LINK,
        *FIGHTER_KIND_CAPTAIN,
        *FIGHTER_KIND_PEACH,
        *FIGHTER_KIND_KOOPA,
        *FIGHTER_KIND_SHEIK,
        *FIGHTER_KIND_ZELDA,
        *FIGHTER_KIND_MARTH,
        *FIGHTER_KIND_LUCINA,
        *FIGHTER_KIND_GANON,
        *FIGHTER_KIND_ROY,
        *FIGHTER_KIND_CHROM,
        *FIGHTER_KIND_SZEROSUIT,
        *FIGHTER_KIND_SNAKE,
        *FIGHTER_KIND_IKE,
        *FIGHTER_KIND_WIIFIT,
        *FIGHTER_KIND_ROSETTA,
        *FIGHTER_KIND_PALUTENA,
        *FIGHTER_KIND_REFLET,
        *FIGHTER_KIND_SHULK,
        *FIGHTER_KIND_RYU,
        *FIGHTER_KIND_KEN,
        *FIGHTER_KIND_CLOUD,
        *FIGHTER_KIND_KAMUI,
        *FIGHTER_KIND_BAYONETTA,
        *FIGHTER_KIND_RIDLEY,
        *FIGHTER_KIND_SIMON,
        *FIGHTER_KIND_RICHTER,
        *FIGHTER_KIND_JACK,
        *FIGHTER_KIND_BRAVE,
        *FIGHTER_KIND_DOLLY,
        *FIGHTER_KIND_MASTER,
        *FIGHTER_KIND_TANTAN,
        *FIGHTER_KIND_EFLAME,
        *FIGHTER_KIND_ELIGHT,
        *FIGHTER_KIND_DEMON,
        *FIGHTER_KIND_TRAIL
    ];

    let fighter_damn_offset = [
        *FIGHTER_KIND_SAMUS,
        *FIGHTER_KIND_SAMUSD,
        *FIGHTER_KIND_EDGE
    ];

    let offset_y = if fighter_low_offset.contains(&kind) {
        2.0
    } else if fighter_mid_offset.contains(&kind) {
        3.5
    } else if fighter_high_offset.contains(&kind) {
        4.0
    } else if fighter_max_offset.contains(&kind) {
        5.0
    } else if fighter_damn_offset.contains(&kind) {
        6.0
    } else {
        3.0 // fallback
    };

    // Skip states that should use vanilla ECB
    if vanilla_ecb || previous_states {
        return;
    }

    // Reset offset during ENTRY or early PASS (e.g. platform drop or training reset)
    let prev_status = StatusModule::prev_status_kind(boma, 0);
    let motion_frame = MotionModule::frame(boma);

    if status == *FIGHTER_STATUS_KIND_ENTRY
        || (prev_status == *FIGHTER_STATUS_KIND_PASS && motion_frame < 3.0)
    {
        GroundModule::set_offset_y(boma, 0.0);
        GroundModule::set_rhombus_offset(boma, &Vector2f { x: 0.0, y: 0.0 });
        return;
    }

    // Check if fighter just entered the air
    let air_trans = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) < 10;

    // Only apply ECB logic for valid airborne movement states
    let valid_air_states = [
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_FALL_SPECIAL,
    ];

    if situation == *SITUATION_KIND_AIR
        && valid_air_states.contains(&status)
        && !(vanilla_ecb || previous_states)
        && motion_frame > 1.0 // avoid applying offset too early during transition
    {
        GroundModule::set_offset_y(boma, offset_y);

        if air_trans {
            GroundModule::set_rhombus_offset(boma, &Vector2f { x: 0.0, y: 0.0 });
        }
    }
    // Reset offset on ground... I think
    else if situation == *SITUATION_KIND_GROUND {
        GroundModule::set_offset_y(boma, 0.0);

        if vanilla_ecb {
            GroundModule::set_rhombus_offset(boma, &Vector2f { x: 0.0, y: 0.0 });
        }
    }
}

unsafe fn djcs(fighter : &mut L2CFighterCommon, boma: &mut smash::app::BattleObjectModuleAccessor) {
    let lua_state = fighter.lua_state_agent;
    let mut l2c_agent = L2CAgent::new(lua_state);
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = get_kind(boma);
    let kinetic_type = KineticModule::get_kinetic_type(boma);

    if [*FIGHTER_KIND_NESS, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_TRAIL].contains(&fighter_kind){ //DJCs
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if kinetic_type == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND {
                if MotionModule::frame(boma) < (7.0) {
                    if !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)) {
                        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                    }
                }
            }
        }
    }
    if [*FIGHTER_KIND_PEACH, *FIGHTER_KIND_DAISY].contains(&fighter_kind) {
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                if MotionModule::frame(boma) < (7.0) {
                    if !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)) {
                        l2c_agent.clear_lua_stack();
                        l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
                        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
                        smash::app::sv_kinetic_energy::set_speed(lua_state);
                    }
                }
            }
        }
    }
}


/*
unsafe fn waveland_check(boma: &mut BattleObjectModuleAccessor) {
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
        let main_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
 
        VarModule::set_float(boma.object(), vars::common::instance::WAVELAND_SPEED_X, main_speed_x);
        VarModule::on_flag(boma.object(), vars::common::instance::WAVELAND_PENDING);
 
        PostureModule::set_pos_2d(boma, &hit_pos);
        StatusModule::set_situation_kind(boma, app::SituationKind(*SITUATION_KIND_GROUND), true);
        GroundModule::attach_ground(boma, false);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
    }
}

unsafe fn waveland_apply_momentum(boma: &mut BattleObjectModuleAccessor) {
    if !VarModule::is_flag(boma.object(), vars::common::instance::WAVELAND_PENDING) {
        return;
    }
 
    if boma.is_status(*FIGHTER_STATUS_KIND_LANDING) {
        let speed_x = VarModule::get_float(boma.object(), vars::common::instance::WAVELAND_SPEED_X);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        let stop_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
        app::lua_bind::KineticEnergy::reset_energy(stop_energy, *ENERGY_STOP_RESET_TYPE_GROUND, &Vector2f { x: speed_x, y: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, boma);
        app::lua_bind::KineticEnergy::enable(stop_energy);
        VarModule::off_flag(boma.object(), vars::common::instance::WAVELAND_PENDING);
    }

    else if !boma.is_status(*FIGHTER_STATUS_KIND_ESCAPE_AIR) {
        VarModule::off_flag(boma.object(), vars::common::instance::WAVELAND_PENDING);
    }

}
*/

pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32, curr_frame: f32) {
    ecb_shifts(boma, status_kind, situation_kind, fighter_kind);
    djcs(fighter, boma);
    waveland_check(boma);
    //waveland_apply_momentum(boma);
}