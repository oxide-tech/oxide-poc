# Oxide Protocol

Oxide is a peer-to-peer network created with Rust. This project focus is to
create and generate a library with tools for building a node for a peer-to-peer
network.

## Project Structure

The project crates:
 - `p2p` :: This crate is used to start an oxide node and accept and process
            incoming requests
    - `oxide-node` :: Is the bin for starting the oxide node
    - `oxide-client` TEST :: This is a cli for communicating with an oxide node

## Libraries Used

- `p2p` crate:
    - `tokio`
    - `serde`
    - `bytes`

# Testing

Run the following command to run all tests:
```bash
cargo test
```

# Starting the node server
Run the following command to start the oxide node:
```bash
cargo run --bin oxide-node
```

Run the following command to start the oxide client cli:
```bash
cargo run --bin oxide-client
```