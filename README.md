# Verifiability and Privacy in Federated Learning through Context-Hiding Multi-Key Homomorphic Authenticators

## Overview

This project is a **toy system for verifiable federated aggregation**, demonstrating how multiple parties — an
**aggregator** and several **clients** — can collaboratively train a global model while preserving trust and privacy.

## How It Works

- **Clients** train the model locally, authenticate and mask their parameters, and send them to the aggregator.
- The **aggregator** collects and aggregates the clients’ weights and authenticators to produce the final global model.
- Clients can optionally **verify** the resulting model to ensure the aggregation process was performed correctly and
  securely.

## Project Structure

```text
viper/
├── Cargo.toml
├── README.md                        - Project overview, architecture, and structure
├── LICENSE
├── src/
│   ├── lib.rs
│   ├── primitives/
│   │   └── mkhs.rs                  - Multi-Key Homomorphic Signatures (BLS12-381) with aggregation
│   ├── system/
│   │   ├── mod.rs
│   │   ├── client.rs                - Client entity: train, sign, verify
│   │   └── aggregator.rs            - Aggregator: aggregate parameters and signatures
│   └── util/
│       └── dataset.rs               - Synthetic dataset generator and conversions to ark_bls12_381::Fr
└── benches/
    ├── aggregator.rs                - Benchmark: aggregate signatures
    ├── client_c.rs                  - Benchmark: client-side signature computation
    ├── client_v.rs                  - Benchmark: client-side signature verification
    └── utils.rs                     - Benchmark helpers: parameters and randomized setup
```

## Requirements

The project requires:

- [Rust](https://www.rust-lang.org/tools/install) (stable version)

## Build & Run

This project has been developed and tested with Rust 1.86

To compile the project in release mode:

```bash
cargo build --release
```

## Benchmarking

Client-side signatures computation

```bash
cargo bench --bench client_c
```

Aggregator operations (signature aggregation)

```bash
cargo bench --bench aggregator
```

Client-side signatures verification

```bash
cargo bench --bench client_v
```

## Disclaimer

This is a research-grade implementation, meant for experimentation and educational use.
Not suitable for production deployment.

---

## Acknowledgements

This work was supported in part by project SERICS (PE00000014) under the NRRP MUR program funded by the EU - NGEU. Views
and opinions expressed are however those of the authors only and do not necessarily reflect those of the European Union
or the Italian MUR. Neither the European Union nor the Italian MUR can be held responsible for them.