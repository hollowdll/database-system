use bookstore::{
    create_db_client,
    get_bookstore_db,
    book::BookDbContext,
};

fn main() {
    let client = create_db_client();
    let book_db = get_bookstore_db(&client).unwrap();
    let book_db_context = BookDbContext::build(&book_db).unwrap();
}
