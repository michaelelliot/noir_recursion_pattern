#!/usr/bin/env sh

set -e

run_benchmark() {
  TEST_NAME=$1
  echo $TEST_NAME
  # Run the cargo test and capture the output
  BENCHMARK=$(/usr/bin/time -l cargo test "$TEST_NAME" -- --test-threads=1 2>&1)
  # Extract peak memory (maximum resident set size) from the output
  PEAK_MEM=$(echo "$BENCHMARK" | awk '/maximum resident set size/ {print $1 / 1024 / 1024 " MB"}')
  # Extract the elapsed time (real time) from the output
  SECS_ELAPSED=$(echo "$BENCHMARK" | awk '/real/ {print $1}')
  # Output the results
  echo "Peak memory: $PEAK_MEM"
  echo "Proving time: $SECS_ELAPSED secs"
}

get_circuit_size() {
  CIRCUIT_SIZE=$(bb gates -b ./target/$1.json -- -h | jq '.functions[0].circuit_size')
  echo "Circuit size: $CIRCUIT_SIZE gates"
}

run_benchmark test_honk_inner1; get_circuit_size inner1; echo
run_benchmark test_honk_inner2; get_circuit_size inner2; echo
run_benchmark test_honk_outer; get_circuit_size outer; echo
