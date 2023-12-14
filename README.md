# Overview

NoSQL document database system built with Rust.

No database server. The database engine is embedded to the application using it. The concept is similar to SQLite.

Databases are stored to local database files in binary format. The database engine uses Protocol Buffers data format to store data.

Currently there are 3 main components:
- `engine` - The database engine
- `shell` - The database shell
- `driver` - Database driver for Rust applications

# Documentation

For more information and to get started, read the [docs](./docs)

# Docker

The database shell can be used with Docker. [See instructions here](./docs/docker.md)

# Build

You can build from source code with Rust tools like Cargo. Because the engine uses Protocol Buffers, you need `protobuf` compiler on your system to compile the `.proto` files.

You can install protobuf compiler [here](https://github.com/protocolbuffers/protobuf#protobuf-compiler-installation)

Compile and run the database shell
```bash
cd database-system
```
```bash
cargo run
```

Or just compile without running
```bash
cargo build
```

Binaries, files, and directories will be created to `target` directory.

# Tests

Tests include unit tests and integration tests. They are written with Rust's built-in tooling.

Run all tests and get result on each test.
```bash
cd database-system
```
```bash
cargo test
```

# License

This project is licensed under MIT license. It is free and open source.
