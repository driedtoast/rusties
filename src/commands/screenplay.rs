use seahorse::{Context, Command};
use fountain;
use fountain::data::{Document, Line};
use nom::error::{ErrorKind, VerboseError};
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn screenplay_action(c: &Context) {
    if c.args.is_empty() {
        println!("screen file not found");
        return;    
    }
    let first_arg = c.args.first();

    // match first_arg.unwrap().as_str() {
    //     "scenes" => run::run(&c.args[1..c.args.len()]),
    //     "characters" => run::run(&c.args[1..c.args.len()]),
    //     _ => {
    //         println!("Problem not found {:?}", c.args.first().unwrap());
    //         problem_not_found();
    //     },
    // }    
    run(&first_arg.unwrap());
}

// Read a file's contents into a string
fn read(filepath: &String) -> Result<String, io::Error> {
    let mut f = File::open(filepath.as_str())?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn run(arg: &String) {

    let text = read(arg);

    let file_text = text.unwrap();
    let parse_result = fountain::parse_document::<VerboseError<&str>>(&file_text);
    
    match parse_result {
        Err(e) => eprintln!("Error while parsing the screenplay: {:?}", e),
        Ok(("", parsed)) => {
            println!("Successfully parsed the document");
            // println!("{}", parsed.as_html());
            process_lines(&parsed);            
        }
        Ok((unparsed, parsed)) => {
            eprintln!("Couldn't parse the entire document. Unparsed section:");
            eprintln!("{}", unparsed);
            process_lines(&parsed);
        }
    }

}

pub fn process_lines(document: &Document) {
    println!("line count is {}", document.lines.len());
    for line in &document.lines {
        println!("line is {}", line.is_scene());
    }
}

pub fn get() -> Command {
    Command::new("screenplay")
        .alias("s")
        .usage("rusties screenplay(s****) [fountain file]")
        .action(screenplay_action)
}