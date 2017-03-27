use std::thread;
use std::time;

mod action;
mod per_second;
mod looper;
mod realtime;

pub use action::Action;
pub use looper::Looper;

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
