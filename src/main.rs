//Import all the modules
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod clock;
mod domain;

fn simulate_fs_rankp(domains: &mut Vec<domain::Domain>) -> u64 {
    let mut clock = clock::Clock::new();
    let constraints = clock::Constraints::new(domains.len() as u16, 7);
    let mut current_domain: u16 = 0;
    while domains.iter().any(|domain| !domain.read_queue.is_empty()) || domains.iter().any(|domain| !domain.write_queue.is_empty()) {
        //check which request (next read or write) was sent first, and send it
        domains[current_domain as usize].send_next_request(clock.time());
        clock.tick_by(constraints.dead_time as u64); //skip to next dead time
        current_domain = (current_domain + 1) % constraints.num_domains;
    }

    println!("{} total system ticks to complete", clock.time());
    clock.time()
    
}

fn simulate_fs_bankp(domains: &mut Vec<domain::Domain>) -> u64 {
    let mut clock = clock::Clock::new();
    let constraints = clock::Constraints::new(domains.len() as u16, 15);
    let mut current_domain: u16 = 0;
    while domains.iter().any(|domain| !domain.read_queue.is_empty()) || domains.iter().any(|domain| !domain.write_queue.is_empty()) {
        //check which request (next read or write) was sent first, and send it
        domains[current_domain as usize].send_next_request(clock.time());
        clock.tick_by(constraints.dead_time as u64); //skip to next dead time

        if domains[current_domain as usize].read_queue.is_empty() && domains[current_domain as usize].write_queue.is_empty()  && domains[current_domain as usize].tick_finished == 0{
            domains[current_domain as usize].tick_finished = clock.time();
        }

        current_domain = (current_domain + 1) % constraints.num_domains;


    }

    println!("{} total system ticks to complete", clock.time());
    clock.time()
    
}

fn simulate_fs_bankp_reorg_noprofile(domains: &mut Vec<domain::Domain>) -> u64 {
    let mut clock = clock::Clock::new();
    let constraints = clock::Constraints::new(domains.len() as u16, 6);

    let mut write: bool = true;
    while domains.iter().any(|domain| !domain.read_queue.is_empty()) ||  domains.iter().any(|domain| !domain.write_queue.is_empty()) {
        //check which request (next read or write) was sent first, and send it
        for i in 0..constraints.num_domains {
            if write{
                    domains[i as usize].send_next_write_request(clock.time());
                    clock.tick_by(constraints.dead_time as u64); //skip to next dead time
                }
            else {
                domains[i as usize].send_next_read_request(clock.time());
                clock.tick_by(constraints.dead_time as u64); //skip to next dead time
            }   
        }
        for current_domain in 0..constraints.num_domains {
            if domains[current_domain as usize].read_queue.is_empty() && domains[current_domain as usize].write_queue.is_empty()  && domains[current_domain as usize].tick_finished == 0{
                domains[current_domain as usize].tick_finished = clock.time();
            }
        }

        write = !write;
        clock.tick_by(9); //skip to next dead time
    }

    println!("{} total system ticks to complete", clock.time());
    clock.time()
}

fn simulate_fs_bankp_reorg_profile(domains: &mut Vec<domain::Domain>) -> u64 {
    let mut clock = clock::Clock::new();
    let constraints = clock::Constraints::new(domains.len() as u16, 6);
    
    let mut write: bool = true;
    while domains.iter().any(|domain| !domain.read_queue.is_empty()) ||  domains.iter().any(|domain| !domain.write_queue.is_empty()) {
        //check which request (next read or write) was sent first, and send it
        for i in 0..constraints.num_domains {
            if write{
                if domains[i as usize].can_write() {
                    domains[i as usize].send_next_write_request(clock.time());
                    clock.tick_by(constraints.dead_time as u64); //skip to next dead time
                }
            }
            else if domains[i as usize].can_read() {
                domains[i as usize].send_next_read_request(clock.time());
                clock.tick_by(constraints.dead_time as u64); //skip to next dead time
            }
        }

        for current_domain in 0..constraints.num_domains {
            if domains[current_domain as usize].read_queue.is_empty() && domains[current_domain as usize].write_queue.is_empty()  && domains[current_domain as usize].tick_finished == 0{
                domains[current_domain as usize].tick_finished = clock.time();
            }
        }

        
        write = !write;
        clock.tick_by(9); //skip to next dead time
        
    }

    println!("{} total system ticks to complete", clock.time());
    clock.time()
}

fn simulate_fs_nop(domains: &mut Vec<domain::Domain>) -> u64{
    let mut clock = clock::Clock::new();
    let constraints = clock::Constraints::new(domains.len() as u16, 43);
    let mut current_domain: u16 = 0;
    while domains.iter().any(|domain| !domain.read_queue.is_empty()) || domains.iter().any(|domain| !domain.write_queue.is_empty()) {
        //check which request (next read or write) was sent first, and send it
        domains[current_domain as usize].send_next_request(clock.time());
        clock.tick_by(constraints.dead_time as u64); //skip to next dead time
        current_domain = (current_domain + 1) % constraints.num_domains;

        if domains[current_domain as usize].read_queue.is_empty() && domains[current_domain as usize].write_queue.is_empty()  && domains[current_domain as usize].tick_finished == 0{
            domains[current_domain as usize].tick_finished = clock.time();
        }
    }

    println!("{} total system ticks to complete", clock.time());
    for domain in domains.iter() {
        println!("Domain {} finished in {} ticks", domain.id, domain.tick_finished);
    }
    clock.time()
}

