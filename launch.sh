#!/bin/bash

for i in {1..10}; do
    # Generate the trace file
    #                    domains: int, cycles: int, banks: int, file_name: str,   cycle_range: tuple[int, int], threads: int, write_ratio: int, channel_ratio: int
    python3 trace_gen.py 8             100000        16          traces/trace.txt 1 20                          8            30                50

    # Run the Rust program 10 times and output the result to a file

    cargo run >> outputs/BASELINE8domains_16banks_lowCycleStep_8threads_30write_50channel.txt
done

for i in {1..10}; do
    # Generate the trace file
    python3 trace_gen.py 8             100000        16          traces/trace.txt 10 50                          8            30                50

    cargo run >> outputs/8domains_16banks_midCycleStep_8threads_30write_50channel.txt
done

for i in {1..10}; do
    # Generate the trace file
    python3 trace_gen.py 8             100000        16          traces/trace.txt 50 300                         8            30                50

    cargo run >> outputs/8domains_16banks_highCycleStep_8threads_30write_50channel.txt
done

for i in {1..10}; do
    # Generate the trace file
    python3 trace_gen.py 8             100000        16          traces/trace.txt 1 20                          8            10                50

    cargo run >> outputs/8domains_16banks_lowCycleStep_8threads_10write_50channel.txt
done

for i in {1..10}; do
    # Generate the trace file
    python3 trace_gen.py 8             100000        16          traces/trace.txt 1 20                          8            50                50

    cargo run >> outputs/8domains_16banks_lowCycleStep_8threads_50write_50channel.txt
done

for i in {1..10}; do
    # Generate the trace file
    #                    domains: int, cycles: int, banks: int, file_name: str,   cycle_range: tuple[int, int], threads: int, write_ratio: int, channel_ratio: int
    python3 trace_gen.py 8             100000        16          traces/trace.txt 1 20                          8            30                80

    # Run the Rust program 10 times and output the result to a file

    cargo run >> outputs/8domains_16banks_lowCycleStep_8threads_30write_80channel.txt
done