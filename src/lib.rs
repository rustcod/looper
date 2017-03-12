use std::thread;
use std::time;

pub enum Action {
    Stop,
    Continue,
}

pub struct Looper {
    fps: f32,
}

impl Looper {
    pub fn new(fps: f32) -> Self {
        Looper { fps: fps }
    }

    pub fn run<F>(&self, mut f: F)
        where F: FnMut() -> Action
    {
        let mut acc = time::Duration::new(0, 0);
        let mut prev = time::Instant::now();
        let step = time::Duration::new(0, (1.0 / self.fps * 1_000_000_000.0) as u32);

        loop {
            match f() {
                Action::Stop => break,
                Action::Continue => (),
            };

            let now = time::Instant::now();
            acc += now - prev;
            prev = now;

            while acc >= step {
                acc -= step;
                // game
            }

            thread::sleep(step - acc);
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
