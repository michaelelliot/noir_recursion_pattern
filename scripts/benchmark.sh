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
  echo
}

run_benchmark "test_honk_inner1"
run_benchmark "test_honk_inner2"
run_benchmark "test_honk_outer"
