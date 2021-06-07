use seahorse::{Context, Command};
use crate::leetcode::*;

fn leetcode_action(c: &Context) {
    if c.args.is_empty() {
        problem_not_found();
        return;    
    }
    let first_arg = c.args.first();
    match first_arg.unwrap().as_str() {
        "rotate_array" => rotate_array::run(&c.args[1..c.args.len()]),
        "search_sorted_array" => search_sorted_array::run(&c.args[1..c.args.len()]),
        _ => {
            println!("Problem not found {:?}", c.args.first().unwrap());
            problem_not_found();
        },
    }    
}

fn problem_not_found() {
    println!("try one of the following:");
    println!("-> rotate_array <num to rotate>");
}

pub fn get() -> Command {
    Command::new("leetcode")
        .alias("l")
        .usage("rusties leetcode(h****) [problem name]")
        .action(leetcode_action)
}
