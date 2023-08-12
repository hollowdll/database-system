# Database CLI shell

There is an interactive CLI shell program which can be used to operate the database. Source code for it can be found in the `cli` crate in repository root.

The program can be used to do database operations, such as create databases, collections and documents. To access a created database, you need to connect to it using one of the connect commands.

There are two ways to connect to a database: using the name of the database in the database directory or by file path if the database locates somewhere else.

Databases created using the CLI will always be created to the database directory.

Command `/help` shows all the commands.


