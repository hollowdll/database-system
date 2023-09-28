use clap::{
    Parser,
    Subcommand,
    Args,
};

use bookstore::{
    create_db_client,
    get_bookstore_db,
    book::{BookDbContext, Book},
};

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
    /// Adds a book to the database to collection "books"
    Add(AddBook),
}

#[derive(Args)]
struct AddBook {
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

fn main() {
    let client = create_db_client();
    let book_db = get_bookstore_db(&client).unwrap();
    let book_db_context = BookDbContext::build(&book_db).unwrap();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Book(args)) => {
            match &args.command {
                Some(BookCommands::Add(args)) => {
                    println!("Name: {}", &args.name);
                    println!("Year: {}", &args.year);
                    println!("Author: {}", &args.author);

                    let book = Book::new(&args.name, args.year, &args.author);
                    let inserted_book = book_db_context.book_collection
                        .insert_one(book.document)
                        .unwrap();

                    println!("\nInserted document");
                    println!("-----------------");
                    println!("_id: {}", inserted_book.id());

                    for (key, value) in inserted_book.data {
                        println!("{}: {}", key, value);
                    }
                },
                None => return,
            }
        },
        None => return,
    }
}
