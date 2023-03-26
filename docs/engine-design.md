# Engine design

This documentation is still in planning phase. Not final.

Documentations and designs for the engine.

## About

Small Document-oriented NoSQL database.

Uses key-value pairs for data.

No database server. Data is stored in JSON files.

- Storage system
- Data retrieval system
- Query API
- Data encoding from Rust code to JSON
- Database event logs
- Connections

## Structure

Currently planned structure. Not final.

<pre>
+-Client/management system
    +-Database manager
        +-Database 1
            +-Collections
                +-Collection 1
                    +-Document 1
                    +-Document 2
                    +-Document 3
                +-Collection 2
                    +-Document 1
                    +-Document 2
                    +-Document 3
        +-Database 2
            +-Collections
                +-Collection 1
                    +-Document 1
                    +-Document 2
                    +-Document 3
                +-Collection 2
                    +-Document 1
                    +-Document 2
                    +-Document 3
</pre>

## Connections

<pre>
Client/management system
    -> Access databases

Database manager
    -> Connect database
        -> Access collections and documents
</pre>

## Documents

Document data can be different in each document. They don't need to have the same schema.

Each document has a unique ID identifying the document in the collection it is in.

For example

<pre>
Example collection
    Document 1
        ID - 1
        field 1 - some data
        field 2 - some data

    Document 2
        ID - 2
        field 1 - some data
        field 2 - some data
        field 3 - some data
        field 4 - some data
</pre>



