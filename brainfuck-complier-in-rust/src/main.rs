use std::env;
use std::io::Read;
use std::fs::File;

#[derive(Debug)]
#[derive(Clone)]

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 2 {
        println!("usage: bf <file.bf>");
        std::process::exit(1);
    }

    let filename = &args[1];

    let mut file = File::open(filename).expect("file not found");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("failed to read file");

    let op = lex(src);
    let prog = parse(op);

    let mut tape: Vec<u8> = vec![0; 1024];
    let mut data_ptr = 512;
    run(&program, &mut tape, &mut data_ptr);
}
