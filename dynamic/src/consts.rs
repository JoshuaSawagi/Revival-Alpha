use super::*;
use smash::lib::LuaConst;

pub mod globals {
    // 0x1
    pub const FIGHTER_KIND: i32 = 0x2;
    pub const OBJECT_ID: i32 = 0x3;
    pub const FIGHTER: i32 = 0x4;
    pub const MODULE_ACCESSOR: i32 = 0x5;
    // 0x6
    pub const INIT_STATUS_FUNC: i32 = 0x7;
    pub const IS_STOPPING: i32 = 0x8;
    pub const STATUS_KIND_INTERRUPT: i32 = 0x9;
    pub const PREV_STATUS_KIND: i32 = 0xA;
    pub const STATUS_KIND: i32 = 0xB;
    pub const STATUS_COUNT: i32 = 0xC;
    // 0xD
    pub const CURRENT_FRAME: i32 = 0xE;
    pub const CURRENT_FRAME2: i32 = 0xF;
    // 0x10
    // 0x11 func ptr
    // 0x12
    pub const SUB_STATUS3: i32 = 0x13;
    pub const SUB_STATUS2: i32 = 0x14;
    pub const SUB_STATUS: i32 = 0x15;
    pub const SITUATION_KIND: i32 = 0x16;
    pub const PREV_SITUATION_KIND: i32 = 0x17;
    pub const PREV_STATUS_FRAME: i32 = 0x18;
    // 0x19
    pub const STICK_X: i32 = 0x1A;
    pub const STICK_Y: i32 = 0x1B;
    pub const FLICK_X: i32 = 0x1C;
    pub const FLICK_Y: i32 = 0x1D;
    pub const FLICK_Y_DIR: i32 = 0x1E;
    pub const PAD_FLAG: i32 = 0x1F;
    pub const CMD_CAT1: i32 = 0x20;
    pub const CMD_CAT2: i32 = 0x21;
    pub const CMD_CAT3: i32 = 0x22;
    pub const CMD_CAT4: i32 = 0x23;
    pub const GUARD_CONT_UNIQ: i32 = 0x34;
    // 0x24
    // 0x25
    // 0x26
    // 0x27
    // 0x28 some substatus
    pub const DASH_CALLBACK: i32 = 0x29;
    // 0x2A
    pub const CUSTOM_ROUTINE: i32 = 0x2B;
    // 0x2C
    // 0x2D
    // 0x2E
    // 0x2F
    // 0x30
    // 0x31
    // 0x32 some substatus
    pub const USE_SPECIAL_N_CALLBACK: i32 = 0x38;
    pub const USE_SPECIAL_S_CALLBACK: i32 = 0x39;
    pub const USE_SPECIAL_HI_CALLBACK: i32 = 0x3A;
    pub const USE_SPECIAL_LW_CALLBACK: i32 = 0x3B;
    pub const CHECK_SPECIAL_COMMAND: i32 = 0x3C;
    pub const WAZA_CUSTOMIZE_CONTROL: i32 = 0x3D;
    pub const STATUS_CHANGE_CALLBACK: i32 = 0x3E;
    pub const DAMAGE_MOTION_KIND_CALLBACK: i32 = 0x42;
    pub const DASH_POST_TRANSITION_CALLBACK: i32 = 0x57;
}

pub mod vars {
    pub mod common {
        pub mod instance {
            // flags
            pub const CSTICK_OVERRIDE: i32 = 0x0001;
            pub const CSTICK_OVERRIDE_SECOND: i32 = 0x0002;
            pub const ENABLE_AIR_ESCAPE_MAGNET: i32 = 0x003;
            pub const SPECIAL_WALL_JUMP: i32 = 0x0004;
            pub const IS_DACUS: i32 = 0x0005;
            pub const PERFECT_WAVEDASH: i32 = 0x0006;
            pub const IS_LATE_PIVOT: i32 = 0x0007;
            pub const CAN_PERFECT_PIVOT: i32 = 0x0008;
            pub const IS_SMASH_TURN: i32 = 0x0009;
            pub const ENABLE_AIR_ESCAPE_JUMPSQUAT: i32 = 0x000A;
            pub const IS_KNOCKDOWN_THROW: i32 = 0x000B;
            pub const IS_CC_NON_TUMBLE: i32 = 0x000C;
            pub const IS_GETTING_POSITION_FOR_ECB: i32 = 0x000D;
            pub const CHECK_CHANGE_MOTION_ONLY: i32 = 0x000E;
            pub const IS_INIT: i32 = 0x000F;
            pub const WEIRD_ASS_TURN_RUN_ANIMATION: i32 = 0x0010;
            pub const WAS_PREV_STATUS_CANCELABLE: i32 = 0x0011;
            pub const IS_ENTER_DASH_CANCEL: i32 = 0x0012;
            pub const IS_DITCIT: i32 = 0x0013;
            pub const NO_GROUND_BOUNCE: i32 = 0x0014;
            
