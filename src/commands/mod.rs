use seahorse::{Context, Command};

pub mod screenplay;

pub fn default_action(c: &Context) {
    println!("Not found: {:?}", c.args);
}

// Just for testing 
fn add_action(c: &Context) {
    let sum: i32 = c.args.iter().map(|n| n.parse::<i32>().unwrap()).sum();
    println!("{}", sum);
}

pub fn add_command() -> Command {
    Command::new("add")
        .alias("a")
        .usage("rusties add(a****) [nums...]")
        .action(add_action)
}