fn simulate_fs_bta(domains: &mut Vec<domain::Domain>) -> u64 {
    let mut clock = clock::Clock::new();
    let constraints = clock::Constraints::new(domains.len() as u16, 15);
    let mut current_domain: u16 = 0;
    while domains.iter().any(|domain| !domain.read_queue.is_empty()) || domains.iter().any(|domain| !domain.write_queue.is_empty()) {
        //check which request (next read or write) was sent first, and send it
        domains[current_domain as usize].send_next_request(clock.time());
        clock.tick_by(constraints.dead_time as u64); //skip to next dead time
        current_domain = (current_domain + 1) % constraints.num_domains;
    }

    println!("{} ticks to complete", clock.time());
    clock.time()
}

fn test_side_channel_potential_wrprofile_vs_none(mut domains: Vec<domain::Domain>) {
    //the closer to 50/50 the worse the gains
    //more requests = more gains
    //we guess if we are going to have a read or write, if we dont we can skip it.
    //this leaks the write/read odds, but thats it I dont see a situation where that matters.
    //risk of worse preformance if odds are profiled incorrectly.

    print!("FS Bank Partition: ");
    let mut no_profile = domains.clone();
    let bank = simulate_fs_bankp_reorg_noprofile(&mut no_profile);

    print!("FS Bank R/W Profile: ");
    let mut profile = domains.clone();
    let wr_profile = simulate_fs_bankp_reorg_profile(&mut profile);

    println!("Preformance Increase: {}\n", bank as f64/wr_profile as f64);
    
    println!("Fake Requests No Profile");
    for domain in no_profile.iter(){
        println!("Domain: {}, Fake Requests: {}", domain.id, domain.fake_requests);
    }

    println!("\nFake Requests W/ Profile");
    for domain in profile.iter() {
        println!("Domain: {}, Fake Requests: {}", domain.id, domain.fake_requests);
    }

    println!("\nClock Fisnished Time No Profile");
    for domain in no_profile.iter() {
        println!("Domain: {}, Finished at: {}", domain.id, domain.tick_finished);
    }

    println!("\nClock Fisnished Time W/ Profile");
    for domain in profile.iter() {
        println!("Domain: {}, Finished at: {}", domain.id, domain.tick_finished);
    }

}

fn main() {
    //read trace.txt to build the write and read queues
    let mut domains: Vec<domain::Domain> = Vec::new();

    //turn data into domain structs
    let file = File::open("trace.txt").expect("Failed to open trace.txt");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let line = line.trim();
        let line = line.split_whitespace();
        let line: Vec<&str> = line.collect();
        let domain_id: u16 = line[0].parse().expect("Failed to parse domain id");
        let request_type: &str = line[1];
        let cylce_in: u64 = line[2].parse().expect("Failed to parse cycle in");
        let request: domain::Request = match request_type {
            "W" => domain::Request::new(domain::RequestType::WriteRequest, cylce_in),
            "R" => domain::Request::new(domain::RequestType::ReadRequest, cylce_in),
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
    

    //print all domains
    // for domain in domains.iter() {
    //     println!("Domain {}", domain.id);
    //     println!("Write Queue:");
    //     for request in domain.write_queue.iter() {
    //         println!("Request Type: {:?}, Cycle In: {}, Domain {}", request.request_type, request.cylce_in, domain.id);
    //     }
    //     println!("Read Queue:");
    //     for request in domain.read_queue.iter() {
    //         println!("Request Type: {:?}, Cycle In: {}, Domain {}", request.request_type, request.cylce_in, domain.id);
    //     }
    // }

    //run the simulation
    //simulate_baseline(&mut domains);
    domains[0].set_write_tracker(25);
    domains[1].set_write_tracker(25);
    test_side_channel_potential_wrprofile_vs_none(domains.clone());
}


//     print!("FS No Partition: ");
//     domains_copy = domains.clone();
//     let nop = simulate_fs_nop(&mut domains_copy);

//     print!("FS BTA: ");
//     domains_copy = domains.clone();
//     let bta = simulate_fs_bta(&mut domains_copy);
    
//    println!();
//     //Preformance
//     //FS Rank VS No Partition
//     println!("Prefromane of Rank over No P {}", nop as f64/rank  as f64 );
//     println!("Expected {}", 0.74/0.20);

//     println!();
//     //FS BTA VS No Partition
//     println!("Prefromane of BTA over No P {}", nop as f64/bta  as f64);
//     println!("Expected {}", 0.40/0.20);

//     println!();
//     //FS BTA VS Rank
//     println!("Prefromane of BTA over Rank {}", bta as f64/rank  as f64);
//     println!("Expected {}", 0.74/0.40);
    
    