            pub const OCCUPIED_LEDGE_ID: i32 = 0x0017;
            pub const OCCUPIED_LEDGE_ID_FOR_TETHERS: i32 = 0x0016;
            pub const TEMPORARY_CLIFF_STOP: i32 = 0x0061;
            pub const TETHER_HOGGED: i32 = 0x0018;
            pub const IS_STICKY_WALK: i32 = 0x102;

            // ints
            pub const COSTUME_SLOT_NUMBER: i32 = 0x0001;
            pub const CLIFF_XLU_FRAME: i32 = 0x0002;
            pub const LEFT_STICK_FLICK_X: i32 = 0x0003;
            pub const LEFT_STICK_FLICK_Y: i32 = 0x0004;
            pub const RIGHT_STICK_FLICK_X: i32 = 0x0005;
            pub const RIGHT_STICK_FLICK_Y: i32 = 0x0006;
            pub const PREV_STATUS_TRANSITION_FRAME: i32 = 0x0007;
            pub const ATTACK_LR_CHECK: i32 = 0x0008;
            pub const SPECIAL_STALL: i32 = 0x0009;
            pub const SPECIAL_STALL_USED: i32 = 0x0015;

            // floats
            pub const CURRENT_MOMENTUM: i32 = 0x0001;
            pub const JUMPSQUAT_VELOCITY: i32 = 0x0002;
            pub const GROUND_VEL: i32 = 0x0003;
            pub const RAR_LENIENCY: i32 = 0x0004;
            pub const CURRENT_MOMENTUM_SPECIALS: i32 = 0x0005;
            pub const ECB_BOTTOM_Y_OFFSET: i32 = 0x0006;
            pub const CURR_DASH_SPEED: i32 = 0x0007;
            pub const ECB_CENTER_Y_OFFSET: i32 = 0x0008;
            pub const DASH_HIP_OFFSET_X: i32 = 0x0009;
            pub const RUN_HIP_OFFSET_X: i32 = 0x000A;
            pub const LAND_CANCEL_LAG: i32 = 0x000B;
            pub const ATTACK_S3_CSTICK_X: i32 = 0x000C;

            pub const LEDGE_POS: i32 = 0x0071;
            pub const LEDGE_POS_X: i32 = 0x0072;
            pub const LEDGE_POS_Y: i32 = 0x0073;
            pub const LEDGE_POS_Z: i32 = 0x0074;

            pub const GET_DIST_TO_FLOOR: i32 = 0x0075;
            pub const Y_POS: i32 = 0x0076;
            pub const IS_KILLING_BLOW: i32 = 0x0077;

            pub const ENABLE_FRAME_DATA_DEBUG: i32 = 0x0062;

            pub const LAST_ATTACK_RECEIVER_ENTRY_ID: i32 = 0x0078;
            pub const LAST_ATTACK_DAMAGE_DEALT: i32 = 0x0079;
            pub const IS_ATTACK_CANCEL: i32 = 0x0080;

            pub const JUMP_SPEED_MAX_MUL: i32 = 0x0081;

            pub const IS_IGNORED_STATUS_FRAME_0: i32 = 0x0082;
            pub const FLUSH_EFFECT_ACMD: i32 = 0x0083;
            pub const LAST_GROUNDED_POS: i32 = 0x00084;
            pub const BEFORE_GROUND_COLLISION: i32 = 0x0085;
            pub const JUMP_SPEED_RATIO: i32 = 0x0086;

