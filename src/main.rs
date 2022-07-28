use std::fs;
use std::str::FromStr;

const MEMORY_INIT_ALLOCATE: usize = 1024;
const MEMORY_DYN_ALLOCATE: usize = 128;

enum Instruction {
    Right,
    Left,
    Increment,
    Decrement,
    Print,
    Read,
    BeginLoop(usize),
    EndLoop(usize),
}

type CompiledCode = Vec<Instruction>;

struct Interpreter {
    memory: Vec<u8>,
    memory_pointer: usize,
    code: CompiledCode,
    instruction_pointer: usize,
}

impl Interpreter {
    fn new(code: CompiledCode) -> Interpreter {
        Interpreter {
            memory: vec![0; MEMORY_INIT_ALLOCATE],
            memory_pointer: 0,
            code: code,
            instruction_pointer: 0,
        }
    }

    fn current_memory(&self) -> u8 {
        self.memory[self.memory_pointer]
    }

    fn next_instruction(&self) -> &Instruction {
        self.code.get(self.instruction_pointer).unwrap()
    }
}

fn right(context: &mut Interpreter) {
    context.memory_pointer += 1;
    if context.memory_pointer >= context.memory.len() {
        context.memory.resize(context.memory.len() + MEMORY_DYN_ALLOCATE, 0);
    }
}

fn left(context: &mut Interpreter) {
    if context.memory_pointer == 0 {
        panic!("Inaccessible memory");
    }
    context.memory_pointer -= 1;
}

fn increment(context: &mut Interpreter) {
    if context.current_memory() == 255 {
        context.memory[context.memory_pointer] = 0;
    } else {
        context.memory[context.memory_pointer] += 1;
    }
}

fn decrement(context: &mut Interpreter) {
    if context.current_memory() == 0 {
        context.memory[context.memory_pointer] = 255;
    } else {
        context.memory[context.memory_pointer] -= 1;
    }
}

fn input(context: &mut Interpreter) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    context.memory[context.memory_pointer] = input.trim().chars().next().expect("Expected ascii char") as u8;
}

fn output(context: &mut Interpreter) {
    print!("{}", context.current_memory() as char);
}


fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        println!("Usage:\n\t <code> \n\t -f <file> \n\t --file <file>");
        return;
    }
    let mut code;
    if args.len() >= 3 && (args[2] == "-f" || args[2] == "--file") {
        if args.len() == 3 {
            println!("No file specified");
            return;
        }
        let filename = &args[3];
        code = fs::read_to_string(filename).expect("Something went wrong reading the file");
    }
    else {
        code = String::from_str(&args[2..].join(" ")).expect("Enter valid code");
    }
    code.retain(|c| "<>+-.,[]".contains(c));

    let mut compiled: CompiledCode = Vec::new();
    let mut loop_stack = Vec::new();
    let mut index = 0;
    for char in code.chars() {
        index += 1;
        match char {
            '>' => compiled.push(Instruction::Right),
            '<' => compiled.push(Instruction::Left),
            '+' => compiled.push(Instruction::Increment),
            '-' => compiled.push(Instruction::Decrement),
            '.' => compiled.push(Instruction::Print),
            ',' => compiled.push(Instruction::Read),
            '[' => {
                compiled.push(Instruction::BeginLoop(0));
                loop_stack.push(index);
            },
            ']' => {
                let loop_start = loop_stack.pop().expect("Unmatched ]");
                compiled[loop_start - 1] = Instruction::BeginLoop(index);
                compiled.push(Instruction::EndLoop(loop_start));
            },
            _ => {index -= 1}
        }
    }
    if !loop_stack.is_empty() {
        panic!("Unmatched [");
    }

    let mut interpreter = Interpreter::new(compiled);

    while interpreter.instruction_pointer < interpreter.code.len() {
        match interpreter.next_instruction() {
            Instruction::Right => right(&mut interpreter),
            Instruction::Left => left(&mut interpreter),
            Instruction::Increment => increment(&mut interpreter),
            Instruction::Decrement => decrement(&mut interpreter),
            Instruction::Print => output(&mut interpreter),
            Instruction::Read => input(&mut interpreter),
            Instruction::BeginLoop(loop_end) => {
                if interpreter.current_memory() == 0 {
                    interpreter.instruction_pointer = loop_end - 0;
                }
            }
            Instruction::EndLoop(loop_start) => {
                if interpreter.current_memory() != 0 {
                    interpreter.instruction_pointer = loop_start - 1;
                }
            }
        }
        interpreter.instruction_pointer += 1;
    }

}
