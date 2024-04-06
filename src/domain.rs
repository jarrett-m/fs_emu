// use std::cmp::max;

use rand::Rng;
#[derive(Clone)]
pub struct Domain {
    pub id: u16,
    pub write_queue: Vec<Request>,
    pub read_queue: Vec<Request>,

    //profile stuff, not always used
    // pub write_tracker: Vec<char>,
    // pub pointer: usize,
    pub read_queue_node2: Vec<Request>,
    pub write_queue_node2: Vec<Request>,

    //numa stuff
    pub numa1_to_numa2: Vec<Request>,
    pub numa2_to_numa1: Vec<Request>,

    //stats
    pub fake_requests: u64,
    pub tick_finished: u64,
}

impl Domain {
    pub fn new(id: u16) -> Domain {
        Domain {
            id,
            write_queue: Vec::new(),
            read_queue: Vec::new(),
            // write_tracker: Vec::new(),
            // pointer: 0,
            fake_requests: 0,
            tick_finished: 0,
            read_queue_node2: Vec::new(),
            write_queue_node2: Vec::new(),
            numa1_to_numa2: Vec::new(),
            numa2_to_numa1: Vec::new(),
        }
    }

    // pub transfer_node_requests(&mut self) {
    //     //check all read and write queues

    //     //if read or write is from node 1 to node 2, move it to node 2
    //     //if read or write is from node 2 to node 1, move it to node 1
    //     //add a delay of 64 cycles Number of cycles required = (Transfer rate) * (Total time) = (10 * 10^9 cycles/second) * (6.4 * 10^-9 seconds) = 64 cycles

    //     //node 1 to node 2
    //     todo()!
        
    // }
    pub fn copy_write_to_transfer(&mut self, time: u64) {
        // put all pending writes in the transfer queue that are ready
        let mut t1: Vec<_> = self.write_queue.iter().filter(|x| x.cylce_in <= time).cloned().collect();
        // remove duplicates in t1 that are in self.numa1_to_numa2
        t1.retain(|x| !self.numa1_to_numa2.contains(x));
        //remove from write queue
        self.write_queue.retain(|x| !t1.contains(x));
        // add 64 cycles to the time
        for mut x in &mut t1 {
            x.cylce_in += 64;
        }
        self.numa1_to_numa2.extend(t1);
        // again

        let mut t2: Vec<_> = self.write_queue_node2.iter().filter(|x| x.cylce_in <= time).cloned().collect();
        t2.retain(|x| !self.numa2_to_numa1.contains(x));
        self.write_queue_node2.retain(|x| !t2.contains(x));
        for mut x in &mut t2 {
            x.cylce_in += 64;
        }
        self.numa2_to_numa1.extend(t2);
    }

    pub fn get_next_t_from_numa1(&mut self, time: u64) -> Option<Request> {
        if self.numa1_to_numa2.first().is_some() {
            if self.numa1_to_numa2.first().unwrap().cylce_in <= time {
                let r = self.numa1_to_numa2.remove(0);
                return Some(r)
            }
        }
        return None;
    }

    pub fn get_next_t_from_numa2(&mut self, time: u64) -> Option<Request> {
        if self.numa2_to_numa1.first().is_some() {
            if self.numa2_to_numa1.first().unwrap().cylce_in <= time {
                let r = self.numa2_to_numa1.remove(0);
                return Some(r)
            }
        }
        return None;
    }

    pub fn get_next_transfer_request_numa(&mut self, time: u64, transfer_to_node: u16) -> Option<Request> {
        if transfer_to_node == 2{
            for (index, req) in self.read_queue.iter().enumerate() {
                if req.cylce_in <= time {
                    if req.channel == 1 {
                        let mut request = self.read_queue.remove(index);
                        request.cylce_in = time + 128;
                        return Some(request);
                    }
                }
                else {
                    break;
                }
            }
            for (index, req) in self.write_queue.iter().enumerate() {
                if req.cylce_in <= time {
                    if req.channel == 1 {
                        let mut request = self.write_queue.remove(index);
                        request.cylce_in = time + 64;
                        return Some(request);
                    }
                }
                else {
                    break;
                }
            }
            return None;
        }
        if transfer_to_node == 1{
            for (index, req) in self.read_queue_node2.iter().enumerate() {
                if req.cylce_in <= time {
                    if req.channel == 0 {
                        let mut request = self.read_queue_node2.remove(index);
                        request.cylce_in = time + 128;
                        return Some(request);
                    }
                }
                else {
                    break;
                }
            }
            for (index, req) in self.write_queue_node2.iter().enumerate() {
                if req.cylce_in <= time {
                    if req.channel == 0 {
                        let mut request = self.write_queue_node2.remove(index);
                        request.cylce_in = time + 64;
                        return Some(request);
                    }
                }
                else {
                    break;
                }
            }
            return None;
        }
        return None;
    }

