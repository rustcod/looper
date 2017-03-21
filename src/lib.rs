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
    curr: time::Instant,
    iter: RealtimeIter,
}

impl Realtime {
    pub fn new(fps: f32) -> Self {
        Realtime {
            acc: time::Duration::new(0, 0),
            curr: time::Instant::now(),
            iter: RealtimeIter::new(time::Duration::new(0, (1.0 / fps * 1_000_000_000.0) as u32)),
        }
    }

    pub fn tick(&mut self) -> RealtimeIter {
        let new = time::Instant::now();
        self.curr = new;
        self.acc += new - self.curr;
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
    type Item = ();

    fn next(&mut self) -> Option<()> {
        while self.acc >= self.step {
            self.acc -= self.step;
            return Some(());
        }
        // XXX
        if self.update {
            self.update = false;
            return Some(());
        } else {
            thread::sleep(self.step - self.acc);
            None
        }
    }
}

impl Looper {
    pub fn new(fps: f32) -> Self {
        Looper { fps: fps }
    }

    pub fn run<R, U>(&self, mut render: R, mut update: U)
        where R: FnMut() -> Action,
              U: FnMut() -> Action
    {
        let mut realtime = Realtime::new(self.fps);

        loop {
            match render() {
                Action::Stop => break,
                Action::Continue => (),
            };

            for _ in realtime.tick() {
                match update() {
                    Action::Stop => break,
                    Action::Continue => (),
                }
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

        let render = move || if state != 0 {
            state -= 1;
            Action::Continue
        } else {
            Action::Stop
        };

        let update = || Action::Continue;

        Looper::new(60.0).run(render, update);
    }
}
