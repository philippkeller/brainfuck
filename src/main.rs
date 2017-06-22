use std::io::{stdout, Write};

fn main() {
    let mut data = [0;256];
    let mut data_ptr = 0;
    let program = ",>,[<+>-]<.".to_string();
    let mut loop_stack = Vec::new();

    let mut program_ptr = 0;
    while program_ptr < program.len() {
        match (program.chars().nth(program_ptr).unwrap(), data[data_ptr]) {
            ('+', _) => data[data_ptr] += 1,
            ('-', _) => data[data_ptr] -= 1,
            ('>', _) => data_ptr += 1,
            ('<', _) => data_ptr -= 1,
            ('[', 0) => { // find corresponding closing bracket
                let mut open_brackets = 0;
                program_ptr += 1;
                loop {
                    match (program.chars().nth(program_ptr).unwrap(), open_brackets) {
                        ('[', _) => open_brackets += 1,
                        (']', 0) => break,
                        (']', _) => open_brackets -= 1,
                        _ => {},
                    }
                    program_ptr += 1;
                }
                },
            ('[', _) => loop_stack.push(program_ptr),
            (']', 0) => {loop_stack.pop();},
            (']', _) => program_ptr = *loop_stack.last().unwrap(),
            ('.', _) => println!("{}", data[data_ptr]),
            (',', _) => {
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
