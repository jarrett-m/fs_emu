from random import randint
from sys import argv

def gen_trace(domains: int, cycles: int, banks: int, file_name: str, cycle_range: tuple[int, int], threads: int, write_ratio: int, channel_ratio: int):
    print (f"Generating trace with {domains} domains, {cycles} cycles, {banks} banks, {cycle_range} cycle range, {threads} threads, {write_ratio}% write ratio, {channel_ratio}% channel ratio")
    with open(file_name, 'w') as f:
        curr = 0
        for _ in range(cycles):
            curr += randint(cycle_range[0], cycle_range[1])
            thread = randint(0, threads-1)
            
            if thread > threads//2:
                node = 2
            else:
                node = 1

            channel = 0 if randint(0, 100) <= channel_ratio else 1
            #domain, op, cycle, bank, thread
                        #domain                #op                                            #cycle    #bank            #thread  #node  #channel
            f.write(f"{randint(0, domains-1)} {'W' if randint(0, 100) < write_ratio else 'R'} {curr} {randint(0,banks)} {thread} {node} {channel}\n")

def gen_no_rand_trace (domains: int, cycles: int, banks: int, file_name: str):
    with open(file_name, 'w') as f:
        for cycle in range(cycles):
            f.write(f"{cycle % domains} {'W' if randint(0,1) == 0 else 'R'} {cycle}\n")

def gen_trace_with_odds(domains: int, odds_of_write, cycles: int, banks: int, file_name: str):
    with open(file_name, 'w') as f:
        current_cycle = 0
        for cycle in range(cycles):
            domain = randint(0, domains-1)
            f.write(f"{domain} {'W' if randint(0,100) < odds_of_write[domain] else 'R'} {current_cycle}\n")
            current_cycle += randint(1, 10)
            if current_cycle > cycles:
                break

def gen_trace_constant_and_random(domains: int, odds_of_write, cycles: int, banks: int, file_name: str):
    constant_trace = []
    #inject a instruction every 100 cycles, back and forth reads and writes for domain 1
    for i in range(0, cycles, 100):
        constant_trace.append(f"0 {'W' if i % 400 == 0 else 'R'} {i}\n")
        
    
    random_trace = []
    #inject random instructions for domain 0, cannot be on cycles that are already used by the constant trace
    for i in range(cycles):
        if i % 100 != 0:
            if randint(0, 100) > 95: 
                random_trace.append(f"1 {'W' if randint(0,100) < odds_of_write[1] else 'R'} {i}\n")
    
    with open(file_name, 'w') as f:
        #write both traces, sort by cycle
        for line in sorted(constant_trace + random_trace, key=lambda x: int(x.split(' ')[2])):
            f.write(line)


def gen_trace_constant_and_random_bank(domains: int, odds_of_write, cycles: int, banks: int, file_name: str):
    constant_trace = []
    #inject a instruction every 100 cycles, back and forth reads and writes for domain 1
    for i in range(0, cycles, 100):
        constant_trace.append(f"0 {'W' if i % 400 == 0 else 'R'} {i} {randint(0,15)}\n")
        
    
    random_trace = []
    #inject random instructions for domain 0, cannot be on cycles that are already used by the constant trace
    for i in range(cycles):
        if i % 100 != 0:
            if randint(0, 100) > 95: 
                random_trace.append(f"1 {'W' if randint(0,100) < odds_of_write[1] else 'R'} {i} {randint(0,15)}\n")
    
    with open(file_name, 'w') as f:
        #write both traces, sort by cycle
        for line in sorted(constant_trace + random_trace, key=lambda x: int(x.split(' ')[2])):
            f.write(line)

def gen_trace_with_odds_bank(domains: int, odds_of_write, cycles: int, banks: int, file_name: str):
    with open(file_name, 'w') as f:
        current_cycle = 0
        for cycle in range(cycles):
            domain = randint(0, domains-1)
            f.write(f"{domain} {'W' if randint(0,100) < odds_of_write[domain] else 'R'} {current_cycle} {randint(0,15)}\n")
            current_cycle += randint(7, 15)
            if current_cycle > cycles:
                break
            
if __name__ == "__main__":
    if len(argv) == 1:
        # gen_trace(1, 1000, 15,"trace.txt") #default
        gen_trace_with_odds(8, [30, 30, 30, 30, 30, 30, 30, 30], randint(1100000,1500000), 16,"trace.txt")
        #gen_trace_with_odds_bank(8, [25,25,75,50,50,50,70,90], 1500000, 16,"trace.txt")
        # gen_trace_with_odds_bank(1, [100], 10000, 16,"trace.txt")
    elif len(argv) == 4:
        print("Usage: python trace_gen.py <domains> <cycles> <banks> <file_name>")
        # gen_trace(int(argv[1]), int(argv[2]), int(argv[3]), argv[4])
        gen_trace_with_odds(int(argv[1]), [20, 10], int(argv[2]), int(argv[3]), argv[4])
        print("Trace generated")
    else:
            
        gen_trace(int(argv[1]), int(argv[2]), int(argv[3]), argv[4], (int(argv[5]), int(argv[6])), int(argv[7]), int(argv[8]), int(argv[9]))

    