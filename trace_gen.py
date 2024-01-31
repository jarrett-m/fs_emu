from random import randint
from sys import argv

# def get_randint_mod3(banks: int, current_bank: int):
#     if current_bank % 3 == 0:
#         #return a random bank that mod % 3 == 1
#         while True:
#             next_bank = randint(0, banks-1)
#             if next_bank % 3 == 1:
#                 return next_bank
#     elif current_bank % 3 == 1:
#         #return a random bank that mod % 3 == 2
#         while True:
#             next_bank = randint(0, banks-1)
#             if next_bank % 3 == 2:
#                 return next_bank
#     else:
#         #return a random bank that mod % 3 == 0
#         while True:
#             next_bank = randint(0, banks-1)
#             if next_bank % 3 == 0:
#                 return next_bank

def gen_trace(domains: int, cycles: int, banks: int, file_name: str):
    with open(file_name, 'w') as f:
        for cycle in range(cycles):
            cycle += randint(1, 1000)
            f.write(f"{randint(0, domains-1)} {'W' if randint(0,1) == 0 else 'R'} {cycle}\n")

def gen_no_rand_trace (domains: int, cycles: int, banks: int, file_name: str):
    with open(file_name, 'w') as f:
        for cycle in range(cycles):
            f.write(f"{cycle % domains} {'W' if randint(0,1) == 0 else 'R'} {cycle}\n")

if __name__ == "__main__":
    if len(argv) == 1:
        gen_trace(4, randint(1000000,5000000), 16,"trace.txt") #default
    elif len(argv) < 4:
        print("Usage: python trace_gen.py <domains> <cycles> <banks> <file_name>")
    else:
        gen_trace(int(argv[1]), int(argv[2]), int(argv[3]), argv[4])

    