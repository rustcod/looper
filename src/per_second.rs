use std::time;

pub struct PerSecond {
    frames: i32,
    fps: i32,
    start: time::Instant,
    curr: u64,
}

impl PerSecond {
    pub fn new() -> Self {
        let start = time::Instant::now();

        PerSecond {
            frames: 0,
            fps: 0,
            start: start,
            curr: start.elapsed().as_secs(),
        }
    }

    pub fn tick(&mut self) {
        let next = self.start.elapsed().as_secs();
        if self.curr < next {
            self.reset(next);
        }

        self.frames += 1;
    }

    pub fn get_fps(&self) -> i32 {
        self.fps
    }

    fn reset(&mut self, next: u64) {
        self.curr = next;
        self.fps = self.frames;
        self.frames = 0;
    }
}
