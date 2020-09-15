use seahorse::{Context, Command};
use crate::hacker_rank::*;

fn hacker_rank_action(c: &Context) {
    if c.args.is_empty() {
        problem_not_found();
        return;    
    }
    let first_arg = c.args.first();
    match first_arg.unwrap().as_str() {
        "cat_and_mouse" => cat_and_mouse::run(&c.args[1..c.args.len()]),
        _ => {
            println!("Problem not found {:?}", c.args.first().unwrap());
            problem_not_found();
        },
    }    
}

fn problem_not_found() {
    println!("try one of the following:");
    println!("-> cat_and_mouse <cat a> <cat b> <cat c>");
}

pub fn get() -> Command {
    Command::new("hackerrank")
        .alias("h")
        .usage("rusties hackerrank(h****) [problem name]")
        .action(hacker_rank_action)
}