    pub fn split_requests_node_based(&mut self) {
        //move middle_bank_id to node 2
        self.read_queue_node2 = self.read_queue.iter().filter(|x| x.node == 0).cloned().collect();
        self.write_queue_node2 = self.write_queue.iter().filter(|x| x.node == 0).cloned().collect();

        //remove them from node 1
        self.read_queue = self.read_queue.iter().filter(|x| x.node == 1).cloned().collect();
        self.write_queue = self.write_queue.iter().filter(|x| x.node == 1).cloned().collect();
    }

    pub fn split_threads_evenly(&mut self) {
        //find largest thread id
        let largest_thread_id = self.write_queue.iter().chain(self.read_queue.iter()).map(|x| x.thread_id).max().unwrap();
        let lower_bound = largest_thread_id / 2;
        
        //move lower_bound threads to node 2
        self.read_queue_node2 = self.read_queue.iter().filter(|x| x.thread_id > lower_bound).cloned().collect();
        self.write_queue_node2 = self.write_queue.iter().filter(|x| x.thread_id > lower_bound).cloned().collect();

        //remove them from node 1
        self.read_queue = self.read_queue.iter().filter(|x| x.thread_id <= lower_bound).cloned().collect();
        self.write_queue = self.write_queue.iter().filter(|x| x.thread_id <= lower_bound).cloned().collect();
    }
    pub fn split_by_channel(&mut self){
        // self.read_queue_node2 = self.read_queue.iter().filter(|x| x.bank_id >= 4).cloned().collect();
        // self.write_queue_node2 = self.write_queue.iter().filter(|x| x.bank_id >= 4).cloned().collect();

        //remove them from node 1
        // self.read_queue = self.read_queue.iter().filter(|x| x.bank_id < 4).cloned().collect();
        // self.write_queue = self.write_queue.iter().filter(|x| x.bank_id < 4).cloned().collect();

        //split by about 60% with rand
        let mut rng = rand::thread_rng();
        let mut read_queue_node2 = Vec::new();
        let mut write_queue_node2 = Vec::new();
        for req in self.read_queue.iter() {
            let rand: f64 = rng.gen();
            if rand > 0.70 {
                read_queue_node2.push(req.clone());
            }
        }
        for req in self.write_queue.iter() {
            let rand: f64 = rng.gen();
            if rand > 0.70 {
                write_queue_node2.push(req.clone());
            }
        }

                //remove them from node 1
        self.read_queue = self.read_queue.iter().filter(|x| !read_queue_node2.contains(x)).cloned().collect();
        self.write_queue = self.write_queue.iter().filter(|x| !write_queue_node2.contains(x)).cloned().collect();



        self.read_queue_node2 = read_queue_node2;
        self.write_queue_node2 = write_queue_node2;


       
    
    }

