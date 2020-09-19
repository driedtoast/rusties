use seahorse::{Context, Command};
use fountain;
use fountain::data::{Document, Line};
use nom::error::{ErrorKind, VerboseError};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

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

// pub struct Dialog {
//     pub scene: Option<String>,
//     pub character: Option<String>,
//     pub text: Option<String>,
// }

// pub text: Vec<(String, String)>,

pub fn process_lines(document: &Document) {
    let mut characters = HashSet::new();
    let mut scenes = HashSet::new();
    // let mut dialogs = Vec<Dialog>::new();

    println!("line count is {}", document.lines.len());
    for line in &document.lines {
        println!("line is {}", line.is_speaker());
        match line {
            Line::Scene(s) => { scenes.insert(s.to_string()); },
            // Line::Action(s) => format!("<p class='action'>{}</p>", s),
            // Line::Dialogue(s) => { 
            //     dialogs.push(Dialog {
            //         text: s,
            //         character: characters.last(),

            //     })            
            // },
            Line::Speaker { name, is_dual: _ } => { characters.insert(name.to_string()); }
            _ => {}
            // Line::Parenthetical(s) => format!("<p class='parenthetical'>({})</p>", s),
            // Line::Transition(s) => format!("<p class='transition'>({})</p>", s),
            // Line::Lyric(s) => format!("<p class='lyric'>({})</p>", s),
        }
    }

    println!("Characters: ");
    for character in &characters {
        println!(" - {}", character);
    }

    println!("Scenes: ");
    for scene in &scenes {
        println!(" - {}", scene);
    }
}

pub fn get() -> Command {
    Command::new("screenplay")
        .alias("s")
        .usage("rusties screenplay(s****) [fountain file]")
        .action(screenplay_action)
}