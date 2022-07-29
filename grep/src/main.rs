use std::env;
#[allow(unused_imports)]
use std::option::Option;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let para = grep::Config::parse_config(&args);
    if !para.is_none() {
        if let Err(e) = grep::run(para.unwrap()) {
            println!("Can't find the file.");
        }
    } else {
        println!("Tip:The number of parameters does not match.");
        process::exit(1);
    }
}
