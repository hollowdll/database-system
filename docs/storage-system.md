# Storage system

This documentation is still in planning phase. Not final.

Documentations and designs of how the engine stores data.

## Format

- Data will be stored in JSON format
- Data will be stored in .json files

## Storage

Each database will be stored in a separate database file. That file will contain all data of the database.

- Location: `databases` directory in project root
- Example file name: example-database.json

Optional design:

- Folder for each database
- Separate JSON files for info, tables, fields, data etc.

## Structure

- Size on disk
- Database name
- tables
- table fields
- stored data

## Parsing

Engine will parse data from Rust code to JSON format and vice versa.

## Storage library module

storage.rs

Will contain code to manipulate data.

- Read
- Add
- Update
- Delete

## Query API

No query language. Instead make API for different queries.


## Example file structure

TODO