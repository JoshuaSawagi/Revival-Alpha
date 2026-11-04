
use crate::offsets;
use std::collections::HashSet;
use std::str::FromStr;

static mut IS_PENDING_NEW_GAME: bool = false;


#[export_name = "hdr__game_modes__signal_new_game"]
pub extern "Rust" fn signal_new_game() {
    unsafe {
        IS_PENDING_NEW_GAME = true;
    }
}

static mut PREVIOUS_GAME_STATE_PTR: u64 = 0;

fn detect_new_game(game_state_ptr: u64) -> bool {
    unsafe {
        let prev = PREVIOUS_GAME_STATE_PTR;
        PREVIOUS_GAME_STATE_PTR = game_state_ptr;

        let pending = IS_PENDING_NEW_GAME;
        IS_PENDING_NEW_GAME = false;

        prev != game_state_ptr || pending
    }

}

#[export_name = "hdr__game_modes__get_melee_mode"]
pub extern "Rust" fn get_melee_mode() -> i32 {
    unsafe {
        return CURRENT_MELEE_MODE;
    }
}

static mut CURRENT_MELEE_MODE: i32 = 0x0;

// updates when initiating the CSS of any game mode
#[skyline::hook(offset = 0x1a2625c, inline)]
unsafe fn read_melee_mode(ctx: &mut skyline::hooks::InlineCtx) {
    let mode = ctx.registers[8].x();
    CURRENT_MELEE_MODE = mode as i32;
}

pub fn install() {
    skyline::install_hook!(
        read_melee_mode
    );
}