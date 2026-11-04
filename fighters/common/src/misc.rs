use smash::app::lua_bind::*;
use smash::app::*;
use utils::ext::*;
use utils::*;

#[skyline::hook(offset = 0x6417f4, inline)]
unsafe fn shield_damage_analog(ctx: &skyline::hooks::InlineCtx) {
    let boma =
        *(*ctx.registers[0].x.as_ref() as *const u64).add(1) as *mut BattleObjectModuleAccessor;
    let current_shield = WorkModule::get_float(boma, 6);
    let attack_power = *(*ctx.registers[19].x.as_ref() as *const f32).add(0xf730 / 4);
    let analog = InputModule::get_analog_for_guard((*boma).object());
    let damage_mul = WorkModule::get_param_float(
        boma,
        smash::hash40("common"),
        smash::hash40("shield_damage_mul"),
    );
    let damage_mul = if analog > 0.0 && analog < 1.0 {
        damage_mul + 0.2 * (1.0 - analog)
    } else {
        damage_mul
    };
    WorkModule::set_float(boma, current_shield - attack_power * damage_mul, 6);
}

#[skyline::hook(offset = 0x6285d0, inline)]
unsafe fn shield_pushback_analog(ctx: &skyline::hooks::InlineCtx) {
    let fighter = *ctx.registers[19].x.as_ref();
    let boma = *(fighter as *const u64).add(4);
    let attack_module: u64 = *(boma as *const u64).add(0xa0 / 8);
    let transactor_count: u64 = *(attack_module as *const u64).add(0x20 / 8);
    let transactors: u64 = *(attack_module as *const u64).add(0x28 / 8);

    let mul = WorkModule::get_param_float(
        boma as _,
        smash::hash40("common"),
        smash::hash40("shield_rebound_speed_mul"),
    );

    for x in 0..transactor_count {
        let transactor = transactors + 720 * x;
        let p_list = *(transactor as *const u64).add(608 / 8);
        if p_list == 0 {
            continue;
        }

        let mut current = *(p_list as *const u64);
        while current != p_list && current != 0 {
            if *(current as *const u8).add(47) == 2 {
                let battle_object_id = *(current as *const u32).add(36 / 4);
                let object = utils::util::get_battle_object_from_id(battle_object_id);
                let analog = InputModule::get_analog_for_guard(object);
                let mul = if analog > 0.0 && analog < 1.0 {
                    mul * analog * 0.1
                } else {
                    mul
                };
                std::arch::asm!("fmov s0, w8", in("w8") mul);
                return;
            }

            current = *(current as *const u64);
        }
    }
    std::arch::asm!("fmov s0, w8", in("w8") mul);
}

pub fn install() {
    skyline::patching::Patch::in_text(0x6417f4).nop();
    skyline::patching::Patch::in_text(0x6285d0).nop();
    skyline::install_hooks!(shield_damage_analog, shield_pushback_analog);
}