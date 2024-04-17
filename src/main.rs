//Import all the modules
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread;
use crate::domain::INIT_REQUEST_NODE_DELAY;
use crate::domain::DATA_RETRIVAL_NODE_DELAY;

mod clock;
mod domain;


fn simulate_fs_rankp(domains: &mut Vec<domain::Domain>) -> (u64, Vec<domain::Domain>) {
    let mut clock = clock::Clock::new();
    let constraints = clock::Constraints::new(domains.len() as u16, 7);
    let mut current_domain: u16 = 0;
    while domains.iter().any(|domain| !domain.read_queue.is_empty()) || domains.iter().any(|domain| !domain.write_queue.is_empty()) {
        //check which request (next read or write) was sent first, and send it
        domains[current_domain as usize].send_next_request(clock.time());
        clock.tick_by(constraints.dead_time as u64); //skip to next dead time

         //if there are no more request, set the program finish time
         if domains[current_domain as usize].read_queue.is_empty() && domains[current_domain as usize].write_queue.is_empty()  && domains[current_domain as usize].tick_finished == 0{
            domains[current_domain as usize].tick_finished = clock.time();
        }

        current_domain = (current_domain + 1) % constraints.num_domains;
    }
    (clock.time(), domains.clone())
}

fn simulate_fs_bta(domains: &mut Vec<domain::Domain>) -> (u64, Vec<domain::Domain>){
    let mut clock = clock::Clock::new();
    let constraints = clock::Constraints::new(domains.len() as u16, 15);
    let mut current_domain: u16 = 0;
    let mut current_bank_id: u16 = 0;

    while domains.iter().any(|domain| !domain.read_queue.is_empty()) || 
        domains.iter().any(|domain| !domain.write_queue.is_empty() || 
        domains.iter().any(|domain| !domain.write_queue_node2.is_empty()) ||
        domains.iter().any(|domain| !domain.read_queue_node2.is_empty()))
        {
    

        //Send the next request with allowed bank
        domains[current_domain as usize].send_next_request_bank(clock.time(), current_bank_id);
        clock.tick_by(constraints.dead_time as u64); //skip to next dead time

        //if there are no more request, set the program finish time
        if domains[current_domain as usize].read_queue.is_empty() && domains[current_domain as usize].write_queue.is_empty()  && domains[current_domain as usize].tick_finished == 0{
            domains[current_domain as usize].tick_finished = clock.time();
    }
        current_domain = (current_domain + 1) % constraints.num_domains;
        current_bank_id = (current_bank_id + 1) % 3;
    }
    (clock.time(), domains.clone())
}

fn simulate_base_two_channels(domains: &mut Vec<domain::Domain>) -> (u64, Vec<domain::Domain>){
    let mut clock = clock::Clock::new();
    let constraints = clock::Constraints::new(domains.len() as u16, 15);
    let mut current_domain: u16 = 0;
    let mut current_bank_id: u16 = 0;

    //for each domain, split the threads evenly between qeueu and queue_node2
    for domain in domains.iter_mut() {
        domain.split_by_channel();
    }

    while domains.iter().any(|domain| !domain.read_queue.is_empty()) || 
          domains.iter().any(|domain| !domain.write_queue.is_empty() || 
          domains.iter().any(|domain| !domain.write_queue_node2.is_empty()) ||
          domains.iter().any(|domain| !domain.read_queue_node2.is_empty())) {
        //Send the next request with allowed bank
        domains[current_domain as usize].send_next_request_channel(clock.time(), current_bank_id);
        clock.tick_by(constraints.dead_time as u64); //skip to next dead time

        //if there are no more request, set the program finish time
        //if there are no more request, set the program finish time
        if  domains[current_domain as usize].read_queue.is_empty() && 
        domains[current_domain as usize].write_queue.is_empty() && 
        domains[current_domain as usize].write_queue_node2.is_empty() && 
        domains[current_domain as usize].read_queue_node2.is_empty() && 
        domains[current_domain as usize].tick_finished == 0
        {
            domains[current_domain as usize].tick_finished = clock.time();
        }
        current_domain = (current_domain + 1) % constraints.num_domains;
        current_bank_id = (current_bank_id + 1) % 3;
    }
    (clock.time(), domains.clone())
}

