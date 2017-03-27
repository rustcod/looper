use action;
use per_second;
use realtime;

pub struct Looper {
    fps: f32,
}

impl Looper {
    pub fn new(fps: f32) -> Self {
        Looper { fps: fps }
    }

    pub fn run<R, U>(&self, mut render: R, mut update: U)
        where R: FnMut(i32) -> action::Action,
              U: FnMut(i32) -> action::Action
    {
        let mut realtime = realtime::Realtime::new(self.fps);
        let mut rps = per_second::PerSecond::new();
        let mut ups = per_second::PerSecond::new();

        loop {
            rps.tick();

            match render(rps.get_fps()) {
                action::Action::Stop => break,
                action::Action::Continue => (),
            };

            for acc in realtime.tick() {
                ups.tick();
                match update(ups.get_fps()) {
                    action::Action::Stop => break,
                    action::Action::Continue => (),
                }
                realtime.acc(acc);
            }
        }
    }
}
