use std::env;
use text_colorizer::*;


#[derive(Debug)]  // 编译器生成额外的代码，可以直接打印：println!("{:?}", arguments)
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!("{} - change occurrences of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>")
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4, got {}.", "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    // 返回值
    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn main() {
    let args = parse_args();
    println!("{:?}", args);
}