fn simulate_fs_mirrior_channels(domains: &mut Vec<domain::Domain>) -> (u64, Vec<domain::Domain>){
    let mut clock = clock::Clock::new();
    let constraints = clock::Constraints::new(domains.len() as u16, 15);
    let mut current_domain: u16 = 0;
    let mut current_bank_id: u16 = 0;

    //for each domain, split the threads evenly between qeueu and queue_node2
    for domain in domains.iter_mut() {
        domain.split_evenly();
    }

    while domains.iter().any(|domain| !domain.read_queue.is_empty()) || 
        domains.iter().any(|domain| !domain.write_queue.is_empty() || 
        domains.iter().any(|domain| !domain.write_queue_node2.is_empty()) ||
        domains.iter().any(|domain| !domain.read_queue_node2.is_empty())) {
        //Send the next request with allowed bank
        domains[current_domain as usize].send_next_request_bank_mirror(clock.time(), current_bank_id);
        clock.tick_by(constraints.dead_time as u64); //skip to next dead time

        //if there are no more request, set the program finish time
        if  domains[current_domain as usize].read_queue.is_empty() && 
        domains[current_domain as usize].write_queue.is_empty() && 
        domains[current_domain as usize].write_queue_node2.is_empty() && 
        domains[current_domain as usize].read_queue_node2.is_empty() && 
        domains[current_domain as usize].tick_finished == 0
        {
            domains[current_domain as usize].tick_finished = clock.time();
        }
        current_domain = (current_domain + 1) % constraints.num_domains;
        current_bank_id = (current_bank_id + 1) % 3;
    }
    (clock.time(), domains.clone())
}
fn simulate_fs_base_numa(domains: &mut Vec<domain::Domain>) -> (u64, Vec<domain::Domain>){
    let mut clock = clock::Clock::new();
    let constraints = clock::Constraints::new(domains.len() as u16, 15);
    let mut current_domain: u16 = 0;
    let mut current_bank_id: u16 = 0;

    //for each domain, split the threads evenly between qeueu and queue_node2
    for domain in domains.iter_mut() {
        domain.split_threads_evenly();
    }

    let mut domain_transfer_timer = 0;
    let mut allowed_domain_for_node_transfer = 0;

    while domains.iter().any(|domain| !domain.read_queue.is_empty()) || 
        domains.iter().any(|domain| !domain.write_queue.is_empty() || 
        domains.iter().any(|domain| !domain.write_queue_node2.is_empty()) ||
        domains.iter().any(|domain| !domain.read_queue_node2.is_empty())) ||
        clock.transfer_queue_to_node_1.clone() != None || clock.transfer_queue_to_node_2.clone() != None || 
        domains.iter().any(|domain| !domain.numa1_to_numa2.is_empty()) ||
        domains.iter().any(|domain| !domain.numa2_to_numa1.is_empty()) ||
        clock.read_back_from_node_1 != 0 || clock.read_back_from_node_2 != 0
        {

        if domain_transfer_timer == 0{
            //transfer requests from node 1 to node 2
            if clock.read_back_from_node_2 != 0 {
                clock.read_back_from_node_2 -= DATA_RETRIVAL_NODE_DELAY;
            }         
            else {
                clock.transfer_queue_to_node_2 = domains[allowed_domain_for_node_transfer as usize].get_next_transfer_request_numa(&mut clock, 2);
            }
            
            //transfer requests from node 2 to node 1
            if clock.read_back_from_node_1 != 0 {
                clock.read_back_from_node_1 -= DATA_RETRIVAL_NODE_DELAY;
            }   
            else {
                clock.transfer_queue_to_node_1 = domains[allowed_domain_for_node_transfer as usize].get_next_transfer_request_numa(&mut clock, 1);
            }
            domain_transfer_timer = DATA_RETRIVAL_NODE_DELAY;
            allowed_domain_for_node_transfer = (allowed_domain_for_node_transfer + 1) % constraints.num_domains;
        }
        else {
            domain_transfer_timer -= 1;
        }
    
        //remove the request from the transfer queue
        if clock.transfer_queue_to_node_1 != None {
            let request = clock.transfer_queue_to_node_1.unwrap();
            clock.transfer_queue_to_node_1 = None;
            if request.request_type == domain::RequestType::ReadRequest{
                domains[request.domain as usize].data_transfers += DATA_RETRIVAL_NODE_DELAY;
                domains[request.domain as usize].read_queue.push(request);
                clock.read_back_from_node_1 += DATA_RETRIVAL_NODE_DELAY;
            }
            else if request.request_type == domain::RequestType::WriteRequest{
                domains[request.domain  as usize].write_queue.push(request);
            }
        }
    
        if clock.transfer_queue_to_node_2 != None {
            let request = clock.transfer_queue_to_node_2.unwrap();
            clock.transfer_queue_to_node_2 = None;
            if request.request_type == domain::RequestType::ReadRequest{
                domains[request.domain as usize].data_transfers += DATA_RETRIVAL_NODE_DELAY;
                domains[request.domain as usize].read_queue_node2.push(request);
                clock.read_back_from_node_2 += DATA_RETRIVAL_NODE_DELAY;
            }
            else if request.request_type == domain::RequestType::WriteRequest{
                domains[request.domain as usize].write_queue_node2.push(request);
            }
        
        }

        if clock.time() % constraints.dead_time  != 0{
            clock.tick_by(1);
            continue;
        }
        //Send the next request with allowed bank
        domains[current_domain as usize].send_next_request_bank_numa(clock.time(), current_bank_id);
        clock.tick_by(1); //skip to next dead time

        //if there are no more request, set the program finish time
        if  domains[current_domain as usize].read_queue.is_empty() && 
        domains[current_domain as usize].write_queue.is_empty() && 
        domains[current_domain as usize].write_queue_node2.is_empty() && 
        domains[current_domain as usize].read_queue_node2.is_empty() && 
        domains[current_domain as usize].tick_finished == 0 
        {
            domains[current_domain as usize].tick_finished = clock.time();
        }
        current_domain = (current_domain + 1) % constraints.num_domains;
        current_bank_id = (current_bank_id + 1) % 3;
    }
    (clock.time(), domains.clone())
}

