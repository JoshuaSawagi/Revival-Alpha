// status imports
use super::*;
use globals::*;

pub fn install() {
    /*
    unsafe {
        // linear hitstun patch
        skyline::patching::Patch::in_text(0x62BA74).data(0xD2800000u32);
        
    }
    */
    //skyline::install_hook!(stub_kill_screen);
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_dead_uniq_process_init_hook,
            
        );
    }
}
/*
#[skyline::hook(offset=0x6a7100)] // unadjusted for 13.0.2?
pub fn stub_kill_screen() {}
*/
// this runs as you are KO'd
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_dead_uniq_process_init)]
pub unsafe fn sub_dead_uniq_process_init_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Kill rage smoke gfx on star/screen KO
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_steam1"), true, true);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_steam2"), true, true);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_steam3"), true, true);
    original!()(fighter)
}
