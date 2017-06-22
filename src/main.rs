use std::io::{stdout, Write};

fn main() {
    let mut data = [0;256];
    let mut data_ptr = 0;
    let program = ",>,[<+>-]<.".as_bytes();
    let mut loop_stack = Vec::new();

    let mut program_ptr = 0;
    while program_ptr < program.len() {
        match program[program_ptr] as char {
            '+' => data[data_ptr] += 1,
            '-' => data[data_ptr] -= 1,
            '>' => data_ptr += 1,
            '<' => data_ptr -= 1,
            '[' if data[data_ptr] == 0 => { // find corresponding closing bracket
                let mut open_brackets = 0;
                program_ptr += 1;
                loop {
                    match program[program_ptr] as char {
                        '[' => open_brackets += 1,
                        ']' if open_brackets == 0 => break,
                        ']' => open_brackets -= 1,
                        _ => {},
                    }
                    program_ptr += 1;
                }
                },
            '[' => loop_stack.push(program_ptr),
            ']' if data[data_ptr] == 0 => {loop_stack.pop();},
            ']' => program_ptr = *loop_stack.last().unwrap(),
            '.' => println!("{}", data[data_ptr]),
            ',' => {
                print!("enter number> ");
                stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                data[data_ptr] = input.trim().parse::<i32>().unwrap();
            }
            _ => println!("unexpected command")
        }
        program_ptr+=1;
    }
}
