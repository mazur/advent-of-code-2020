pub fn run() {
    let acc = debug_program(include_str!("input.txt"));

    println!("Day08 - Part 1: {}", acc);
}

fn debug_program(program: &str) -> i32 {
    let mut inst = parse_program(program);
    let mut pc = 0;
    let mut acc: i32 = 0;

    loop {
        let mut curr = inst[pc];

        println!("{} {} {}", curr.0, curr.1, curr.2);

        if curr.2 {
            return acc;
        }

        curr.2 = true;
        inst[pc] = curr;

        match curr.0 {
            "acc" => acc += curr.1,
            "jmp" => pc = (pc as i32 + (curr.1 - 1)) as usize,
            _ => {} 
        }

        pc += 1;
    }
}

fn parse_program(program: &str) -> Vec<(&str, i32, bool)> {
    program.lines().map(|l| parse_line(l)).collect()
}

fn parse_line(line: &str) -> (&str, i32, bool) {
    let inst_arg: Vec<&str> = line.split(" ").collect();

    (inst_arg[0], inst_arg[1].parse().expect("arg not i32"), false)
}

#[cfg(test)]
mod tests {
    use super::*;

static TEST_INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_parse_line() {
        let inst = parse_line("acc -99");

        assert_eq!("acc", inst.0);
        assert_eq!(-99, inst.1);
        assert_eq!(false, inst.2);
    }

    #[test]
    fn test_parse_program() {
        let prog = parse_program(TEST_INPUT);

        assert_eq!(9, prog.len());
    }

    #[test]
    fn test_debug_program() {
        let acc = debug_program(TEST_INPUT);

        assert_eq!(5, acc);
    }
}
