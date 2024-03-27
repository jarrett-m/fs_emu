
set -e
            #domains, cycles, banks, trace file
python3 trace_gen.py 2 100000 15 traces/trace.txt
cargo run