            pub const IS_TAP_JUMP: i32 = 0x0087;
            pub const JUMP_SQUAT_FRAME: i32 = 0x0088;

            pub const UP_SPECIAL_CANCEL: i32 = 0x0089;
            pub const SIDE_SPECIAL_CANCEL: i32 = 0x0090;
            pub const SIDE_SPECIAL_CANCEL_NO_HIT: i32 = 0x0091;
            pub const DISABLE_AIR_LASSO: i32 = 0x0092;
            pub const PREV_STATUS_INFLICT_STATUS: i32 = 0x0093;
            pub const IS_MOTION_BASED_ATTACK: i32 = 0x0094;
            pub const LAST_ATTACKER_ENTRY_ID: i32 = 0x0078;

            //pub const GET_DIST_TO_FLOOR: i32 = 0x0095;
            pub const AIR_ESCAPE_MAGNET_FRAME: i32 = 0x094;
            pub const FLY_NEXT_FRAME: i32 = 0x095;
            pub const UP_SPECIAL_LAG: i32 = 0x096;
            pub const DAMAGE_DEALT_THIS_STOCK: i32 = 0x097;
            pub const WAVELAND_SPEED_X: i32 = 0x098;
            pub const WAVELAND_PENDING: i32 = 0x099;
            pub const ENABLE_BOOST_RUN: i32 = 0x100;
            pub const TURN_DASH_FRAME: i32 = 0x101;
            pub const MOONWALK_SPEED: i32 = 0x103;
        }

        pub mod status {
            // flags
            pub const DISABLE_ECB_SHIFT: i32 = 0x1001;
            pub const IS_DASH_TO_RUN_FRAME: i32 = 0x1002;
            pub const IS_AFTER_DASH_TO_RUN_FRAME: i32 = 0x1003;
            pub const APPLY_DASH_END_SPEED_MUL: i32 = 0x1004;
            pub const ATTACK_DASH_CANCEL_DISABLE: i32 = 0x1005;
            pub const ATTACK_DASH_ENABLE_AIR_FALL: i32 = 0x1006;
            pub const ATTACK_DASH_ENABLE_AIR_CONTINUE: i32 = 0x1007;
            pub const ATTACK_DASH_ENABLE_AIR_DRIFT: i32 = 0x1008;
            pub const ATTACK_DASH_AIR_DRIFT_ENABLED: i32 = 0x1009;
            pub const ATTACK_DASH_ENABLE_AIR_LANDING: i32 = 0x100A;
            pub const SHOULD_WAVELAND: i32 = 0x100B;
            pub const DAMAGE_FLY_RESET_TRIGGER: i32 = 0x100C;
            pub const CSTICK_IRAR: i32 = 0x100D;
            pub const ENABLE_SPECIAL_WALLJUMP: i32 = 0x100E;
            pub const NO_POCKET: i32 = 0x100F;
            pub const IS_DASH_CANCEL: i32 = 0x1010;
            pub const CHECK_HOLD_INPUT: i32 = 0x1011;
            
            pub const IS_MOONWALK: i32 = 0x1016;
            pub const IS_MOONWALK_JUMP: i32 = 0x1017;

            // ints
            pub const ESCAPE_AIR_CLIFF_CATCH_FRAME: i32 = 0x1000;

            // floats
            pub const INITIAL_KNOCKBACK_VEL_X: i32 = 0x1001;
            pub const INITIAL_KNOCKBACK_VEL_Y: i32 = 0x1002;

            /// FIGHTER_STATUS_KIND_DOWN
            pub const RESTING_HIP_OFFSET_Y: i32 = 0x1003;

            //Meteor Cancel Flags
            pub const TELEPORT_INITIAL_SPEED_X: i32 = 0x1011;
            pub const TELEPORT_INITIAL_SPEED_Y: i32 = 0x1012;
        }
    }

    pub mod captain {
        pub mod status {
            pub const YES: i32 = 0x1100;
        }
    }

    pub mod chrom {