fn simulate_fs_bta_dve(domains: &mut Vec<domain::Domain>) -> (u64, Vec<domain::Domain>){
    let mut clock = clock::Clock::new();
    let constraints = clock::Constraints::new(domains.len() as u16, 15);
    let mut current_domain: u16 = 0;
    let mut current_bank_id: u16 = 0;

    //for each domain, split the threads evenly between qeueu and queue_node2
    for domain in domains.iter_mut() {
        domain.split_threads_evenly();
    }

    let mut last_transfer1 = 0;
    let mut last_transfer2 = 0;

    let mut allowed_domain_for_node_transfer = 0;
    let mut domain_transfer_timer = 0;

    while domains.iter().any(|domain| !domain.read_queue.is_empty()) || 
        domains.iter().any(|domain| !domain.write_queue.is_empty() || 
        domains.iter().any(|domain| !domain.write_queue_node2.is_empty()) ||
        domains.iter().any(|domain| !domain.read_queue_node2.is_empty()))||
        clock.transfer_queue_to_node_1 != None || clock.transfer_queue_to_node_2 != None ||
        domains.iter().any(|domain| !domain.numa1_to_numa2.is_empty()) ||
        domains.iter().any(|domain| !domain.numa2_to_numa1.is_empty()) ||
        clock.read_back_from_node_1 != 0 || clock.read_back_from_node_2 != 0
        {
        if domain_transfer_timer == 0{   
            //transfer requests from node 1 to node 2
            clock.transfer_queue_to_node_2 = domains[allowed_domain_for_node_transfer as usize].get_next_t_from_numa1(clock.time());
            if clock.transfer_queue_to_node_2 != None {
                domains[allowed_domain_for_node_transfer as usize].data_transfers += DATA_RETRIVAL_NODE_DELAY;
            }
            
            //transfer requests from node 2 to node 1
            clock.transfer_queue_to_node_1 = domains[allowed_domain_for_node_transfer as usize].get_next_t_from_numa2(clock.time());
            if clock.transfer_queue_to_node_1 != None {
                domains[allowed_domain_for_node_transfer as usize].data_transfers += DATA_RETRIVAL_NODE_DELAY;
            }
            
            allowed_domain_for_node_transfer = (allowed_domain_for_node_transfer + 1) % constraints.num_domains;
            domain_transfer_timer = DATA_RETRIVAL_NODE_DELAY;
        }
        else {
            domain_transfer_timer -= 1;
        }
        
        //remove the request from the transfer queue if it is done into the read or write queue
        if clock.transfer_queue_to_node_1 != None {
            let mut request = clock.transfer_queue_to_node_1.unwrap();
            //do not propagate the read again
            request.skip = true;
            clock.transfer_queue_to_node_1 = None;
            domains[request.domain as usize].write_queue.push(request);
        }

        if clock.transfer_queue_to_node_2 != None{
            let mut request = clock.transfer_queue_to_node_2.unwrap();
            request.skip = true;
            clock.transfer_queue_to_node_2 = None;
            domains[request.domain as usize].write_queue_node2.push(request);
        }

        if clock.time() % constraints.dead_time  != 0{
            clock.tick_by(1);
            continue;
        }
        //Send the next 
        domains[current_domain as usize].send_next_request_bank_dve(clock.time(), current_bank_id);
        clock.tick_by(1);

        //if there are no more request, set the program finish time
        if  domains[current_domain as usize].read_queue.is_empty() && 
        domains[current_domain as usize].write_queue.is_empty() && 
        domains[current_domain as usize].write_queue_node2.is_empty() && 
        domains[current_domain as usize].read_queue_node2.is_empty() && 
        domains[current_domain as usize].tick_finished == 0
        {
            domains[current_domain as usize].tick_finished = clock.time();
        }
        current_domain = (current_domain + 1) % constraints.num_domains;
        current_bank_id = (current_bank_id + 1) % 3;
    }
    (clock.time(), domains.clone())
}

