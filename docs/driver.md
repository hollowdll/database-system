# Database driver

There is a database driver for the Rust programming language. With the driver, a program can interact with the database engine to create and manage databases. This is a database client and an alternative to the database CLI shell. The database engine is embedded to the driver library.

The driver doesn't need a config file. Is is very minimal to setup in a program using it. You only need to build the database client with a directory path to your database directory. Databases will be created to this directory.

The driver consists of the following pieces:

- `Client`
- `Database`
- `Collection`
- `DocumentModel`

## Client

This is the database client, which communicates with the database engine. This is needed to connect to databases. Only one client is needed, but the client can be used to connect to multiple databases.

## Database

This is the database API. It provides functionality to get collections and database metadata. This stores the connection string to each database. Connection string is a file path to the database file.

## Collection

This is the collection API. It provides functionality to work with documents. For example, you can use this to insert documents to a collection and find all documents in the collection.

## DocumentModel

This is a data structure to manage database documents. With this, you can work with database documents.
