#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::ops::AddAssign;
    use std::rc::Rc;
    use std::time::{Duration, Instant};

    use task8::{Clock, EventStatistic, LastHourEventRPMStatistic};

    struct TestClock {
        time: RefCell<Instant>,
    }

    impl Clock for TestClock {
        fn now(&self) -> Instant {
            *self.time.borrow()
        }
    }

    impl TestClock {
        pub fn move_time(&self, offset: Duration) {
            self.time.borrow_mut().add_assign(offset);
        }
    }

    fn setup() -> (Rc<TestClock>, LastHourEventRPMStatistic<TestClock>) {
        let clock = Rc::new(TestClock { time: RefCell::new(Instant::now()) });
        let event_statistic = LastHourEventRPMStatistic::<TestClock>::new(clock.clone());
        (clock, event_statistic)
    }

    #[test]
    fn no_events() {
        let (_clock, mut event_statistic) = setup();
        assert_eq!(event_statistic.get_all_event_statistic().len(), 0);
    }

    #[test]
    fn multiple_events() {
        let (_clock, mut event_statistic) = setup();
        event_statistic.inc_event("test1");
        event_statistic.inc_event("test2");
        assert_eq!(event_statistic.get_all_event_statistic(), HashMap::<String, f32>::from([
            ("test1".to_owned(), 1.0 / 60.0),
            ("test2".to_owned(), 1.0 / 60.0)
        ]));
        assert_eq!(event_statistic.get_event_statistic_by_name("test1"), 1.0 / 60.0);
        assert_eq!(event_statistic.get_event_statistic_by_name("test2"), 1.0 / 60.0);
    }

    #[test]
    fn simple_event() {
        let (_clock, mut event_statistic) = setup();
        for _ in 0..100 {
            event_statistic.inc_event("test1");
        }
        assert_eq!(event_statistic.get_all_event_statistic(), HashMap::<String, f32>::from([
            ("test1".to_owned(), 100.0 / 60.0),
        ]));
        assert_eq!(event_statistic.get_event_statistic_by_name("test1"), 100.0 / 60.0);
    }

    #[test]
    fn expired_events() {
        let (clock, mut event_statistic) = setup();
        for _ in 0..100 {
            event_statistic.inc_event("test1");
        }
        clock.move_time(Duration::from_secs(60 * 60));

        assert_eq!(event_statistic.get_all_event_statistic(), HashMap::<String, f32>::from([
            ("test1".to_owned(), 0.0),
        ]));
        assert_eq!(event_statistic.get_event_statistic_by_name("test1"), 0.0);
    }

    #[test]
    fn expired_events_with_new() {
        let (clock, mut event_statistic) = setup();
        for _ in 0..100 {
            event_statistic.inc_event("test1");
        }
        clock.move_time(Duration::from_secs(60 * 60));
        for _ in 0..25 {
            event_statistic.inc_event("test1");
        }

        assert_eq!(event_statistic.get_all_event_statistic(), HashMap::<String, f32>::from([
            ("test1".to_owned(), 25.0 / 60.0),
        ]));
        assert_eq!(event_statistic.get_event_statistic_by_name("test1"), 25.0 / 60.0);
    }
}