    pub fn split_evenly(&mut self) {
        
        //split in half every other request
        self.read_queue_node2 = self.read_queue.iter().enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, x)| x.clone()).collect();
        self.write_queue_node2 = self.write_queue.iter().enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, x)| x.clone()).collect();

        //remove them from node 1
        self.read_queue = self.read_queue.iter().enumerate().filter(|(i, _)| i % 2 == 1).map(|(_, x)| x.clone()).collect();
        self.write_queue = self.write_queue.iter().enumerate().filter(|(i, _)| i % 2 == 1).map(|(_, x)| x.clone()).collect();
    }

    // pub fn set_write_tracker(&mut self, odds: u8)  {
    //     //writes 
    //     let write_count = max(odds, 1);
    //     let read_count = max(100 - write_count, 1);


    //     //w_r = for every x writes, there are y reads
    //     let w_r;
    //     let mut write_tracker: Vec<char>;
    //     if write_count > read_count {
    //         w_r = write_count / read_count;
    //         write_tracker = vec!['w'; w_r as usize];
    //         write_tracker.push('r');
    //     } else {
    //         w_r = read_count / write_count;
    //         write_tracker = vec!['r'; w_r as usize];
    //         write_tracker.push('w');
    //     }

    //     self.write_tracker = write_tracker;
    //     //println!("write_tracker: {:?}", self.write_tracker);
    // }

    pub fn add_write_request(&mut self, request: Request) {
        self.write_queue.push(request);
    }

    pub fn add_read_request(&mut self, request: Request) {
        self.read_queue.push(request);
    }

    pub fn send_next_request_bank_mirror(&mut self, time: u64, bank_id_allowed: u16){
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
                let req = self.write_queue.remove(next_write_with_bank_id_index.unwrap());
                self.read_queue_node2.push(req);
            }
        }
        //if only read, send it
        else if next_read_with_bank_id.is_some() {
            self.read_queue.remove(next_read_with_bank_id_index.unwrap());
        }
        //if only write, send it 
        else if next_write_with_bank_id.is_some(){
            let req = self.write_queue.remove(next_write_with_bank_id_index.unwrap());
            self.read_queue_node2.push(req);
        }
        else {
            self.fake_requests += 1;
        }

        //----------------- then again for node 2 --------------------
        let mut next_read_with_bank_id_index = None;
        for (index, read) in self.read_queue_node2.iter().enumerate() {
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
        for (index, write) in self.write_queue_node2.iter().enumerate() {
            if write.cylce_in > time {
                break;
            }
            if write.bank_id % 3 == bank_id_allowed {
                next_write_with_bank_id_index = Some(index);
                break;
            }
        }

        let next_read_with_bank_id = match next_read_with_bank_id_index {
            Some(index) => Some(self.read_queue_node2[index].clone()),
            None => None,
        };

        let next_write_with_bank_id= match next_write_with_bank_id_index {
            Some(index) => Some(self.write_queue_node2[index].clone()),
            None => None,
        };

        //if both are None, we have a fake request
        if !next_read_with_bank_id.is_some() && !next_write_with_bank_id.is_some(){
            self.fake_requests += 1;
        }

        //if both are Some, send the oldest request
        if next_read_with_bank_id.is_some() && next_write_with_bank_id.is_some() {
            if next_read_with_bank_id.clone().unwrap().cylce_in <= next_write_with_bank_id.clone().unwrap().cylce_in {
                self.read_queue_node2.remove(next_read_with_bank_id_index.unwrap());
            } else {
                let req = self.write_queue_node2.remove(next_write_with_bank_id_index.unwrap());
                self.read_queue.push(req);
            }
        }
        //if only read, send it
        else if next_read_with_bank_id.is_some() {
            self.read_queue_node2.remove(next_read_with_bank_id_index.unwrap());
        }
        //if only write, send it 
        else if next_write_with_bank_id.is_some(){
            let req = self.write_queue_node2.remove(next_write_with_bank_id_index.unwrap());
            self.read_queue.push(req);
        }
        else {
            self.fake_requests += 1;
        }
        
    }

    pub fn send_next_request_channel(&mut self, time: u64, bank_id_allowed: u16){
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

        //----------------- then again for node 2 --------------------
        let mut next_read_with_bank_id_index = None;
        for (index, read) in self.read_queue_node2.iter().enumerate() {
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
        for (index, write) in self.write_queue_node2.iter().enumerate() {
            if write.cylce_in > time {
                break;
            }
            if write.bank_id % 3 == bank_id_allowed {
                next_write_with_bank_id_index = Some(index);
                break;
            }
        }

        let next_read_with_bank_id = match next_read_with_bank_id_index {
            Some(index) => Some(self.read_queue_node2[index].clone()),
            None => None,
        };

        let next_write_with_bank_id= match next_write_with_bank_id_index {
            Some(index) => Some(self.write_queue_node2[index].clone()),
            None => None,
        };

        //if both are None, we have a fake request
        if !next_read_with_bank_id.is_some() && !next_write_with_bank_id.is_some(){
            self.fake_requests += 1;
        }

        //if both are Some, send the oldest request
        if next_read_with_bank_id.is_some() && next_write_with_bank_id.is_some() {
            if next_read_with_bank_id.clone().unwrap().cylce_in <= next_write_with_bank_id.clone().unwrap().cylce_in {
                self.read_queue_node2.remove(next_read_with_bank_id_index.unwrap());
            } else {
                self.write_queue_node2.remove(next_write_with_bank_id_index.unwrap());
            }
        }
        //if only read, send it
        else if next_read_with_bank_id.is_some() {
            self.read_queue_node2.remove(next_read_with_bank_id_index.unwrap());
        }
        //if only write, send it 
        else if next_write_with_bank_id.is_some(){
            self.write_queue_node2.remove(next_write_with_bank_id_index.unwrap());
        }
        else {
            self.fake_requests += 1;
        }
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

    // pub fn send_next_write_request_bta(&mut self, time: u64, bank_id_allowed: u16){
    //     let mut next_write_with_bank_id_index = None;
    //     for (index, write) in self.write_queue.iter().enumerate() {
    //         if write.cylce_in > time {
    //             break;
    //         }
    //         if write.bank_id % 3 == bank_id_allowed {
    //             next_write_with_bank_id_index = Some(index);
    //             break;
    //         }
    //     }
    //     let next_write_with_bank_id= match next_write_with_bank_id_index {
    //         Some(index) => Some(self.write_queue[index].clone()),
    //         None => None,
    //     };

    //     if next_write_with_bank_id.is_some() && next_write_with_bank_id.unwrap().cylce_in <= time {
    //         self.write_queue.remove(next_write_with_bank_id_index.unwrap());
    //     } else {
    //         self.fake_requests += 1;
    //     }

    // }

    // pub fn send_next_read_request_bta(&mut self, time: u64, bank_id_allowed: u16){
    //     let mut next_read_with_bank_id_index = None;
    //     for (index, read) in self.read_queue.iter().enumerate() {
    //         if read.cylce_in > time {
    //             break;
    //         }
    //         if read.bank_id % 3 == bank_id_allowed {
    //             next_read_with_bank_id_index = Some(index);
    //             break;
    //         }
    //     }
    //     let next_read_with_bank_id= match next_read_with_bank_id_index {
    //         Some(index) => Some(self.read_queue[index].clone()),
    //         None => None,
    //     };

    //     if next_read_with_bank_id.is_some() && next_read_with_bank_id.unwrap().cylce_in <= time {
    //         self.read_queue.remove(next_read_with_bank_id_index.unwrap());
    //     } else {
    //         self.fake_requests += 1;
    //     }
    // }
    
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


    pub fn send_next_request_bank_numa(&mut self, time: u64, bank_id_allowed: u16){
        //if next request is before time, send it
        //get oldest sent request for the req.bank_id % 3 = bank_id_allowed from read
        // remove it if time allows
        //get next apprioriate read
        let mut next_read_with_bank_id_index = None;
        for (index, read) in self.read_queue.iter().enumerate() {
            if read.cylce_in >= time {
                break;
            }
            if read.channel != 0 {
                continue;
            }
            if read.bank_id % 3 == bank_id_allowed {
                next_read_with_bank_id_index = Some(index);
                break;
            }
        }

        //get next apprioriate write
        let mut next_write_with_bank_id_index = None;
        for (index, write) in self.write_queue.iter().enumerate() {
            if write.cylce_in >= time {
                break;
            }
            if write.channel != 0 {
                continue;
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

        //----------------- then again for node 2 --------------------
        let mut next_read_with_bank_id_index = None;
        for (index, read) in self.read_queue_node2.iter().enumerate() {
            if read.cylce_in >= time {
                break;
            }
            if read.channel != 1 {
                continue;
            }
            if read.bank_id % 3 == bank_id_allowed {
                next_read_with_bank_id_index = Some(index);
                break;
            }
        }

        //get next apprioriate write
        let mut next_write_with_bank_id_index = None;
        for (index, write) in self.write_queue_node2.iter().enumerate() {
            if write.cylce_in >= time {
                break;
            }
            if write.channel != 1 {
                continue;
            }
            if write.bank_id % 3 == bank_id_allowed {
                next_write_with_bank_id_index = Some(index);
                break;
            }
        }

        let next_read_with_bank_id = match next_read_with_bank_id_index {
            Some(index) => Some(self.read_queue_node2[index].clone()),
            None => None,
        };

        let next_write_with_bank_id= match next_write_with_bank_id_index {
            Some(index) => Some(self.write_queue_node2[index].clone()),
            None => None,
        };

        //if both are None, we have a fake request
        if !next_read_with_bank_id.is_some() && !next_write_with_bank_id.is_some(){
            self.fake_requests += 1;
        }

        //if both are Some, send the oldest request
        if next_read_with_bank_id.is_some() && next_write_with_bank_id.is_some() {
            if next_read_with_bank_id.clone().unwrap().cylce_in <= next_write_with_bank_id.clone().unwrap().cylce_in {
                self.read_queue_node2.remove(next_read_with_bank_id_index.unwrap());
            } else {
                self.write_queue_node2.remove(next_write_with_bank_id_index.unwrap());
            }
        }
        //if only read, send it
        else if next_read_with_bank_id.is_some() {
            self.read_queue_node2.remove(next_read_with_bank_id_index.unwrap());
        }
        //if only write, send it 
        else if next_write_with_bank_id.is_some(){
            self.write_queue_node2.remove(next_write_with_bank_id_index.unwrap());
        }
        else {
            self.fake_requests += 1;
        }
        
    }

    pub fn send_next_request_bank_dve(&mut self, time: u64, bank_id_allowed: u16){
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

        //----------------- then again for node 2 --------------------
        let mut next_read_with_bank_id_index = None;
        for (index, read) in self.read_queue_node2.iter().enumerate() {
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
        for (index, write) in self.write_queue_node2.iter().enumerate() {
            if write.cylce_in > time {
                break;
            }
            if write.bank_id % 3 == bank_id_allowed {
                next_write_with_bank_id_index = Some(index);
                break;
            }
        }

        let next_read_with_bank_id = match next_read_with_bank_id_index {
            Some(index) => Some(self.read_queue_node2[index].clone()),
            None => None,
        };

        let next_write_with_bank_id= match next_write_with_bank_id_index {
            Some(index) => Some(self.write_queue_node2[index].clone()),
            None => None,
        };

        //if both are None, we have a fake request
        if !next_read_with_bank_id.is_some() && !next_write_with_bank_id.is_some(){
            self.fake_requests += 1;
        }

        //if both are Some, send the oldest request
        if next_read_with_bank_id.is_some() && next_write_with_bank_id.is_some() {
            if next_read_with_bank_id.clone().unwrap().cylce_in <= next_write_with_bank_id.clone().unwrap().cylce_in {
                self.read_queue_node2.remove(next_read_with_bank_id_index.unwrap());
            } else {
                self.write_queue_node2.remove(next_write_with_bank_id_index.unwrap());
            }
        }
        //if only read, send it
        else if next_read_with_bank_id.is_some() {
            self.read_queue_node2.remove(next_read_with_bank_id_index.unwrap());
        }
        //if only write, send it 
        else if next_write_with_bank_id.is_some(){
            self.write_queue_node2.remove(next_write_with_bank_id_index.unwrap());
        }
        else {
            self.fake_requests += 1;
        }

    }

    // pub fn send_next_read_request(&mut self, time: u64) {
    //     if self.read_queue.first().is_some() && self.read_queue.first().unwrap().cylce_in <= time {
    //         self.read_queue.remove(0);
    //     } else {
    //         self.fake_requests += 1;
    //     }
    //     self.pointer = (self.pointer + 1) % self.write_tracker.len(); //move to next read or write
    // }

    // pub fn send_next_write_request(&mut self, time: u64) {
    //     if self.write_queue.first().is_some() && self.write_queue.first().unwrap().cylce_in <= time {
    //         self.write_queue.remove(0);
    //     } else {
    //         self.fake_requests += 1;
    //     }
    //     self.pointer = (self.pointer + 1) % self.write_tracker.len(); //move to next read or write
    // }

    // pub fn can_write(&mut self) -> bool {
    //     self.write_tracker[self.pointer] == 'w'
    // }

    // pub fn can_read(&mut self) -> bool {
    //     self.write_tracker[self.pointer] == 'r'
    // }

    // pub fn send_next_request_odds(&mut self, time: u64) {
    //     if self.write_tracker[self.pointer] == 'w'{ //if next is a write
    //         if self.write_queue.first().is_some() && self.write_queue.first().unwrap().cylce_in <= time {
    //             self.write_queue.remove(0); //send next write
    //         }
    //     } 
    //     //if next is a read
    //     else if self.read_queue.first().is_some() && self.read_queue.first().unwrap().cylce_in <= time { //else its read, priority to read
    //         self.read_queue.remove(0); //else we can only send a read
    //     }
    //     else if self.write_queue.first().is_some() && self.write_queue.first().unwrap().cylce_in <= time {
    //         self.write_queue.remove(0); //send next write
    //     }
    //     //send nothing, pretend ;)
    //     self.pointer = (self.pointer + 1) % self.write_tracker.len(); //move to next read or write
    // }

    // pub fn is_write(&self) -> bool {
    //     self.write_tracker[self.pointer] == 'w'
    // }



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
    pub thread_id: u16,
    pub node: u16,
    pub channel: u16,
    pub domain: u16,
}

impl Request {
    // pub fn new(request_type: RequestType, cylce_in: u64) -> Request {
    //     Request { request_type, cylce_in}
    // }

    pub fn new(request_type: RequestType, cylce_in: u64, bank_id: u16, thread_id: u16, node: u16, channel: u16, domain: u16) -> Request {
        Request {request_type, cylce_in, bank_id, thread_id, node, channel, domain}
    }
}
