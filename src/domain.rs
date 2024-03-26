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
    
    pub tick_finished: u64,
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
            tick_finished: 0,
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
        if self.read_queue.first().is_some() && self.write_queue.first().is_some() {
            if self.read_queue.first().unwrap().cylce_in <= self.write_queue.first().unwrap().cylce_in && self.read_queue.first().unwrap().cylce_in <= time{
                self.read_queue.remove(0);
                return;
            } else if self.write_queue.first().unwrap().cylce_in <= time {
                self.write_queue.remove(0);
                return;
            }
        } else if self.read_queue.first().is_some() && self.read_queue.first().unwrap().cylce_in <= time {
            self.read_queue.remove(0);
            return;
        } else if self.write_queue.first().is_some() && self.write_queue.first().unwrap().cylce_in <= time {
            self.write_queue.remove(0);
            return;
        }

        self.fake_requests += 1;
        //send nothing, pretend ;)
    }

    pub fn send_next_write_request_bta(&mut self, time: u64, bank_id_allowed: u16){
        let mut next_write_with_bank_id_index = None;
        for (index, write) in self.write_queue.iter().enumerate() {
            if write.cylce_in > time {
                break;
            }
            if write.bank_id % 3 == bank_id_allowed {
                next_write_with_bank_id_index = Some(index);
                break;
            }
        }
        let next_write_with_bank_id= match next_write_with_bank_id_index {
            Some(index) => Some(self.write_queue[index].clone()),
            None => None,
        };

        if next_write_with_bank_id.is_some() && next_write_with_bank_id.unwrap().cylce_in <= time {
            self.write_queue.remove(next_write_with_bank_id_index.unwrap());
        } else {
            self.fake_requests += 1;
        }

    }

    pub fn send_next_read_request_bta(&mut self, time: u64, bank_id_allowed: u16){
        let mut next_read_with_bank_id_index = None;
        for (index, read) in self.read_queue.iter().enumerate() {
            if read.cylce_in > time {
                break;
            }
            if read.bank_id % 3 == bank_id_allowed {
                next_read_with_bank_id_index = Some(index);
                break;
            }
        }
        let next_read_with_bank_id= match next_read_with_bank_id_index {
            Some(index) => Some(self.read_queue[index].clone()),
            None => None,
        };

        if next_read_with_bank_id.is_some() && next_read_with_bank_id.unwrap().cylce_in <= time {
            self.read_queue.remove(next_read_with_bank_id_index.unwrap());
        } else {
            self.fake_requests += 1;
        }

    }
    
    pub fn send_next_request_bank(&mut self, time: u64, bank_id_allowed: u16){
        //if next request is before time, send it

        //get oldest sent request for the req.bank_id % 3 = bank_id_allowed from read
        // remove it if time allows


        //get next apprioriate read
        let mut next_read_with_bank_id_index = None;
        for (index, read) in self.read_queue.iter().enumerate() {
            if read.cylce_in > time {
                break;
            }
            if read.bank_id % 3 == bank_id_allowed {
                next_read_with_bank_id_index = Some(index);
                break;
            }
        }

        //get next apprioriate write
        let mut next_write_with_bank_id_index = None;
        for (index, write) in self.write_queue.iter().enumerate() {
            if write.cylce_in > time {
                break;
            }
            if write.bank_id % 3 == bank_id_allowed {
                next_write_with_bank_id_index = Some(index);
                break;
            }
        }


        let next_read_with_bank_id = match next_read_with_bank_id_index {
            Some(index) => Some(self.read_queue[index].clone()),
            None => None,
        };

        let next_write_with_bank_id= match next_write_with_bank_id_index {
            Some(index) => Some(self.write_queue[index].clone()),
            None => None,
        };

        //if both are None, we have a fake request
        if !next_read_with_bank_id.is_some() && !next_write_with_bank_id.is_some(){
            self.fake_requests += 1;
        }

        //if both are Some, send the oldest request
        if next_read_with_bank_id.is_some() && next_write_with_bank_id.is_some() {
            if next_read_with_bank_id.clone().unwrap().cylce_in <= next_write_with_bank_id.clone().unwrap().cylce_in {
                self.read_queue.remove(next_read_with_bank_id_index.unwrap());
            } else {
                self.write_queue.remove(next_write_with_bank_id_index.unwrap());
            }
        }
        //if only read, send it
        else if next_read_with_bank_id.is_some() {
            self.read_queue.remove(next_read_with_bank_id_index.unwrap());
        }
        //if only write, send it 
        else if next_write_with_bank_id.is_some(){
            self.write_queue.remove(next_write_with_bank_id_index.unwrap());
        }
        else {
            self.fake_requests += 1;
        }
        
    }

    pub fn send_next_read_request(&mut self, time: u64) {
        if self.read_queue.first().is_some() && self.read_queue.first().unwrap().cylce_in <= time {
            self.read_queue.remove(0);
        } else {
            self.fake_requests += 1;
        }
        self.pointer = (self.pointer + 1) % self.write_tracker.len(); //move to next read or write
    }

    pub fn send_next_write_request(&mut self, time: u64) {
        if self.write_queue.first().is_some() && self.write_queue.first().unwrap().cylce_in <= time {
            self.write_queue.remove(0);
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
            if self.write_queue.first().is_some() && self.write_queue.first().unwrap().cylce_in <= time {
                self.write_queue.remove(0); //send next write
            }
        } 
        //if next is a read
        else if self.read_queue.first().is_some() && self.read_queue.first().unwrap().cylce_in <= time { //else its read, priority to read
            self.read_queue.remove(0); //else we can only send a read
        }
        else if self.write_queue.first().is_some() && self.write_queue.first().unwrap().cylce_in <= time {
            self.write_queue.remove(0); //send next write
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
#[derive(PartialEq)]
pub enum RequestType {
    WriteRequest,
    ReadRequest,
}
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Request {
    pub request_type: RequestType,
    pub cylce_in: u64,
    pub bank_id: u16,
}

impl Request {
    // pub fn new(request_type: RequestType, cylce_in: u64) -> Request {
    //     Request { request_type, cylce_in}
    // }

    pub fn new(request_type: RequestType, cylce_in: u64, bank_id: u16) -> Request {
        Request { request_type, cylce_in, bank_id}
    }
}
