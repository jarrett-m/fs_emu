pub struct Clock {
    time: u64,
}

impl Clock {
    pub fn new() -> Self {
        Clock { time: 0 }
    }

    // pub fn tick(&mut self) {
    //     self.time += 1;
    // }

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