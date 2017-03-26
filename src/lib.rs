use std::thread;
use std::time;

pub enum Action {
    Stop,
    Continue,
}

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

pub struct Looper {
    fps: f32,
}

struct Realtime {
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
struct RealtimeIter {
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

impl Looper {
    pub fn new(fps: f32) -> Self {
        Looper { fps: fps }
    }

    pub fn run<R, U>(&self, mut render: R, mut update: U)
        where R: FnMut(i32) -> Action,
              U: FnMut(i32) -> Action
    {
        let mut realtime = Realtime::new(self.fps);
        let mut rps = PerSecond::new();
        let mut ups = PerSecond::new();

        loop {
            rps.tick();

            match render(rps.get_fps()) {
                Action::Stop => break,
                Action::Continue => (),
            };

            for acc in realtime.tick() {
                ups.tick();
                match update(ups.get_fps()) {
                    Action::Stop => break,
                    Action::Continue => (),
                }
                realtime.acc(acc);
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
        let mut state = 1;
        let render = move |_| if state != 0 {
            state -= 1;
            Action::Continue
        } else {
            Action::Stop
        };

        let update = |_| Action::Continue;

        Looper::new(60.0).run(render, update);
    }

    #[test]
    fn it_renders() {
        let mut state = 1;
        let mut rendered = 0;

        {
            let render = |_| if state != 0 {
                rendered += 1;
                state -= 1;
                Action::Continue
            } else {
                Action::Stop
            };

            let update = |_| Action::Continue;
            Looper::new(60.0).run(render, update);
        }

        assert_eq!(rendered, 1);
    }

    #[test]
    fn it_updates() {
        let mut state = 2;
        let mut updated = 0;

        {
            let render = |_fps| if state != 0 {
                state -= 1;
                Action::Continue
            } else {
                Action::Stop
            };

            let update = |_| {
                updated += 1;
                Action::Continue
            };

            Looper::new(60.0).run(render, update);
        }

        assert_eq!(updated, 1);
    }

    #[test]
    fn it_renders_and_updates() {
        let mut state = 60;
        let mut rendered = 0;
        let mut updated = 0;

        {
            let render = |_| if state != 0 {
                rendered += 1;
                state -= 1;
                Action::Continue
            } else {
                Action::Stop
            };

            let update = |_| {
                updated += 1;
                Action::Continue
            };

            Looper::new(60.0).run(render, update);
        }

        assert_eq!(rendered, 60);
        assert_eq!(updated, 59);
    }

    #[test]
    fn it_rps() {
        let mut state = 61;
        let mut rps = 0;

        {
            let render = |fps| if state != 0 {
                rps = fps;
                state -= 1;
                Action::Continue
            } else {
                Action::Stop
            };

            let update = |_| Action::Continue;

            Looper::new(60.0).run(render, update);
        }

        assert_eq!(rps, 60);
    }

    #[test]
    fn it_ups() {
        let mut state = 61;
        let mut ups = 0;

        {
            let render = |_fps| if state != 0 {
                state -= 1;
                Action::Continue
            } else {
                Action::Stop
            };

            let update = |fps| {
                ups = fps;
                Action::Continue
            };

            Looper::new(60.0).run(render, update);
        }

        assert_eq!(ups, 59);
    }
}
