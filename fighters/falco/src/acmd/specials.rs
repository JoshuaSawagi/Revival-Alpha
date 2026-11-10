use super::*;

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        // This allows falco jump stalling. DO NOT REMOVE UNLESS YOU FEEL YOU ARE GOING TO STATUS SCRIPT IT.
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let shine_vec = Vector3f { x: 0.25, y: 0.0, z: 1.0 };
        KineticModule::mul_speed(boma, &shine_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let shineGravity = Vector3f { x: 1.0, y: 0.1588, z: 1.0 };
        KineticModule::mul_accel(boma, &shineGravity, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 84, 48, 0, 92, 8.0, 0.0, 8.0, 0.5, None, None, None, 0.6, 1.15, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.68);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {  }frame(lua_state, 51.0);
    if is_excute(agent) {
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
    }
    
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        // This allows falco jump stalling. DO NOT REMOVE UNLESS YOU FEEL YOU ARE GOING TO STATUS SCRIPT IT.
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let shine_vec = Vector3f { x: 0.25, y: 0.0, z: 1.0 };
        KineticModule::mul_speed(boma, &shine_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let shineGravity = Vector3f { x: 1.0, y: 0.1588, z: 1.0 };
        KineticModule::mul_accel(boma, &shineGravity, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 84, 48, 0, 92, 8.0, 0.0, 8.0, 0.5, None, None, None, 0.6, 1.15, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.68);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {  }frame(lua_state, 51.0);
    if is_excute(agent) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
    }
    
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
		//LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		EFFECT_FOLLOW(agent, Hash40::new("falco_ref_loop"), Hash40::new("top"), 0, 7, -2.0, 0, 0, 0, 0.8, true);
		EffectModule::preset_limit_num(boma, 2);
		EFFECT_FOLLOW(agent, Hash40::new("falco_ref_flash"), Hash40::new("reflector"), 1.4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("falco_ref_ref"), Hash40::new("top"), 0, 7.27, -2.0, 0, 0, 0, 0.5, true);
        //FLASH(agent, 1, 1, 1, 0.627);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 5.0, 0, 0, 0, 0, 2.8, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
        LAST_EFFECT_SET_RATE(agent, 1.5);
	}
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_shield_on"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_item_get"));
    }
}

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 6.0/(14.0-1.0));
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }
}

unsafe extern "C" fn game_specialnloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //DamageModule::add_damage(boma, 1.0, 0);
        FT_MOTION_RATE(agent, 10.0/(4.0-1.0));
        WorkModule::on_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("loop"), false, 0.0);
        }
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 0.25);
    frame(lua_state, 17.0);
    if is_excute(agent) {
    WorkModule::off_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
}

unsafe extern "C" fn game_specialairnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 4.0/(5.0-1.0));
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }
}

unsafe extern "C" fn game_specialairnloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //DamageModule::add_damage(boma, 1.0, 0);
        WorkModule::on_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
        FT_MOTION_RATE(agent, 5.0/(4.0-1.0));
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        //DamageModule::add_damage(boma, 1.0, 0);
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("loop"), false, 0.0);
        }
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
        if WorkModule::is_flag(boma,*FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP){
            //laser loop motion rate(changing this allows for faster or slower sequential lasers from the first)
            FT_MOTION_RATE(agent,0.10);
        }
    }
}

unsafe extern "C" fn sound_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_falco_special_n02"));
    }
}

pub fn install(agent: &mut Agent) {

    agent.acmd("game_specialnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialnloop", game_specialnloop, Priority::Low);
    agent.acmd("game_specialairnstart", game_specialairnstart, Priority::Low);
    agent.acmd("game_specialairnloop", game_specialairnloop, Priority::Low);

    agent.acmd("sound_specialnstart", sound_specialnstart, Priority::Low);
    agent.acmd("sound_specialairnstart", sound_specialnstart, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_speciallw, Priority::Low);
    agent.acmd("expression_speciallw", expression_speciallw, Priority::Low);
    agent.acmd("expression_specialairlw", expression_speciallw, Priority::Low);
    agent.acmd("sound_speciallw", sound_speciallw, Priority::Low);
    agent.acmd("sound_specialairlw", sound_speciallw, Priority::Low);

}