use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App, Arg};

// Run with this command:
// `cargo run -- pattern_to_search path_to_file`
// Where
// pattern_to_search is the pattern to search for.
// path_to_file is the path of file where to search for.

fn main() {
    // Build a command argument parser with only one of it
    let args = App::new("grep-lite")
    .version("0.1")
    .about("Search for patterns")
    .arg(Arg::with_name("pattern")
        .help("The pattern to search for")
        .takes_value(true)
        .required(true))
    .arg(Arg::with_name("input")
        .help("File to search")
        .takes_value(true)
        .required(true))
    .get_matches();

    // Extract pattern argument
    let pattern = args.value_of("pattern").unwrap();

    let re = Regex::new(pattern).unwrap(); // unwraps a Result or crashes if an error occurs.
    
    let input = args.value_of("input").unwrap();
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines(){
        let line = line_.unwrap(); // unwrap at the risk of crashing if an error occurs
        match re.find(&line){
            Some(_) => print!("{}", line),
            None => (),
        }
    }
}

fn find_with_command_args(){

    // Build a command argument parser with only one of it
    let args = App::new("grep-lite")
    .version("0.1")
    .about("Search for patterns")
    .arg(Arg::with_name("pattern")
        .help("The pattern to search for")
        .takes_value(true)
        .required(true))
    .get_matches();

    // Extract pattern argument
    let pattern = args.value_of("pattern").unwrap();

    let re = Regex::new(pattern).unwrap(); // unwraps a Result or crashes if an error occurs.

    // let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, public-house, and 
    dark square in a picture, feverishly turned-in search of what?
    It is the same with books. What do you seek through millions of pages?";

    for line in quote.lines(){
        let contains_substring = re.find(line);
        match contains_substring{
            Some(_) => println!("{}", line),
            None => (), // `()` can be though of as a null placeholder
        }
    }
}

fn find_with_regex(){

    let re = Regex::new("picture").unwrap(); // unwraps a Result or crashes if an error occurs.

    // let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, public-house, and 
    dark square in a picture, feverishly turned-in search of what?
    It is the same with books. What do you seek through millions of pages?";

    for line in quote.lines(){
        let contains_substring = re.find(line);
        match contains_substring{
            Some(_) => println!("{}", line),
            None => (), // `()` can be though of as a null placeholder
        }
    }
}


// Store n lines of context around a match, for instance:
// If we have a text of 5 lines and the match is at line 2
// then store line 1, 3 and 4 (line 2 is also included in results)
fn find_with_context(){
    let ctx_lines = 2;
    let needle =  "oo";
    let haystack = "\
Every fave, every shop,
bedroom window, public-house, and
dark square in a picture,
feverishly turned-in search of what?
It is the same with books.
What do you seek
through millions of pages?";

    // Holds line numbers where matches occur
    let mut tags:Vec<usize> = vec![]; 

    // Holds each context lines in a vector
    let mut ctx:Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate(){
        if line.contains(needle){
            tags.push(i);
        }
        
        // No explicit type signature is needed as it can be inferred 
        // via the definition of `ctx` variable
        let v = Vec::with_capacity(2*ctx_lines + 1);
        // Vec<T> performs best when a size hint is provided

        ctx.push(v);
    }

    if tags.is_empty(){
        return;
    }

    for(i, line) in haystack.lines().enumerate(){
        for(j,tag) in tags.iter().enumerate(){

            // Subtraction that returns 0 on integer underflow rather than
            // crashing the program (CPUs don't like send usize below zero)
            let lower_bound = tag.saturating_sub(ctx_lines);

            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i<=upper_bound ){
                // Copy `line` into a new String
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter(){

        // `ref line` informs the compiler that we want to borrow this value 
        // rather than move it
        for &(i, ref line) in local_ctx.iter(){
            let line_num = i+1;
            println!("{}: {}",line_num, line);
        }
    }

}