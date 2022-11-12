
# Rust Key Stretcher
## Tool created with the goal of transforming a weak key in a strong key, through expensive CPU and memory process.
## DISCLAIMER: I'm not a cryptographer, use it at your own risk.


### The logic behind the process is basically the following:
* Hash the input using SHA-512
* Append the hash bytes into a vector (acumulator) with defined max size
* Loop until the acumulator is completely filled
    * Hash the acumulator and append the hash bytes
    * Duplicate the bytes of the acumulator to increase memory usage
* Hash the acumulator
* Hash the hash of the acumulator 100 times, resulting in the final stretched key

## This process is slow, this is good, brute force attempts will run slowly too.

### Build the source:
````
cargo build --release
````

### Run the tool:
````
./target/release/key_stretcher
````

### Run the tool with debug enabled:
````
RUST_LOG=debug ./target/release/key_stretcher
````