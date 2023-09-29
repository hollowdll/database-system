use driver::{
    client::error::DatabaseClientError,
    database::Database,
    collection::Collection,
    document::{
        DocumentModel,
        DataType,
    },
};

pub struct Book {
    pub document: DocumentModel,
}

impl Book {
    /// Creates a new book with name, publish year, and author.
    pub fn new(name: &str, year: i32, author: &str) -> Book {
        let mut document = DocumentModel::new();
        document.data.insert("name".to_string(), DataType::Text(name.to_string()));
        document.data.insert("year".to_string(), DataType::Int32(year));
        document.data.insert("author".to_string(), DataType::Text(author.to_string()));

        Book { document }
    }
}

/// Database context holding the collections of the book database.
pub struct BookDbContext<'a> {
    pub book_collection: Collection<'a>,
}

impl<'a> BookDbContext<'a> {
    pub fn build(db: &'a Database) -> Result<BookDbContext<'a>, DatabaseClientError> {
        Ok(BookDbContext { book_collection: db.get_collection("books")? })
    }
}