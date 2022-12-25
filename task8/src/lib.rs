use std::collections::{HashMap, LinkedList};
use std::ops::Sub;
use std::rc::Rc;
use std::time::{Duration, Instant};

pub trait EventStatistic {
    fn inc_event(&mut self, name: &str);
    // mut for inner optimization (eg. drop expired events)
    fn get_event_statistic_by_name(&mut self, name: &str) -> f32;
    fn get_all_event_statistic(&mut self) -> HashMap<String, f32>;

    fn print_statistic(&mut self);
}

pub trait Clock {
    fn now(&self) -> Instant;
}

pub struct RealClock;

impl Clock for RealClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

pub struct LastHourEventRPMStatistic<T: Clock> {
    data: HashMap<String, LinkedList<Instant>>,
    clock: Rc<T>,
}

impl<T: Clock> LastHourEventRPMStatistic<T> {
    pub fn new(clock: Rc<T>) -> Self {
        Self { clock, data: HashMap::new() }
    }
}

fn clean_before_last_hour(list: &mut LinkedList<Instant>, now: Instant) {
    let border = now.sub(Duration::from_secs(60 * 60));
    while let Some(x) = list.front() {
        if x > &border {
            return;
        }
        list.pop_front();
    }
}

fn get_last_hour_rpm(list: &mut LinkedList<Instant>, now: Instant) -> f32 {
    clean_before_last_hour(list, now);
    (list.len() as f32) / 60.0
}

impl<T: Clock> EventStatistic for LastHourEventRPMStatistic<T> {
    fn inc_event(&mut self, name: &str) {
        let now = self.clock.now();
        match self.data.get_mut(name) {
            None => {
                let mut list = LinkedList::new();
                list.push_back(now);
                self.data.insert(name.to_owned(), list);
            }
            Some(x) => {
                clean_before_last_hour(x, now);
                x.push_back(now);
            }
        };
    }

    fn get_event_statistic_by_name(&mut self, name: &str) -> f32 {
        if let Some(list) = self.data.get_mut(name) {
            return get_last_hour_rpm(list, self.clock.now());
        }
        0.0
    }

    fn get_all_event_statistic(&mut self) -> HashMap<String, f32> {
        let mut result = HashMap::new();
        let now = self.clock.now();

        for (k, v) in self.data.iter_mut() {
            result.insert(k.clone(), get_last_hour_rpm(v, now));
        }

        result
    }

    fn print_statistic(&mut self) {
        for (k, v) in self.get_all_event_statistic() {
            println!("[{}] {:.2} RPM", k, v)
        }
    }
}
