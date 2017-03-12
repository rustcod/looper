use std::thread;
use std::time;

pub enum Action {
    Stop,
    Continue,
}

pub struct Looper {
    fps: f32,
}

struct Realtime {
    acc: time::Duration,
    prev: time::Instant,
    iter: RealtimeIter,
}

impl Realtime {
    pub fn new(fps: f32) -> Self {
        Realtime {
            acc: time::Duration::new(0, 0),
            prev: time::Instant::now(),
            iter: RealtimeIter::new(time::Duration::new(0, (1.0 / fps * 1_000_000_000.0) as u32)),
        }
    }

    pub fn tick(&mut self) -> RealtimeIter {
        let now = time::Instant::now();
        self.acc = self.acc + (now - self.prev);
        self.prev = now;
        self.iter.set(self.acc);
        self.iter.clone()
    }
}

#[derive(Clone)] // TODO remove
struct RealtimeIter {
    acc: time::Duration,
    step: time::Duration,
}

impl RealtimeIter {
    pub fn new(step: time::Duration) -> Self {
        RealtimeIter {
            acc: time::Duration::new(0, 0),
            step: step,
        }
    }

    pub fn set(&mut self, acc: time::Duration) {
        self.acc = acc;
    }
}

impl Iterator for RealtimeIter {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        while self.acc >= self.step {
            self.acc -= self.step;
            return Some(());
        }
        thread::sleep(self.step - self.acc);
        None
    }
}

impl Looper {
    pub fn new(fps: f32) -> Self {
        Looper { fps: fps }
    }

    pub fn run<F>(&self, mut f: F)
        where F: FnMut() -> Action
    {
        let mut realtime = Realtime::new(self.fps);

        loop {
            match f() {
                Action::Stop => break,
                Action::Continue => (),
            };

            for _ in realtime.tick() {
                println!("game!");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use Action;
    use Looper;

    #[test]
    fn it_works() {
        let mut state = 2;
        Looper::new(60.0).run(move || if state != 0 {
            state -= 1;
            Action::Continue
        } else {
            Action::Stop
        });
    }
}
