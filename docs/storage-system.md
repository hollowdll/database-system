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

## Structure

- Size on disk
- Database name
- Collections
- Document objects

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

## Example database file structure

Simple (previous)
```json
{
    "database_name": "testdb",
    "collections": [
        "collection_1": [
            {
                "document_id": 1,
                "document_field_1": "some data",
                "document_field_2": "some data",
            },
            {
                "document_id": 2,
                "document_field_1": "some data",
                "document_field_2": "some data",
                "document_field_3": "some data",
                "document_field_4": "some data",
            }
        ],
        "collection_2": [
            {
                "document_id": 1,
                "document_field_1": "some data",
                "document_field_2": "some data",
            },
            {
                "document_id": 2,
                "document_field_1": "some data",
                "document_field_2": "some data",
                "document_field_3": "some data",
                "document_field_4": "some data",
            }
        ],
    ]
}
```

Currently planned
```json
{
    "database_name": "testdb",
    "description": "Example database description.",
    "collections": [
        {
            "name": "test_collection_1",
            "id_count": 0,
            "documents": []
        },
        {
            "name": "test_collection_2",
            "id_count": 0,
            "documents": []
        },
    ]
}
```

## Example document structure

Currently planned
```json
{
    "id": 1,
    "data": {
        "key_1": "test_data",
        "key_2": "test_data",
        "key_3": "test_data",
    }
}
```
