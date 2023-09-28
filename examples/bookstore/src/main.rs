use clap::{
    Parser,
    Subcommand,
    Args,
};
/*
use bookstore::{
    create_db_client,
    get_bookstore_db,
    book::BookDbContext,
};*/

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
    name: Option<String>,
}

fn main() {
    // let client = create_db_client();
    // let book_db = get_bookstore_db(&client).unwrap();
    // let book_db_context = BookDbContext::build(&book_db).unwrap();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Book(args)) => {
            match &args.command {
                Some(BookCommands::Add(args)) => {
                    if let Some(name) = &args.name {
                        println!("Book name: {}", name);
                    }
                },
                None => return,
            }
        },
        None => return,
    }
}
