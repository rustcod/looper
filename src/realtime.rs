use std::thread;
use std::time;

pub struct Realtime {
    acc: time::Duration,
    curr: time::Instant,
    iter: RealtimeIter,
}

impl Realtime {
    pub fn new(fps: f32) -> Self {
        let step = time::Duration::new(0, (1.0 / fps * 1_000_000_000.0) as u32);

        Realtime {
            acc: time::Duration::new(0, 0),
            curr: time::Instant::now(),
            iter: RealtimeIter::new(step),
        }
    }

    pub fn acc(&mut self, acc: time::Duration) {
        self.acc = acc
    }

    pub fn tick(&mut self) -> RealtimeIter {
        let now = time::Instant::now();
        self.acc += now - self.curr;
        self.curr = now;
        self.iter.set(self.acc);
        self.iter.clone()
    }
}

#[derive(Clone)] // TODO remove
pub struct RealtimeIter {
    acc: time::Duration,
    step: time::Duration,
    update: bool,
}

impl RealtimeIter {
    pub fn new(step: time::Duration) -> Self {
        RealtimeIter {
            acc: time::Duration::new(0, 0),
            step: step,
            update: true,
        }
    }

    pub fn set(&mut self, acc: time::Duration) {
        self.acc = acc;
    }
}

impl Iterator for RealtimeIter {
    type Item = time::Duration;

    fn next(&mut self) -> Option<time::Duration> {
        while self.acc >= self.step {
            self.acc -= self.step;
            return Some(self.acc);
        }

        thread::sleep(self.step - self.acc);
        None
    }
}
