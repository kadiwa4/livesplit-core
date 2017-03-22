use livesplit_core::component::possible_time_save::State as PossibleTimeSaveComponentState;
use super::{drop, acc, output_str};
use libc::c_char;

pub type OwnedPossibleTimeSaveComponentState = *mut PossibleTimeSaveComponentState;

#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponentState_drop(this: OwnedPossibleTimeSaveComponentState) {
    drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponentState_text(this: *const PossibleTimeSaveComponentState)
                                                  -> *const c_char {
    output_str(&acc(this).text)
}

#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponentState_time(this: *const PossibleTimeSaveComponentState)
                                                  -> *const c_char {
    output_str(&acc(this).time)
}
