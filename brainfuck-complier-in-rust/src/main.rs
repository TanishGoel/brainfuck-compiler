use std::env;
use std::io::Read;
use std::fs::File;

#[derive(Debug)]
#[derive(Clone)]
enum OpCode {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Read,
    Write,
    LoopBegin,
    LoopEnd,
}

#[derive(Debug)]
#[derive(Clone)]
enum Instruction {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Write,
    Read,
    Loop(Vec<Instruction>)
}

fn lex(src: String) -> Vec<OpCode> {
    let mut operations = Vec::new();

    for symbol in src.chars() {
        let op = match symbol {
            '>' => Some(OpCode::IncrementPointer),
            '<' => Some(OpCode::DecrementPointer),
            '+' => Some(OpCode::Increment),
            '-' => Some(OpCode::Decrement),
            '.' => Some(OpCode::Write),
            ',' => Some(OpCode::Read),
            '[' => Some(OpCode::LoopBegin),
            ']' => Some(OpCode::LoopEnd),
            _ => None
        };

        match op {
            Some(op) => operations.push(op),
            None => ()
        }
    }
    operations
}

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
