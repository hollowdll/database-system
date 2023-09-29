# Bookstore CLI program

This is a bookstore CLI program demonstrating the use of the database driver. The program uses a database that has a collection for books. You can add books and find them with commands.

The database file `bookdb.db` is created if it doesn't exist when you run the program. It is saved to the same directory as the binary. This directory is the database directory that the database client uses.

# Build the binary

To try it out
```bash
cargo run -- --help
```

Or build to the target directory
```bash
cargo build
```

# Commands

How to use
```bash
./bookstore --help
```

Add a new book
```bash
./bookstore book add --name <BOOK_NAME> --year <PUBLISH_YEAR> --author <AUTHOR_NAME>
```

Find all books
```bash
./bookstore book find-all
```

Find a book
```bash
./bookstore book find --id <BOOK_ID>
```

# Example

Add an example book to the database
```bash
./bookstore book add --name "Software Engineering" --year 2023 --author "John Smith"
```

Output
```json
{
  _id: 1
  name: Software Engineering
  year: 2023
  author: John Smith
}
```