        pub mod status {
            // ints
            pub const SPECIAL_HI_START_SITUATION: i32 = 0x1101;
            // flags
            pub const SPECIAL_HI_DIVE_ENABLE: i32 = 0x1101;
            pub const SPECIAL_HI_DIVE_START: i32 = 0x1102;
            pub const SPECIAL_HI_AERIAL_CANCEL_ENABLE: i32 = 0x1103;
            pub const SPECIAL_LW_LEDGE_CANCEL: i32 = 0x1104;
        }
    }

    pub mod duckhunt {
        pub mod instance {
            // flags
            pub const GUNMAN_TIMER: i32 = 0x0100;
        }
    }


    pub mod falco {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_DISABLE_STALL: i32 = 0x0100;
            pub const SPECIAL_LW_DISABLE_JC: i32 = 0x0101;
        }
        pub mod status {
            // flags
            pub const SPECIAL_LW_SET_ATTACK: i32 = 0x1100;
            pub const SPECIAL_LW_SET_EFFECT: i32 = 0x1101;
            pub const SPECIAL_LW_CONTINUE_MOTION: i32 = 0x1102;

            // ints
            pub const SPECIAL_LW_STOP_Y_FRAME: i32 = 0x1100;
        }
    }

    pub mod fox {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_DISABLE_JC: i32 = 0x0100;
        }
        
        pub mod status {
            // flags
            pub const ILLUSION_SHORTENED: i32 = 0x1100;
            pub const ILLUSION_SHORTEN: i32 = 0x1101;
        }
    }

    pub mod iceclimbers {
        pub mod instance {
            // flags
            pub const IS_VOLUNTARY_SOPO_A: i32 = 0x0100;
            pub const IS_VOLUNTARY_SOPO_B: i32 = 0x0101;
            pub const SPECIAL_AIR_N: i32 = 0x0102;
            pub const SPECIAL_AIR_N_HOP: i32 = 0x0103;
            pub const SPECIAL_AIR_N_SPECIAL_FALL: i32 = 0x0104;
            
            // floats
            pub const LIMIT_GAUGE: i32 = 0x0100;
        }
        pub mod status {
            
        }
    }

    pub mod ike {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_S: i32 = 0x0100;

        }
        pub mod status {
            // flags
            pub const SPECIAL_S_INSTAKILL: i32 = 0x1100;
            pub const SPECIAL_S_GROUND_START: i32 = 0x1101;
            pub const SPECIAL_S_INSTAKILL_CHECK_HIT: i32 = 0x1102;
            pub const SPECIAL_S_INSTAKILL_HIT: i32 = 0x1103;
            pub const IS_QUICK_DRAW_INSTAKILL: i32 = 0x1104;
        }
    }

    pub mod lucario {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_HIT: i32 = 0x0100;
            
            // floats
            pub const PREV_SPEED_X: i32 = 0x0100;
            pub const PREV_SPEED_Y: i32 = 0x0101;
            pub const PREV_LR: i32 = 0x0102;
        }
        pub mod status {
            // ints
            pub const SPECIAL_S_ROT_ANGLE: i32 = 0x1100;
        }
    }
    
    pub mod lucina {
        pub mod instance {
            // int
            /// This int stores damage received from an attack during quick riposte
            pub const CURRENT_DAMAGE: i32 = 0x0100;
        }
        pub mod status {
            // int
            pub const SPECIAL_LW_MOTION: i32 = 0x1100;
            pub const SPECIAL_LW_MOTION_AIR: i32 = 0x1101;

            // flag
            pub const SPECIAL_LW_SPECIAL_CHECK: i32 = 0x1100;
        }
    }

    pub mod mario {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_INT_MTRANS: i32 = 0x0100;

        }
        pub mod status {
            // flags
            pub const SPECIAL_LW_TATSUMAKI: i32 = 0x1100;
            pub const SPECIAL_S_GROUND_START: i32 = 0x1101;
            pub const SPECIAL_S_INSTAKILL_CHECK_HIT: i32 = 0x1102;
            pub const SPECIAL_S_INSTAKILL_HIT: i32 = 0x1103;
        }
    }

    pub mod metaknight {
        pub mod instance {
            // flags
            pub const SPECIAL_S_HIT: i32 = 0x0100;
        }
    }

    pub mod ridley {
        pub mod instance {
            // flags
            pub const SPECIAL_N_ATTACK: i32 = 0x0104;
            pub const SPECIAL_LW_IS_SKEWER: i32 = 0x0105;
        }
        pub mod status {
            // flags
            pub const SPECIAL_HI_HOVER_DECIDE_STICK: i32 = 0x1100;
            pub const SPECIAL_LW_POGO_ENABLE_LANDING: i32 = 0x1101;
            pub const SPECIAL_LW_POGO_CHECK_BOUNCE: i32 = 0x1102;

            // floats
            pub const SPECIAL_HI_HOVER_DECIDE_STICK_X: i32 = 0x1100;
            pub const SPECIAL_HI_HOVER_DECIDE_STICK_Y: i32 = 0x1101;
            pub const SPECIAL_HI_CHARGE_DIR: i32 = 0x1102;
            pub const SPECIAL_LW_STICK_Y: i32 = 0x1103;
            pub const SPECIAL_LW_POGO_CHECK_PREV_X: i32 = 0x01104;
            pub const SPECIAL_LW_POGO_CHECK_PREV_Y: i32 = 0x01105;
        }
    }
    
    pub mod roy {
        pub mod instance {
            // flags
            pub const TRAIL_EFFECT: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SPECIAL_S4_REVERSE: i32 = 0x1150;
        }
    }

    pub mod samus {
        pub mod instance {
            // flags
            pub const ICE_MODE: i32 = 0x0100;
            pub const SPEEDBOOSTER_ON: i32 = 0x0101;
            pub const SHINESPARK_ON: i32 = 0x0102;
            pub const SPECIAL_HI_HOP_DISABLE: i32 = 0x0103;

            // ints
            pub const SPEEDBOOSTER_STICK_TIMER: i32 = 0x0100;
            pub const SPEEDBOOSTER_EFFECT_TIMER: i32 = 0x0101;
            pub const SHINESPARK_CHARGE_TIMER: i32 = 0x0102;
            pub const SHINESPARK_EFFECT_TIMER: i32 = 0x0103;

            // floats
            pub const AIM_ANGLE: i32 = 0x0100;
            pub const SPECIAL_N_THROW_LW_CHARGE_STORAGE: i32 = 0x0101;
        }
        pub mod status {
            // flags 
            pub const SHINESPARK_IS_SPECIAL_LW: i32 = 0x1100;
            pub const SHINESPARK_ENABLE_GRAVITY: i32 = 0x1101;
            pub const SHINESPARK_ENABLE_CONTROL: i32 = 0x1102;
            pub const ATTACK_LW3_CHECK_CEIL: i32 = 0x1103;
            pub const SPECIAL_HI_LOCK_ANGLE: i32 = 0x1104;
            pub const SPECIAL_HI_FIX_GBEAM_POS: i32 = 0x1105;
            pub const SPECIAL_LW_BOMB_JUMP_ON: i32 = 0x1106;
            pub const SPECIAL_LW_BOMB_JUMP_HOP: i32 = 0x1107;

            // ints 
            pub const SHINESPARK_AIM_TIMER: i32 = 0x1100;
            pub const SHINESPARK_AIM_EFFECT_TIMER: i32 = 0x1101;
            pub const SHINESPARK_LOOP_TIMER: i32 = 0x1102;
            pub const SPECIAL_LW_JUMP_COUNT_FIX: i32 = 0x1103;

            // floats
            pub const SPECIAL_HI_ANGLE: i32 = 0x1100;
        }
    }

    pub mod wolf {
        pub mod instance {
            
        }
        pub mod status {
            // flags
            pub const ILLUSION_SHORTENED: i32 = 0x1100;
            pub const ILLUSION_SHORTEN: i32 = 0x1101;
            pub const SPECIAL_S_DISABLE: i32 = 0x1100;
        }
    }
}

