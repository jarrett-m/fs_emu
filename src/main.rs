//Import all the modules
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread;

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

// fn simulate_fs_bankp_reorg_noprofile(domains: &mut Vec<domain::Domain>) -> u64 {
//     let mut clock = clock::Clock::new();
//     let constraints = clock::Constraints::new(domains.len() as u16, 6);

//     let mut write: bool = true;
//     while domains.iter().any(|domain| !domain.read_queue.is_empty()) ||  domains.iter().any(|domain| !domain.write_queue.is_empty()) {
//         //check which request (next read or write) was sent first, and send it
//         for i in 0..constraints.num_domains {
//             if write{
//                     domains[i as usize].send_next_write_request(clock.time());
//                     clock.tick_by(constraints.dead_time as u64); //skip to next dead time
//                 }
//             else {
//                 domains[i as usize].send_next_read_request(clock.time());
//                 clock.tick_by(constraints.dead_time as u64); //skip to next dead time
//             }   
//         }
//         for current_domain in 0..constraints.num_domains {
//             if domains[current_domain as usize].read_queue.is_empty() && domains[current_domain as usize].write_queue.is_empty()  && domains[current_domain as usize].tick_finished == 0{
//                 domains[current_domain as usize].tick_finished = clock.time();
//             }
//         }

//         write = !write;
//         clock.tick_by(9); //skip to next dead time
//     }

//     //println!("{} total system ticks to complete", clock.time());
//     clock.time()
// }

// fn simulate_fs_bankp_reorg_profile(domains: &mut Vec<domain::Domain>) -> u64 {
//     let mut clock = clock::Clock::new();
//     let constraints = clock::Constraints::new(domains.len() as u16, 6);
    
//     let mut write: bool = true;
//     while domains.iter().any(|domain| !domain.read_queue.is_empty()) ||  domains.iter().any(|domain| !domain.write_queue.is_empty()) {
//         //check which request (next read or write) was sent first, and send it
//         for i in 0..constraints.num_domains {
//             if write{
//                 if domains[i as usize].can_write() {
//                     domains[i as usize].send_next_write_request(clock.time());
//                     clock.tick_by(constraints.dead_time as u64); //skip to next dead time
//                 }
//             }
//             else if domains[i as usize].can_read() {
//                 domains[i as usize].send_next_read_request(clock.time());
//                 clock.tick_by(constraints.dead_time as u64); //skip to next dead time
//             }
//         }

//         for current_domain in 0..constraints.num_domains {
//             if domains[current_domain as usize].read_queue.is_empty() && domains[current_domain as usize].write_queue.is_empty()  && domains[current_domain as usize].tick_finished == 0{
//                 domains[current_domain as usize].tick_finished = clock.time();
//             }
//         }
//         write = !write;
//         clock.tick_by(9); //skip to next dead time
//     }
//     //println!("{} total system ticks to complete", clock.time());
//     clock.time()
// }

// fn simulate_fs_nop(domains: &mut Vec<domain::Domain>) -> u64{
//     let mut clock = clock::Clock::new();
//     let constraints = clock::Constraints::new(domains.len() as u16, 43);
//     let mut current_domain: u16 = 0;
//     while domains.iter().any(|domain| !domain.read_queue.is_empty()) || domains.iter().any(|domain| !domain.write_queue.is_empty()) {
//         //check which request (next read or write) was sent first, and send it
//         domains[current_domain as usize].send_next_request(clock.time());
//         clock.tick_by(constraints.dead_time as u64); //skip to next dead time

//         if domains[current_domain as usize].read_queue.is_empty() && domains[current_domain as usize].write_queue.is_empty()  && domains[current_domain as usize].tick_finished == 0{
//             domains[current_domain as usize].tick_finished = clock.time();
//         }

//         current_domain = (current_domain + 1) % constraints.num_domains;
//     }

//     println!("{} total system ticks to complete", clock.time());
//     // for domain in domains.iter() {
//     //     println!("Domain {} finished in {} ticks", domain.id, domain.tick_finished);
//     // }
//     clock.time()
// }

// fn simulate_fs_bankp(domains: &mut Vec<domain::Domain>) -> u64 {
//     let mut clock = clock::Clock::new();
//     let constraints = clock::Constraints::new(domains.len() as u16, 15);
//     let mut current_domain: u16 = 0;
//     while domains.iter().any(|domain| !domain.read_queue.is_empty()) || domains.iter().any(|domain| !domain.write_queue.is_empty()) {
//         //check which request (next read or write) was sent first, and send it
//         domains[current_domain as usize].send_next_request(clock.time());
//         clock.tick_by(constraints.dead_time as u64); //skip to next dead time

//         if domains[current_domain as usize].read_queue.is_empty() && domains[current_domain as usize].write_queue.is_empty()  && domains[current_domain as usize].tick_finished == 0{
//             domains[current_domain as usize].tick_finished = clock.time();
//         }

