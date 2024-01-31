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

    println!("{} ticks to complete", clock.time());
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
    }

    println!("{} ticks to complete", clock.time());
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
    print!("FS Rank Partition: ");
    let mut domains_copy = domains.clone();
    let rank = simulate_fs_rankp(&mut domains_copy);

    print!("FS No Partition: ");
    domains_copy = domains.clone();
    let nop = simulate_fs_nop(&mut domains_copy);

    print!("FS BTA: ");
    domains_copy = domains.clone();
    let bta = simulate_fs_bta(&mut domains_copy);
    
   println!();
    //Preformance
    //FS Rank VS No Partition
    println!("Prefromane of Rank over No P {}", nop as f64/rank  as f64 );
    println!("Expected {}", 0.74/0.20);

    println!();
    //FS BTA VS No Partition
    println!("Prefromane of BTA over No P {}", nop as f64/bta  as f64);
    println!("Expected {}", 0.40/0.20);

    println!();
    //FS BTA VS Rank
    println!("Prefromane of BTA over Rank {}", bta as f64/rank  as f64);
    println!("Expected {}", 0.74/0.40);
    
}

