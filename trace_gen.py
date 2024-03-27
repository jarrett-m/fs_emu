from random import randint
from sys import argv

def gen_trace(domains: int, cycles: int, banks: int, file_name: str):
    with open(file_name, 'w') as f:
        for cycle in range(cycles):
            cycle += randint(1, 300)
            f.write(f"{randint(0, domains-1)} {'W' if randint(0,1) == 0 else 'R'} {cycle} {randint(0,banks)} {randint(0,7)}\n")

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
        gen_trace(1, 1000, 15,"trace.txt") #default
        #gen_trace_with_odds(2, [25, 75], randint(1100000,1500000), 16,"trace.txt")
        #gen_trace_with_odds_bank(8, [25,25,75,50,50,50,70,90], 1500000, 16,"trace.txt")
        # gen_trace_with_odds_bank(1, [100], 10000, 16,"trace.txt")
    elif len(argv) == 4:
        print("Usage: python trace_gen.py <domains> <cycles> <banks> <file_name>")
        gen_trace(int(argv[1]), int(argv[2]), int(argv[3]), argv[4])
    else:
        gen_trace(int(argv[1]), int(argv[2]), int(argv[3]), argv[4])

    