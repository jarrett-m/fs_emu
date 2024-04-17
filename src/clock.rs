use crate::domain::Request;

pub struct Clock {
    time: u64,
    pub transfer_queue_to_node_1: Option<Request>,
    pub transfer_queue_to_node_2: Option<Request>,

    //data transfer back
    pub read_back_from_node_1: u64,
    pub read_back_from_node_2: u64,


}

impl Clock {
    pub fn new() -> Self {
        Clock { 
            time: 1,
            transfer_queue_to_node_1: None,
            transfer_queue_to_node_2: None,
            read_back_from_node_1: 0,
            read_back_from_node_2: 0,
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
    pub num_domains: u16,   // number of domains
    pub dead_time: u64,     // time between a vm's current request and the next request
}

impl Constraints {
    pub fn new(num_domains: u16, dead_time: u64) -> Self {
        Constraints {
            num_domains,
            dead_time,
        }
    }
}