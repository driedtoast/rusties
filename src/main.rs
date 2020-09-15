use seahorse::{App};
use std::env;

mod hacker_rank;
mod commands;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("rusties [args]")
        .action(commands::default_action)
        .command(commands::add_command())
        .command(hacker_rank::command::get());
    app.run(args);
}