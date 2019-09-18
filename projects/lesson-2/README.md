# Substrate Node Template

A new SRML-based Substrate node, ready for hacking.

## Build

```bash
cargo build --release
```
Log out:
```bash
$ cargo build  --release 
   Compiling substrate-kitties-runtime v2.0.0 (/home/flyq/workspaces/flyq/Team6/projects/lesson-2/runtime)
   Compiling substrate-kitties v2.0.0 (/home/flyq/workspaces/flyq/Team6/projects/lesson-2)
    Finished release [optimized] target(s) in 2m 51s
```


## Run

### Single local testnet

```bash
./target/release/substrate-kitties --dev 
```
