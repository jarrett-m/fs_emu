use crate::domain::Request;

pub struct Clock {
    time: u64,
    pub transfer_queue_to_node_1: Option<Request>,
    pub transfer_queue_to_node_2: Option<Request>,
}

impl Clock {
    pub fn new() -> Self {
        Clock { 
            time: 0,
            transfer_queue_to_node_1: None,
            transfer_queue_to_node_2: None,
        }
    }

    pub fn tick_by(&mut self, ticks: u64) {
        self.time += ticks;
    }

    pub fn time(&self) -> u64 {
        self.time
    }
}

pub struct Constraints {
    pub inject_time: u16,   // time between a vm's current request and the next request
    pub num_domains: u16,   // number of domains
    pub dead_time: u16,     // time between a vm's current request and the next request
}

impl Constraints {
    pub fn new(num_domains: u16, dead_time: u16) -> Self {
        Constraints {
            num_domains,
            dead_time,
            inject_time: dead_time * num_domains
        }
    }
}