use super::*;
use globals::*;
use skyline::hooks::{getRegionAddress, Region, InlineCtx};
use crate::sv_battle_object::module_accessor;
pub static mut FLOAT_OFFSET: usize = 0x4E53C0;

pub static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];

pub static mut INT_OFFSET : usize = 0x4E5380;

pub static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn get_param_float_hook(x0 /*boma*/: u64, x1 /*param_type*/: u64, x2 /*param_hash*/: u64) -> f32 {
    let mut boma = *((x0 as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
    let boma_reference = &mut *boma;
    let fighter_kind = boma_reference.kind();
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if boma_reference.is_fighter() {
        // Coupled with "landing_heavy" change in change_motion hook
        // Because we start heavy landing anims on f2 rather than f1, we need to push back the heavy landing FAF by 1 frame so it is accurate to the defined per-character param
        if x1 == hash40("landing_frame") {
            return original!()(x0, hash40("landing_frame"), 0) + 1.0;
        }
    }
    original!()(x0, x1, x2)
}


#[skyline::hook(offset=0x6a7100)] /*Need to find the actual Offset*/
pub fn stub_kill_screen() {

}

pub fn install() {
	unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
    }
	skyline::install_hooks!(get_param_float_hook);
    //skyline::install_hooks!(stub_kill_screen);
}