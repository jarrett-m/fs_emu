
set -e
            #domains, cycles, banks, trace file
python3 trace_gen.py 2 100000 15 traces/trace.txt
# python3 trace_gen.py 2 100000 15 new_trace/trace.txt
cargo run
