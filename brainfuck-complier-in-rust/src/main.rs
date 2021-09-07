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

fn parse (opcodes : Vec<OpCode>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (i, op) in opcodes.iter().enumerate() {
        if loop_stack == 0 {
            let instr = match op {
                OpCode::IncrementPointer => Some(Instruction::IncrementPointer),
                OpCode::DecrementPointer => Some(Instruction::DecrementPointer),
                OpCode::Increment => Some(Instruction::Increment),
                OpCode::Decrement => Some(Instruction::Decrement),
                OpCode::Write => Some(Instruction::Write),
                OpCode::Read => Some(Instruction::Read),

                OpCode::LoopBegin => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                },

                OpCode::LoopEnd => panic!("loop ending at #{} has no beginning", i),
            };

            match instr {
                Some(instr) => program.push(instr),
                None => ()
            }
        } else {
            match op {
                OpCode::LoopBegin => {
                    loop_stack += 1;
                },
                OpCode::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(Instruction::Loop(parse(opcodes[loop_start+1..i].to_vec())));
                    }
                },
                _ => (),
            }
        }
    }

    if loop_stack != 0 {
        panic!("loop that starts at #{} has no matching ending!", loop_start);
    }

    program
}

fn run(instructions: &Vec<Instruction>, tape: &mut Vec<u8>, data_ptr: &mut usize) {
    for instr in instructions {
        match instr {
            Instruction::IncrementPointer => *data_ptr += 1,
            Instruction::DecrementPointer => *data_ptr -= 1,
            Instruction::Increment => tape[*data_ptr] += 1,
            Instruction::Decrement => tape[*data_ptr] -= 1,
            Instruction::Write => print!("{}", tape[*data_ptr] as char),
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).expect("failed to read stdin");
                tape[*data_ptr] = input[0];
            },
            Instruction::Loop(nested_instructions) => {
                while tape[*data_ptr] != 0 {
                    run(&nested_instructions, tape, data_ptr)
                }
            }
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 2 {
        println!("usage: bf <file.bf>");
        std::process::exit(1);
    }

    let filename = &arguments[1];

    let mut file = File::open(filename).expect("file not found");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("failed to read file");

    let op = lex(src);
    let prog = parse(op);

    let mut tape: Vec<u8> = vec![0; 1024];
    let mut data_ptr = 512;
    run(&prog, &mut tape, &mut data_ptr);
}
