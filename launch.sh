
set -e
            #domains, cycles, banks, trace file
python3 trace_gen.py 8 1000000 15 traces/trace.txt
cargo run