fn process_trace_file() -> Vec<domain::Domain> {
    let mut domains: Vec<domain::Domain> = Vec::new();

    // //turn data into domain structs
     let file = File::open("traces/trace.txt").expect("Failed to open trace.txt");
    // let file = File::open("new_trace/final_trace_new.txt").expect("Failed to open trace.txt");
    let reader = BufReader::new(file);

    //read trace and turn into domain structs
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let line = line.trim();
        let line = line.split_whitespace();
        let line: Vec<&str> = line.collect();
        let domain_id: u16 = line[0].parse().expect("Failed to parse domain id");
        let request_type: &str = line[1];
        let cylce_in: u64 = line[2].parse().expect("Failed to parse cycle in");
        let bank_id: u16 = line[3].parse().expect("Failed to parse bank id");
        let thread_id: u16 = line[4].parse().expect("Failed to parse thread id");
        let node = line[5].parse().expect("Failed to parse node id");
        let channel = line[6].parse().expect("Failed to parse channel id");
        let request: domain::Request = match request_type {
            "W" => domain::Request::new(domain::RequestType::WriteRequest, cylce_in, bank_id, thread_id, node, channel, domain_id),
            "R" => domain::Request::new(domain::RequestType::ReadRequest, cylce_in, bank_id, thread_id, node, channel, domain_id),
            _ => panic!("Invalid request type"),
        };
        //push x domains into the vector until the domain id is reached
        while domains.len() <= domain_id as usize {
            domains.push(domain::Domain::new(domains.len() as u16));
        }

        match request.request_type {
            domain::RequestType::WriteRequest => {
                domains[domain_id as usize].add_write_request(request);
            },
            domain::RequestType::ReadRequest => {
                domains[domain_id as usize].add_read_request(request);
            },
        }
    }

    domains
}
fn main() {
    // read trace file and turn into domain structs
    let domains = process_trace_file();
    
    
    let mut bta_domains = domains.clone();
    let mut bta_dve_domains = domains.clone();
    let mut base_two_domains = domains.clone();
    let mut bta_mirror_domains = domains.clone();

    //threads to make it run faster
    let bta_thread = thread::spawn(move || {
        simulate_fs_base_numa(&mut bta_domains)
    });
    
    let bta_dve_thread = thread::spawn(move || {
        simulate_fs_bta_dve(&mut bta_dve_domains)
    });

    // let base_channels = thread::spawn(move || {
    //     simulate_base_two_channels(&mut base_two_domains)
    // });
    
    // let bta_channel_mirror = thread::spawn(move || {
    //     simulate_fs_mirrior_channels(&mut bta_mirror_domains)
    // });
    
    let bta_data = bta_thread.join().unwrap();
    let bta_dve_data = bta_dve_thread.join().unwrap();
    // let base_channels_data = base_channels.join().unwrap();
    // let bta_channel_mirror_data = bta_channel_mirror.join().unwrap();

    println!("BTA Numa Stats:");
    println!("\tTotal ticks to finish entire simulation: {}", bta_data.0);
    for domain in bta_data.1.iter() {
        println!("\tDomain {} finished in {} ticks: fake requests: {}", domain.id, domain.tick_finished, domain.fake_requests);
    }

    println!("\nDve+FS-BTA Stats:");
    println!("\tTotal ticks to finish entire simulation: {}", bta_dve_data.0);
    for domain in bta_dve_data.1.iter() {
        println!("\tDomain {} finished in {} ticks: fake requests: {}", domain.id, domain.tick_finished, domain.fake_requests);
    }
    
    println!("\nDve+FS:BTA is {} times faster than BTA", bta_data.0 as f64 / bta_dve_data.0 as f64);

    println!("Data Transfers Dve: {}", bta_dve_data.1.iter().fold(0, |acc, domain| acc + domain.data_transfers));

    println!("Data Transfers Norm: {}", bta_data.1.iter().fold(0, |acc, domain | acc + domain.data_transfers));


    // println!("\nFS Mirroring Channels Stats:");
    // println!("\tTotal ticks to finish entire simulation: {}", bta_channel_mirror_data.0);
    // for domain in bta_channel_mirror_data.1.iter() {
    //     println!("\tDomain {} finished in {} ticks: fake requests: {}", domain.id, domain.tick_finished, domain.fake_requests);
    // }

    // println!("\nFS Base Two Channels Stats:");
    // println!("\tTotal ticks to finish entire simulation: {}", base_channels_data.0);
    // for domain in base_channels_data.1.iter() {
    //     println!("\tDomain {} finished in {} ticks: fake requests: {}", domain.id, domain.tick_finished, domain.fake_requests);
    // }

    // println!("\nFS Mirroring Channels is {} times faster than Base Two Channels", base_channels_data.0 as f64 / bta_channel_mirror_data.0 as f64);

}

