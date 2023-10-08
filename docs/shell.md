# Database shell

There is an interactive CLI shell program which can be used to interact with the database. Source code for it can be found in the `shell` crate in repository root.

The program can be used to do database operations, such as create databases, collections and documents. To access a created database, you need to connect to it using one of the connect commands.

There are two ways to connect to a database:
- Use the name of the database in the database directory
- Use the database file path if the database exists somewhere outside the database directory

Databases created using the shell will always be created to the database directory.

Command `/help` shows all the commands.
