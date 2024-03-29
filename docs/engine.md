# Database engine

This is a NoSQL document database. This means it doesn't use SQL or traditional relational database tables. It doesn't have relationships or constraints between data entities. Because it is a document database, it is designed to be dynamic, flexible and close to program logic. Currently everything is very minimal.

Data querying happens with functions. Currently data querying is very limited but there will be more features in future versions.

There is no database server. The database engine can be used by a database client by including it as a library. Currently the engine can be used by Rust clients only. To use the engine, reference the `engine` crate in another crate's `Cargo.toml` under dependencies.

There is also no asynchronous runtime, but this might be changed in the future.

The engine handles data storage and database operations for databases, collections and documents.

The engine has configurations for database directory and logs directory. The default values for these can be changed by a database client. Database directory is where databases are created by default when no explicit database file path is given. Logs directory is where all the log files are created.

There are separate APIs for all major features. Currently there is storage API for database operations and config API to set and get engine configurations.
