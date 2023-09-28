use driver::{
    client::error::DatabaseClientError,
    database::Database,
    collection::Collection, document::DocumentModel,
};

pub struct Book {
    pub document: DocumentModel,
}

impl Book {
    pub fn new() -> Book {
        Book { document: DocumentModel::new() }
    }
}

pub struct BookDbContext<'a> {
    pub book_collection: Collection<'a>,
}

impl<'a> BookDbContext<'a> {
    pub fn build(db: &'a Database) -> Result<BookDbContext<'a>, DatabaseClientError> {
        Ok(BookDbContext { book_collection: db.get_collection("books")? })
    }
}