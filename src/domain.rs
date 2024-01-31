#[derive(Clone)]
pub struct Domain {
    pub id: u16,
    pub write_queue: Vec<Request>,
    pub read_queue: Vec<Request>,
}

impl Domain {
    pub fn new(id: u16) -> Domain {
        Domain {
            id,
            write_queue: Vec::new(),
            read_queue: Vec::new(),
        }
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
    }
        //send nothing, pretend ;)

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
