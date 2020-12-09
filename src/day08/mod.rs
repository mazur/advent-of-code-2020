pub fn run() {
    let prog = parse_program(include_str!("input.txt"));
    let res = run_program(prog.clone());

    println!("Day08 - Part 1: {}", res.0);

    let correct_acc = find_and_run_correct_program(prog);

    println!("Day08 - Part 2: {}", correct_acc);
}

fn run_program(program: Vec<(&str, i32, bool)>) -> (i32, bool) {
    let mut inst = program;
    let mut pc = 0;
    let mut acc: i32 = 0;

    while pc < inst.len()  {
        let mut curr = inst[pc];

        if curr.2 {
            return (acc, false);
        }

        //println!("{} {} {}", curr.0, curr.1, curr.2);

        curr.2 = true;
        inst[pc] = curr;

        match curr.0 {
            "acc" => {
                acc += curr.1;
                pc += 1;
            },
            "jmp" => {
                pc = (pc as i32 + curr.1) as usize;
            },
            _ => pc += 1
        }
    }

    (acc, true)
}

fn mutate_program(program: Vec<(&str, i32, bool)>, inst: usize) -> (Vec<(&str, i32, bool)>, usize) {
    let mut copy = program.clone();
    let mut i = inst;

    loop {
        match copy[i].0 {
            "jmp" => {
                copy[i].0 = "nop";
                break;
            },
            "nop" => {
                copy[i].0 = "jmp";
                break;
            },
            _ => {}
        }
        i += 1;
    }

    (copy, i+1)
}

fn find_and_run_correct_program(program: Vec<(&str, i32, bool)>) -> i32 {
    let mut inc = 0;

    loop {
        let mutated = mutate_program(program.clone(), inc);
        inc = mutated.1;
        let res = run_program(mutated.0);
        if res.1 {
            return res.0;
        }
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

static BAD_PROGRAM: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

static GOOD_PROGRAM: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
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
        let prog = parse_program(BAD_PROGRAM);

        assert_eq!(9, prog.len());
    }

    #[test]
    fn test_run_bad_program() {
        let res = run_program(parse_program(BAD_PROGRAM));

        assert_eq!(5, res.0);
        assert_eq!(false, res.1);
    }

    #[test]
    fn test_run_good_program() {
        let res = run_program(parse_program(GOOD_PROGRAM));

        assert_eq!(8, res.0);
        assert_eq!(true, res.1);
    }

    #[test]
    fn test_mutate_program() {
        let prog = parse_program(BAD_PROGRAM);
        let mutated = mutate_program(prog, 0);

        assert_eq!(1, mutated.1);
        assert_eq!("jmp", mutated.0[0].0);
    }

    #[test]
    fn test_find_and_run_correct_program() {
        let prog = parse_program(BAD_PROGRAM);
        let acc = find_and_run_correct_program(prog);

        assert_eq!(8, acc);
    }
}
