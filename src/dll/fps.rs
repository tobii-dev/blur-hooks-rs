use std::time::{Duration, Instant};

pub fn limit_fps(fps: f64) {
	let target_frametime = Duration::from_secs(1).div_f64(fps);
	let mut current = Instant::now();
	static mut LAST_PRESENT: Option<Instant> = None;
	let last_frame = unsafe {
		if LAST_PRESENT.is_none() {
			LAST_PRESENT = Some(current);
		}
		LAST_PRESENT.unwrap()
	};
	let next_frame = last_frame + target_frametime;
	while current < next_frame {
		// std::thread::yield_now(); // OMNOMNOMNOMNOM delicious CPU cycles
		current = Instant::now();
	}
	unsafe { LAST_PRESENT = Some(current) };
}
