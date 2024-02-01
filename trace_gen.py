from random import randint
from sys import argv

def gen_trace(domains: int, cycles: int, banks: int, file_name: str):
    with open(file_name, 'w') as f:
        for cycle in range(cycles):
            cycle += randint(1, 1000)
            f.write(f"{randint(0, domains-1)} {'W' if randint(0,1) == 0 else 'R'} {cycle}\n")

def gen_no_rand_trace (domains: int, cycles: int, banks: int, file_name: str):
    with open(file_name, 'w') as f:
        for cycle in range(cycles):
            f.write(f"{cycle % domains} {'W' if randint(0,1) == 0 else 'R'} {cycle}\n")

def gen_trace_with_odds(domains: int, odds_of_write, cycles: int, banks: int, file_name: str):
    with open(file_name, 'w') as f:
        current_cycle = 0
        for cycle in range(cycles):
            domain = randint(0, domains-1)
            odds_of_w = odds_of_write[domain]
            f.write(f"{domain} {'W' if randint(0,100) < odds_of_w else 'R'} {current_cycle}\n")
            current_cycle += randint(100, 1000)
            if current_cycle > cycles:
                break

    

if __name__ == "__main__":
    if len(argv) == 1:
        #gen_trace(4, randint(1000000,5000000), 16,"trace.txt") #default
        gen_trace_with_odds(4, [90, 50, 50, 50], randint(1100000,1500000), 16,"trace.txt")
    elif len(argv) < 4:
        print("Usage: python trace_gen.py <domains> <cycles> <banks> <file_name>")
    else:
        gen_trace(int(argv[1]), int(argv[2]), int(argv[3]), argv[4])

    