use std::ffi::{c_char, CStr};

use windows::{core::PCSTR, Win32::System::LibraryLoader::GetModuleHandleA};

pub fn get_saved_profile_username() -> String {
	let ptr_base: *mut std::ffi::c_void =
		unsafe { GetModuleHandleA(PCSTR::null()) }.unwrap().0 as _;

	// "Blur.exe"+0xE144E1
	const OFFSET_PROFILE_USERNAME: isize = 0xE144E1;

	let ptr = ptr_base.wrapping_offset(OFFSET_PROFILE_USERNAME) as *const c_char;
	let s = unsafe { CStr::from_ptr(ptr) };
	s.to_str()
		.expect("Could not read username as UTF-8 str from profile.")
		.to_string()
}