pub mod statuses {
    pub mod falco {
        pub const SPECIAL_LW_LOOP: i32 = 0x1e8;
        pub const SPECIAL_LW_END: i32 = 0x1e9;
        pub const SPECIAL_LW_HIT: i32 = 0x1ea;
    }

    pub mod ridley {
        pub const SPECIAL_LW_POGO: i32 = 0x203;
        pub const SPECIAL_LW_LANDING: i32 = 0x204;
    }

    pub mod wolf {
        pub const SPECIAL_S_RUSH: i32 = 0x1EA;
        pub const SPECIAL_S_END: i32 = 0x1EB;
    }
}

pub mod articles {
    
}

pub mod guard {
    pub mod flag {
        pub const ADD_BUFFER : i32 = 0x1050;
        pub const SET_SHIELD_LOW_SMOKE : i32 = 0x1051;
        pub const VALID_GUARD_CANCEL_INPUT : i32 = 0x1052;
        pub const VALID_GUARD_CANCEL_TAUNT_INPUT : i32 = 0x1053;
    }
    pub mod int {
        pub const SHIELD_EFF_ID : i32 = 0x1050;
        pub const GUARD_OFF_RESERVE_CAT1 : i32 = 0x1051;
        pub const JUST_SHIELD_COUNT : i32 = 0x1052;

