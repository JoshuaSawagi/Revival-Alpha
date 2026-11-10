use super::*;

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    FT_MOTION_RATE_RANGE(agent, 4.0, 15.0, 9.0);
    if is_excute(agent) {
        HitModule::set_hit_stop_mul(boma, 3.0, HitStopMulTarget{_address: (*HIT_STOP_MUL_TARGET_OPPONENT) as u8}, 0.0);
        DamageModule::set_damage_mul(boma, 0.5);
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        VarModule::set_float(boma.object(), vars::lucina::instance::CURRENT_DAMAGE, DamageModule::damage(boma, 0));
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        DamageModule::set_damage_mul(boma, 1.0);
        HitModule::set_hit_stop_mul(boma, 1.0, HitStopMulTarget{_address: (*HIT_STOP_MUL_TARGET_OPPONENT) as u8}, 0.0);
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        VarModule::on_flag(agent.battle_object, vars::lucina::status::SPECIAL_LW_SPECIAL_CHECK);
    }
    frame(lua_state, 26.0);
    FT_MOTION_RATE_RANGE(agent, 26.0, 64.0, 16.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::lucina::status::SPECIAL_LW_SPECIAL_CHECK);
    }
    frame(lua_state, 64.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("lucina_counter_flash"), Hash40::new("top"), 0, 11, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_aura"), Hash40::new("armr"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_aura"), Hash40::new("claviclel"), -0.0, 0, 0, 0, 0, 0, 2, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_aura"), Hash40::new("clavicler"), -0.0, 0, 0, 0, 0, 0, 2, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_aura"), Hash40::new("hip"), -0.0, 0, 0, 0, 0, 0, 2.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_aura"), Hash40::new("kneer"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_aura"), Hash40::new("kneel"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_aura"), Hash40::new("footr"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_aura"), Hash40::new("footl"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("lucina_mc_aura"), false, true);
    }
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    FT_MOTION_RATE_RANGE(agent, 4.0, 15.0, 9.0);
    if is_excute(agent) {
        HitModule::set_hit_stop_mul(boma, 3.0, HitStopMulTarget{_address: (*HIT_STOP_MUL_TARGET_OPPONENT) as u8}, 0.0);
        DamageModule::set_damage_mul(boma, 0.5);
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        VarModule::set_float(boma.object(), vars::lucina::instance::CURRENT_DAMAGE, DamageModule::damage(boma, 0));
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        DamageModule::set_damage_mul(boma, 1.0);
        HitModule::set_hit_stop_mul(boma, 1.0, HitStopMulTarget{_address: (*HIT_STOP_MUL_TARGET_OPPONENT) as u8}, 0.0);
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        VarModule::on_flag(agent.battle_object, vars::lucina::status::SPECIAL_LW_SPECIAL_CHECK);
    }
    frame(lua_state, 28.0);
    FT_MOTION_RATE_RANGE(agent, 28.0, 64.0, 30.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::lucina::status::SPECIAL_LW_SPECIAL_CHECK);
    }
    frame(lua_state, 64.0);
    FT_MOTION_RATE(agent, 1.0);
}


pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_specialairlw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_speciallw, Priority::Low);
}