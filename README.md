# Overview

Small document-oriented NoSQL database engine built with Rust.

No database server. The engine can be included as a library to a client application.

Databases are stored to single database files that contain everything the database needs.

Currently there is only a CLI client but a GUI client is being planned.

NOTE! This project is in early development and code will likely change a lot.
Currently documentation is very limited because things keep changing.

# Data storage

The engine uses Protocol Buffers for storing data.
Protocol Buffers is a very fast and efficient data format for serialized structured data.
It replaced the previously used JSON, as JSON wasn't very fast for large database files.

