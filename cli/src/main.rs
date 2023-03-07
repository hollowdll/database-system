// Entry point to database via terminal
// Under development!
// Currently in prototyping phase.
// Code will be improved later.

fn main() {
    let config = cli::Config::build();
    cli::run(config);
}
