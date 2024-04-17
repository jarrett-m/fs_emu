import os
import sys 
folder_path = 'new_trace/'

def address_to_bank(addr):
    addr = int(addr, 16)
    bank = (addr >> 6) & 0x7 # 8 banks
    return bank

def address_to_channel(addr):
    addr = int(addr, 16)
    channel = (addr >> 3) & 0x1 # 2 channels
    return channel

def thread_to_node(thread):
    if int(thread) % 2 == 1:
        return 1
    else:
        return 0

for filename in os.listdir(folder_path):
    if filename == 'final_trace.txt' or filename == 'final_trace_new.txt':
        continue
    file_path = os.path.join(folder_path, filename)
    if os.path.isfile(file_path):
        with open(file_path, 'r') as file:
            smallest_bank = 0
            with open('new_trace/final_trace.txt', 'a') as f:
                for l in file:
                    line = l.strip().split(',')
                    cycle = line[0].strip()
                    try:
                        op = line[1].strip()
                    except:
                        print(l)
                    addr = line[2].strip()
                    thread = file_path.split('/')[-1].split('_')[-1].split('.')[0][1]
                    bank = address_to_bank(addr)
                    channel = address_to_channel(addr)
                    node = thread_to_node(thread)

                        #domain, op, cycle, bank, thread
                    f.write(f"{sys.argv[1]} {op} {cycle} {bank} {thread} {node} {channel}\n")

with open('new_trace/final_trace.txt', 'r') as input_file, open('new_trace/final_trace_new.txt', 'w') as output_file:
    # Read each line from the input file, sort, and write to the output file
    for line in sorted(input_file, key=lambda x: int(x.split()[2])):
        output_file.write(line)

