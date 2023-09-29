use clap::{
    Parser,
    Subcommand,
    Args,
};
use bookstore::{
    create_db_client,
    get_bookstore_db,
    document::{
        display_document,
        display_document_list,
    },
    book::{BookDbContext, Book},
};
use driver::document::DocumentId;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Book related features
    Book(BookArgs),
}

#[derive(Args)]
struct BookArgs {
    #[command(subcommand)]
    command: Option<BookCommands>,
}

#[derive(Subcommand)]
enum BookCommands {
    /// Add a book to the database
    Add(AddBookArgs),
    /// Find all books from the database
    FindAll(FindAllBooksArgs),
    /// Find a book from the database
    Find(FindBookArgs),
}

#[derive(Args)]
struct AddBookArgs {
    /// Name of the book
    #[arg(short, long)]
    name: String,

    /// Year published
    #[arg(short, long)]
    year: i32,

    /// Name of the author
    #[arg(short, long)]
    author: String,
}

#[derive(Args)]
struct FindAllBooksArgs {}

#[derive(Args)]
struct FindBookArgs {
    /// Id of the book
    #[arg(short, long)]
    id: u64,
}

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
