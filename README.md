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

> **NOTE**
>
> Documentation is not final. This is still in early development.

# License

This project is licensed under MIT license. It is guaranteed to stay 100% free and open source.