        pub const GUARD_CANCEL_STATUS : i32 = 0x1053;
        pub const JUST_SHIELD_XLU_COUNT : i32 = 0x1054;
    }
    pub mod float {
        pub const GUARD_DAMAGE_FACING_DIR : i32 = 0x1050;
    }
}


pub mod melee_mode {
    pub const SMASH: i32 = 0x0;

    pub const CUSTOM_SMASH: i32 = 0x3;
    pub const SUPER_SUDDEN_DEATH: i32 = 0x4;
    pub const SMASHDOWN: i32 = 0x5;
    pub const SPIRIT_BOARD: i32 = 0x6;
    pub const ADVENTURE: i32 = 0x7;

    pub const CLASSIC: i32 = 0x9;
    pub const MOB_SMASH: i32 = 0xa;
    pub const TRAINING: i32 = 0xb;

    pub const HOMERUN_SOLO: i32 = 0xd;
    pub const HOMERUN_CO_OP: i32 = 0xe;
    pub const HOMERUN_VERSUS: i32 = 0xf;
    pub const STAGE_BUILDER: i32 = 0x10;

    pub const ARENA: i32 = 0x13;

    pub const TIPS: i32 = 0x1b;
}

pub mod smash_mode {
    pub const TIME: i32 = 0x0;
    pub const STOCK: i32 = 0x1;
}

// extra lua_consts
pub const COLLISION_KIND_MASK_PARRY: smash::lib::LuaConst = smash::lib::LuaConst::new(0x80);

pub static mut IS_SALTY_RUNBACK: bool = false;
pub static mut IS_SALTY_MATCH_EXIT: bool = false;

pub const NUM_ANGLES_CHECKED: i32 = 12;
pub const NUM_ANGLES_CHECKED_FINAL: i32 = 12;
// maximum number of survivable DI angles in a finishing hit
pub const SURVIVABLE_ANGLES_ALLOWED: i32 = 0;
pub const SURVIVABLE_ANGLES_ALLOWED_FINAL: i32 = 1;
// how many units into the blastzone a fighter will be declared dead
pub const DEAD_AREA_LENIENCY: f32 = 7.5;
pub const DEAD_AREA_LENIENCY_FINAL: f32 = 2.5;

pub static mut LAST_ATTACK_HITBOX_ID: i32 = 0;
pub static mut LAST_ATTACK_HITBOX_LOCATION_X: f32 = 0.0;
pub static mut LAST_ATTACK_HITBOX_LOCATION_Y: f32 = 0.0;
pub static mut LAST_ATTACK_HITBOX_LOCATION_Z: f32 = 0.0;
pub static mut SHOULD_END_RESULT_SCREEN: bool = false;


pub const FIGHTER_INSTANCE_WORK_ID_FLAG_CALCULATING_ECB: LuaConst = LuaConst::new(0x2000011D);
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_ECB_OFFSET_B: LuaConst = LuaConst::new(0x645);
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_ECB_OFFSET_C: LuaConst = LuaConst::new(0x646);