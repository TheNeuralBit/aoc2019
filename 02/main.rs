fn main() {
    let input = include_str!("input");

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program: Vec<usize> = input.split(|c| c=='\n' || c == ',')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            program[1] = noun;
            program[2] = verb;

            println!("Initial program: {:?}", program);

            let mut pc: usize = 0;
            loop {
                if pc > program.len() {
                    panic!("pc ({}) is greater than the length of the program ({})!", pc, program.len())
                }
                let opcode = program[pc];
                let left_pos = program[pc+1];
                let right_pos = program[pc+2];
                let out_pos = program[pc+3];
                println!("{}, {}, {}, {}", opcode, left_pos, right_pos, out_pos);
                match opcode {
                    1 => {
                        program[out_pos] = program[left_pos] + program[right_pos];
                    },
                    2 => {
                        program[out_pos] = program[left_pos] * program[right_pos];
                    },
                    99 => break,
                    _ => panic!("Encountered unexpected opcode: {}", opcode),
                }
                pc += 4;
            }

            println!("N: {}, V: {}", noun, verb);
            println!("Final program: {:?}", program);
            if program[0] == 19690720 {
                panic!("Done!");
            }
        }
    }
}
