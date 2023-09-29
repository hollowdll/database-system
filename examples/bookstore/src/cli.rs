use clap::{
    Parser,
    Subcommand,
    Args,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Book related features
    Book(BookArgs),
}

#[derive(Args)]
pub struct BookArgs {
    #[command(subcommand)]
    pub command: Option<BookCommands>,
}

#[derive(Subcommand)]
pub enum BookCommands {
    /// Add a book to the database
    Add(AddBookArgs),

    /// Find all books from the database
    FindAll(FindAllBooksArgs),

    /// Find a book from the database
    Find(FindBookArgs),
}

#[derive(Args)]
pub struct AddBookArgs {
    /// Name of the book
    #[arg(short, long)]
    pub name: String,

    /// Year published
    #[arg(short, long)]
    pub year: i32,

    /// Name of the author
    #[arg(short, long)]
    pub author: String,
}

#[derive(Args)]
pub struct FindAllBooksArgs {}

#[derive(Args)]
pub struct FindBookArgs {
    /// Id of the book
    #[arg(short, long)]
    pub id: u64,
}