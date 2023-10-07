# Database data storage

The database engine uses Protocol Buffers to store data. Protocol Buffers is a fast and efficient data format for serializing structured data. It can be encoded to binary, making it much faster and smaller than something like JSON. The data has a schema in .proto files and can be compiled to programming language data structures. The speed, small size and efficiency make it optimal format for storing data.

The created databases are stored to database files that have file format `.db`. The name of the database file without file format is the name of the database.

Each database file contains a single database in binary format. The binary format can be read by the engine and decoded to data structures. If the content of the file has been changed and is invalid, protobuf decode will fail.

Databases are stored to a database directory when they are created. The database directory is configured by the engine, but the configuration can be changed.

To read the data, the database file path is passed to the engine in requests that need it.

When a database needs to be updated, the engine will overwrite the database file content with the updated database data.
