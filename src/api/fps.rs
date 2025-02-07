use std::time::{Duration, Instant};

pub struct FpsLimiter {
	fps: f64,
	last_frame: Instant,
	target_frametime: Option<Duration>,
}

impl FpsLimiter {
	pub fn new() -> Self {
		Self {
			last_frame: Instant::now(),
			target_frametime: None,
			fps: 0.0f64,
		}
	}

	pub fn set_fps(&mut self, fps: Option<f64>) {
		match fps {
			Some(fps) if 0.0f64 < fps => {
				self.fps = fps;
				self.target_frametime = Some(Duration::from_secs(1).div_f64(fps));
			}
			_ => {
				self.fps = 0.0f64;
				self.target_frametime = None;
			}
		}
	}

	pub fn get_fps(&self) -> f64 {
		self.fps
	}

	pub fn limit_fps(&mut self) {
		let Some(target_frametime) = self.target_frametime else {
			return;
		};
		let mut current_frame = Instant::now();
		let next_frame = self.last_frame + target_frametime;
		while current_frame < next_frame {
			// std::thread::yield_now(); // OMNOMNOMNOMNOM delicious CPU cycles :]
			current_frame = Instant::now();
		}
		self.last_frame = current_frame;
	}
}
