use std::time::{Duration, Instant};

pub fn limit_fps(fps: f64) {
	static mut LAST_FRAME: Option<Instant> = None;
	let target_frametime = Duration::from_secs(1).div_f64(fps);
	let mut current_frame = Instant::now();
	let last_frame = unsafe {
		if LAST_FRAME.is_none() {
			LAST_FRAME = Some(current_frame);
		}
		LAST_FRAME.unwrap()
	};
	let next_frame = last_frame + target_frametime;
	while current_frame < next_frame {
		// std::thread::yield_now(); // this line is commented out because OMNOMNOMNOMNOM delicious CPU cycles :]
		current_frame = Instant::now();
	}
	unsafe { LAST_FRAME = Some(current_frame) };
}
