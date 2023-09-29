use bookstore::{
    db::{
        create_db_client,
        get_bookstore_db,
    },
    document::{
        display_document,
        display_document_list,
    },
    cli::{
        Cli,
        Commands,
        BookCommands,
    },
    book::{BookDbContext, Book},
};
use driver::document::DocumentId;
use clap::Parser;

fn main() {
    let client = create_db_client();
    let book_db = get_bookstore_db(&client).unwrap();
    let book_db_context = BookDbContext::build(&book_db).unwrap();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Book(args)) => {
            match &args.command {
                Some(BookCommands::Add(args)) => {
                    let book = Book::new(&args.name, args.year, &args.author);
                    let inserted_book = book_db_context.book_collection
                        .insert_one(book.document)
                        .unwrap();

                    println!("Inserted book");
                    println!("-------------");
                    display_document(&inserted_book);
                },
                Some(BookCommands::FindAll(_args)) => {
                    let books = book_db_context.book_collection
                        .find_all()
                        .unwrap();

                    println!("Books found: {}", books.len());
                    display_document_list(&books);
                },
                Some(BookCommands::Find(args)) => {
                    let book = book_db_context.book_collection
                        .find_one_by_id(&DocumentId(args.id))
                        .unwrap();

                    if let Some(book) = book {
                        println!("Found book");
                        println!("----------");
                        display_document(&book);
                    } else {
                        println!("No book found");
                    }
                },
                None => return,
            }
        },
        None => return,
    }
}
