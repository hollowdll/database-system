# Overview

Small NoSQL document database system built with Rust.

No database server. The database engine is embedded to the application using it. If you are familiar with SQLite, the concept is similar to it.

Databases are stored to local database files in binary format. The database engine uses Protocol Buffers data format to store data.

Currently there are 3 main components:
- `engine` - The database engine
- `shell` - The database shell
- `driver` - Database driver to use in Rust applications

No releases yet.

# Documentation

For more information and to get started, read the [docs](./docs)

# Docker

The easiest way to get started is using Docker. [See instructions here](./docs/docker.md)

# Build

Alternatively, you can build from source code with Rust tools like Cargo. Because the engine uses Protocol Buffers, you need `protoc` compiler on your system to compile the `.proto` files. This is also needed when using the database driver.

You can install protobuf compiler [here](https://github.com/protocolbuffers/protobuf#protobuf-compiler-installation)

Compile and run the database shell
```
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

# License

This project is licensed under MIT license. It is guaranteed to stay 100% free and open source.
