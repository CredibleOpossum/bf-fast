use std::str;

enum Instructions {
    PointerRight(usize),
    PointerLeft(usize),

    Add(u8),
    Sub(u8),

    PutChar,
    GetChar,

    LoopStart(usize),
    LoopEnd(usize),

    Clear,

    ScanLeft,
    ScanRight,
}

fn minify(source: &str) -> String {
    let mut minified = Vec::new();
    for character in source.chars() {
        match character {
            '>' | '<' | '+' | '-' | ',' | '.' | '[' | ']' => minified.push(character),
            _ => {}
        }
    }
    return minified.into_iter().collect();
}

fn optimize(source: &str) -> String {
    return source
        .replace("[-]", "c")
        .replace("[+]", "c")
        .replace("[<]", "l")
        .replace("[>]", "r");
}

fn compile(source: &str) -> Vec<Instructions> {
    let mut program = Vec::new();
    let characters = source.as_bytes();
    let mut position = 0;
    while position != characters.len() {
        let instruction = match characters[position] as char {
            '>' | '<' => {
                // This code compresses both pointer increase and decrease into one instruction
                let mut value: i32 = 0;
                match characters[position] as char {
                    '>' => value += 1,
                    '<' => value -= 1,
                    _ => panic!(),
                }
                while position < (characters.len() - 1) {
                    match characters[position + 1] as char {
                        '>' => value += 1,
                        '<' => value -= 1,
                        _ => break,
                    }
                    position += 1;
                }
                match value {
                    x if x > 0 => Instructions::PointerLeft(value as usize),
                    x if x < 0 => Instructions::PointerRight(value.abs() as usize),
                    _ => break,
                }
            }
            '+' | '-' => {
                // Same as before but with memory increase and decrease
                let mut value: i32 = 0;
                match characters[position] as char {
                    '+' => value += 1,
                    '-' => value -= 1,
                    _ => panic!(),
                }
                while position < (characters.len() - 1) {
                    match characters[position + 1] as char {
                        '+' => value += 1,
                        '-' => value -= 1,
                        _ => break,
                    }
                    position += 1;
                }
                if value > 0 {
                    Instructions::Add(value as u8)
                } else if value < 0 {
                    Instructions::Sub(value.abs() as u8)
                } else {
                    break;
                }
            }
            '.' => Instructions::PutChar, // These can be converted directly
            ',' => Instructions::GetChar,
            '[' => Instructions::LoopStart(0),
            ']' => Instructions::LoopEnd(0),
            'c' => Instructions::Clear,
            'l' => Instructions::ScanLeft,
            'r' => Instructions::ScanRight,
            _ => panic!(),
        };
        program.push(instruction);
        position += 1;
    }

    // Match brackets, doing this now makes runtime faster
    for position in 0..program.len() {
        match program[position] {
            Instructions::LoopStart(_) => {
                let mut seek = position;
                let mut unmatched = 1;
                while unmatched > 0 {
                    seek += 1;
                    match &program[seek] {
                        x if matches!(x, Instructions::LoopStart(_)) => {
                            unmatched += 1;
                        }
                        x if matches!(x, Instructions::LoopEnd(_)) => {
                            unmatched -= 1;
                        }
                        _ => continue,
                    }
                    program[position] = Instructions::LoopStart(seek);
                }
            }
            Instructions::LoopEnd(_) => {
                let mut seek = position;
                let mut unmatched = 1;
                while unmatched > 0 {
                    seek -= 1;
                    match &program[seek] {
                        x if matches!(x, Instructions::LoopStart(_)) => {
                            unmatched -= 1;
                        }
                        x if matches!(x, Instructions::LoopEnd(_)) => {
                            unmatched += 1;
                        }
                        _ => continue,
                    }
                    program[position] = Instructions::LoopEnd(seek);
                }
            }
            _ => continue,
        }
    }
    return program;
}

fn execute(program: &Vec<Instructions>, print_live: bool) -> String {
    let mut memory: [u8; 30_000] = [0; 30_000];
    let mut program_pos: usize = 0;
    let mut pointer_pos: usize = 0;
    let mut output = Vec::new();

    while program_pos != program.len() {
        match program[program_pos] {
            Instructions::PointerLeft(num) => pointer_pos += num,

            Instructions::PointerRight(num) => pointer_pos -= num,

            Instructions::Add(num) => {
                memory[pointer_pos] = memory[pointer_pos].wrapping_add(num);
            }

            Instructions::Sub(num) => {
                memory[pointer_pos] = memory[pointer_pos].wrapping_sub(num);
            }

            Instructions::LoopStart(pos) => {
                if memory[pointer_pos] == 0 {
                    program_pos = pos;
                }
            }

            Instructions::LoopEnd(pos) => {
                if memory[pointer_pos] != 0 {
                    program_pos = pos;
                }
            }

            Instructions::PutChar => {
                output.push(memory[pointer_pos]);
                if print_live {
                    print!("{}", memory[pointer_pos] as char);
                }
            }

            Instructions::GetChar => {
                panic!();
            }

            Instructions::Clear => {
                memory[pointer_pos] = 0;
            }

            Instructions::ScanRight => {
                while memory[pointer_pos] != 0 {
                    pointer_pos += 1;
                }
            }

            Instructions::ScanLeft => {
                while memory[pointer_pos] != 0 {
                    pointer_pos -= 1;
                }
            }
        }
        program_pos += 1;
    }
    return String::from_utf8(output).unwrap();
}

pub fn evaluate(source: &str, print_live: bool) -> String {
    return execute(&compile(&optimize(&minify(&source))), print_live);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn hello_world() {
        assert_eq!(evaluate("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.", false), "Hello World!\n");
    }
}
