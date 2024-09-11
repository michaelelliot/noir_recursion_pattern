# noir_recursion_pattern

## Compile circuits
```sh
./scripts/prove-honk.sh inner1 && \
./scripts/prove-honk.sh inner2 && \
./scripts/prove-honk.sh outer
```

## Update LOCAL_SRS
Update `const LOCAL_SRS: &str = "transcript00.dat"` in `src/main.rs` to point to a local copy of [transcript00.dat](https://aztec-ignition.s3.eu-west-2.amazonaws.com/MAIN+IGNITION/sealed/transcript00.dat).

## Run tests
```sh
cargo test -- --test-threads=1
```

## Benchmarks

| Test Name         | Peak Memory (MB) | Proving Time (secs) |
|-------------------|------------------|---------------------|
| test_honk_inner1  | 660              | 0.84                |
| test_honk_inner2  | 570              | 0.73                |
| test_honk_outer   | 6904             | 21                  |

### Run benchmarks
```sh
./scripts/benchmark.sh
```