//         current_domain = (current_domain + 1) % constraints.num_domains;
//     }

//     println!("FS_BANKP: {} total system ticks", clock.time());
//     clock.time()
    
// }

// fn simulate_fs_bta_wrprofile(domains: &mut Vec<domain::Domain>) -> u64 {
//     let mut clock = clock::Clock::new();
//     let constraints = clock::Constraints::new(domains.len() as u16, 6);

//     let mut current_bank_id: u16 = 0;
//     let mut write: bool = true;

//     while domains.iter().any(|domain| !domain.read_queue.is_empty()) || domains.iter().any(|domain| !domain.write_queue.is_empty()) {
//         for i in 0..constraints.num_domains {
//             if write{
//                 if domains[i as usize].can_write() {
//                     domains[i as usize].send_next_write_request_bta(clock.time(), current_bank_id);
//                     clock.tick_by(constraints.dead_time as u64); //skip to next dead time
//                 }
//             }
//             else if domains[i as usize].can_read() {
//                 domains[i as usize].send_next_read_request_bta(clock.time(), current_bank_id);
//                 clock.tick_by(constraints.dead_time as u64); //skip to next dead time
//             }

//             current_bank_id = (current_bank_id + 1) % 3;
//             domains[i as usize].pointer = (domains[i as usize].pointer + 1) % domains[i as usize].write_tracker.len(); //move to next read or write
//         }

//         for current_domain in 0..constraints.num_domains {
//             if domains[current_domain as usize].read_queue.is_empty() && domains[current_domain as usize].write_queue.is_empty()  && domains[current_domain as usize].tick_finished == 0{
//                 domains[current_domain as usize].tick_finished = clock.time();
//             }
//         }
//         write = !write;
//         clock.tick_by(9); //skip to next dead time
//     }
//     println!("FS_BTP: {} ticks to complete with ", clock.time());
//     clock.time()
// }

fn simulate_fs_bta(domains: &mut Vec<domain::Domain>) -> (u64, Vec<domain::Domain>){
    let mut clock = clock::Clock::new();
    let constraints = clock::Constraints::new(domains.len() as u16, 15);
    let mut current_domain: u16 = 0;
    let mut current_bank_id: u16 = 0;

    while domains.iter().any(|domain| !domain.read_queue.is_empty()) || domains.iter().any(|domain| !domain.write_queue.is_empty()) {
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

fn simulate_fs_bta_dve(domains: &mut Vec<domain::Domain>) -> (u64, Vec<domain::Domain>){
    let mut clock = clock::Clock::new();
    let constraints = clock::Constraints::new(domains.len() as u16, 15);
    let mut current_domain: u16 = 0;
    let mut current_bank_id: u16 = 0;

    //for each domain, split the threads evenly between qeueu and queue_node2
    for domain in domains.iter_mut() {
        domain.split_threads_evenly();
    }

    while domains.iter().any(|domain| !domain.read_queue.is_empty()) || domains.iter().any(|domain| !domain.write_queue.is_empty()) {
        //Send the next request with allowed bank
        domains[current_domain as usize].send_next_request_bank_dve(clock.time(), current_bank_id);
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

fn process_trace_file() -> Vec<domain::Domain> {
    let mut domains: Vec<domain::Domain> = Vec::new();

    //turn data into domain structs
    let file = File::open("traces/trace.txt").expect("Failed to open trace.txt");
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
        let request: domain::Request = match request_type {
            "W" => domain::Request::new(domain::RequestType::WriteRequest, cylce_in, bank_id, thread_id),
            "R" => domain::Request::new(domain::RequestType::ReadRequest, cylce_in, bank_id, thread_id),
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

    //threads to make it run faster
    let bta_thread = thread::spawn(move || {
        simulate_fs_bta(&mut bta_domains)
    });
    
    let bta_dve_thread = thread::spawn(move || {
        simulate_fs_bta_dve(&mut bta_dve_domains)
    });
    
    let bta_data = bta_thread.join().unwrap();
    let bta_dve_data = bta_dve_thread.join().unwrap();

    println!("BTA Stats:");
    println!("\tTotal ticks to finish entire simulation: {}", bta_data.0);
    for domain in bta_data.1.iter() {
        println!("\tDomain {} finished in {} ticks: fake requests: {}", domain.id, domain.tick_finished, domain.fake_requests);
    }

    println!("\nDve+FSL:BTA Stats");
    println!("\tTotal ticks to finish entire simulation: {}", bta_dve_data.0);
    for domain in bta_dve_data.1.iter() {
        println!("\tDomain {} finished in {} ticks: fake requests: {}", domain.id, domain.tick_finished, domain.fake_requests);
    }
    
    println!("\nRank Partition is {} times faster than BTA", bta_data.0 as f64 / bta_dve_data.0 as f64);
}

