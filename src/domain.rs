use std::cmp::max;


#[derive(Clone)]
pub struct Domain {
    pub id: u16,
    pub write_queue: Vec<Request>,
    pub read_queue: Vec<Request>,

    //profile stuff, not always used
    pub write_tracker: Vec<char>,
    pub pointer: usize,
    pub fake_requests: u64,
    
}

impl Domain {
    pub fn new(id: u16) -> Domain {
        Domain {
            id,
            write_queue: Vec::new(),
            read_queue: Vec::new(),
            write_tracker: Vec::new(),
            pointer: 0,
            fake_requests: 0,
        }
    }

    pub fn set_write_tracker(&mut self, odds: u8)  {
        //writes 
        let write_count = max(odds, 1);
        let read_count = max(100 - write_count, 1);


        //w_r = for every x writes, there are y reads
        let w_r;
        let mut write_tracker: Vec<char>;
        if write_count > read_count {
            w_r = write_count / read_count;
            write_tracker = vec!['w'; w_r as usize];
            write_tracker.push('r');
        } else {
            w_r = read_count / write_count;
            write_tracker = vec!['r'; w_r as usize];
            write_tracker.push('w');
        }

        self.write_tracker = write_tracker;
        //println!("write_tracker: {:?}", self.write_tracker);
    }

    pub fn add_write_request(&mut self, request: Request) {
        self.write_queue.push(request);
    }

    pub fn add_read_request(&mut self, request: Request) {
        self.read_queue.push(request);
    }

    pub fn send_next_request(&mut self, time: u64) {
        //if next request is before time, send it
        if self.read_queue.last().is_some() && self.write_queue.last().is_some() {
            if self.read_queue.last().unwrap().cylce_in < self.write_queue.last().unwrap().cylce_in {
                if self.read_queue.last().unwrap().cylce_in <= time {
                    self.read_queue.pop();
                }
            } else if self.write_queue.last().unwrap().cylce_in <= time {
                self.write_queue.pop();
            }
        } else if self.read_queue.last().is_some() {
            if self.read_queue.last().unwrap().cylce_in <= time {
                self.read_queue.pop();
            }
        } else if self.write_queue.last().is_some() && self.write_queue.last().unwrap().cylce_in <= time {
            self.write_queue.pop();
        }
        else {
            self.fake_requests += 1;
        }
        //send nothing, pretend ;)
    }

    pub fn send_next_read_request(&mut self, time: u64) {
        if self.read_queue.last().is_some() && self.read_queue.last().unwrap().cylce_in <= time {
            self.read_queue.pop();
        } else {
            self.fake_requests += 1;
        }
        self.pointer = (self.pointer + 1) % self.write_tracker.len(); //move to next read or write
    }

    pub fn send_next_write_request(&mut self, time: u64) {
        if self.write_queue.last().is_some() && self.write_queue.last().unwrap().cylce_in <= time {
            self.write_queue.pop();
        } else {
            self.fake_requests += 1;
        }
        self.pointer = (self.pointer + 1) % self.write_tracker.len(); //move to next read or write
    }

    pub fn can_write(&mut self) -> bool {
        self.write_tracker[self.pointer] == 'w'
    }

    pub fn can_read(&mut self) -> bool {
        self.write_tracker[self.pointer] == 'r'
    }

    pub fn send_next_request_odds(&mut self, time: u64) {
        if self.write_tracker[self.pointer] == 'w'{ //if next is a write
            if self.write_queue.last().is_some() && self.write_queue.last().unwrap().cylce_in <= time {
                self.write_queue.pop(); //send next write
            }
        } 
        //if next is a read
        else if self.read_queue.last().is_some() && self.read_queue.last().unwrap().cylce_in <= time { //else its read, priority to read
            self.read_queue.pop(); //else we can only send a read
        }
        else if self.write_queue.last().is_some() && self.write_queue.last().unwrap().cylce_in <= time {
            self.write_queue.pop(); //send next write
        }
        //send nothing, pretend ;)
        self.pointer = (self.pointer + 1) % self.write_tracker.len(); //move to next read or write
    }

    pub fn is_write(&self) -> bool {
        self.write_tracker[self.pointer] == 'w'
    }



}

#[derive(Debug)]
#[derive(Clone)]
pub enum RequestType {
    WriteRequest,
    ReadRequest,
}
#[derive(Clone)]
pub struct Request {
    pub request_type: RequestType,
    pub cylce_in: u64,
}

impl Request {
    pub fn new(request_type: RequestType, cylce_in: u64) -> Request {
        Request { request_type, cylce_in}
    }
}
