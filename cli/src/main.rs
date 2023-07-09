// Database engine CLI client

use cli::{
    Cli,
    run,
};

fn main() {
    let cli = Cli::build();
    run(cli);
}
