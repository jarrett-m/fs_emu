#!/bin/bash

set -e
for i in {1..10}; do
    # Generate the trace file
    python3 trace_gen.py 2 100000 16 traces/trace.txt

    # Run the Rust program 10 times and output the result to a file

    cargo run >> output.txt
done


for i in {1..10}; do
    # Generate the trace file
    python3 trace_gen.py 8 100000 16 traces/trace.txt

    # Run the Rust program 10 times and output the result to a file

    cargo run >> output.txt
done
