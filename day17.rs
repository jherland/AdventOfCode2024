use std::collections::VecDeque;
use std::io;
use std::io::Read;

use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Adv(i64),
    Bxl(i64),
    Bst(i64),
    Jnz(i64),
    Bxc(i64),
    Out(i64),
    Bdv(i64),
    Cdv(i64),
}

use Instruction::*;

impl Instruction {
    fn parse(opcode: i64, operand: i64) -> Self {
        match opcode {
            0 => Adv(operand),
            1 => Bxl(operand),
            2 => Bst(operand),
            3 => Jnz(operand),
            4 => Bxc(operand),
            5 => Out(operand),
            6 => Bdv(operand),
            7 => Cdv(operand),
            _ => panic!("Invalid opcode"),
        }
    }
}

#[derive(Clone, Debug)]
struct Machine {
    a: i64,
    b: i64,
    c: i64,
    ip: usize,
    program: Vec<i64>,
    output: Vec<i64>,
}

impl Machine {
    fn parse(input: &str) -> Self {
        let re = Regex::new(r"Register A: (\d+)
Register B: (\d+)
Register C: (\d+)

Program: ([\d,]+)").unwrap();

        let caps = re.captures(input).unwrap();
        let a: i64 = caps.get(1).unwrap().as_str().parse().unwrap();
        let b: i64 = caps.get(2).unwrap().as_str().parse().unwrap();
        let c: i64 = caps.get(3).unwrap().as_str().parse().unwrap();
        let program = caps.get(4).unwrap().as_str()
            .split(",").map(|s| s.parse::<i64>().unwrap()).collect();
        Self { a, b, c, ip: 0, program, output: Vec::new() }
    }

    fn execute_one(&self) -> Self {
        let instr = Instruction::parse(self.program[self.ip], self.program[self.ip + 1]);
        let mut ip = self.ip + 2;
        // dbg!(&instr, ip);
        // dbg!(&self);
        let (mut a, mut b, mut c) = (self.a, self.b, self.c);
        let mut output = self.output.to_vec();

        let combo = |arg: i64| match arg {
            0 | 1 | 2 | 3 => arg,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("Invalid combo operand"),
        };
        let literal = |arg: i64| arg;

        match instr {
            Adv(arg) => a = a >> combo(arg),
            Bxl(arg) => b = b ^ literal(arg),
            Bst(arg) => b = combo(arg) % 8,
            Jnz(arg) => match a {
                0 => (),
                _ => ip = literal(arg) as usize,
            },
            Bxc(_arg) => b = b ^ c,
            Out(arg) => output.push(combo(arg) % 8),
            Bdv(arg) => b = a >> combo(arg),
            Cdv(arg) => c = a >> combo(arg),
        };
        // dbg!(instr, ip);
        Self { a, b, c, ip, program: self.program.to_vec(), output }
    }

    fn run_program(&self) -> Self {
        let mut ret = self.clone();
        while ret.ip < ret.program.len() {
            ret = ret.execute_one();
            // dbg!(&ret);
        }
        ret
    }

    fn update_a(&self, a: i64) -> Self {
        Self { a, program: self.program.to_vec(), output: Vec::new(), ..*self }
    }
}

fn byte_groups(n: i64) -> String {
    let mut v = Vec::new();
    let mut x = n;
    while x > 0 {
        v.push(format!("{:03b} ", x % 8));
        x /= 8;
    }
    v.reverse();
    v.join(" ")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let machine = Machine::parse(&input);
    // dbg!(&machine);

    let part1 = machine.run_program();
    // dbg!(&machine);
    println!("Part 1: {}", part1.output
        .iter().map(|n| n.to_string()).collect::<Vec<String>>().join(","));

    // See analysis below for how we calculate part 2
    // let mut a: i64 = 0;
    // for group in 1..=machine.program.len() {
    //     let bit_shift = 3 * (machine.program.len() - group);
    //     dbg!(group, bit_shift, a);
    //     for bits in 0b000..=0b111 {
    //         let test_a = a + (bits << bit_shift);
    //         let result = machine.update_a(test_a).run_program();
    //         println!("test_a: {}", byte_groups(test_a));
    //         println!("bits:   {:b}", bits);
    //         dbg!(group, test_a, &result);
    //         // Check resulting output against last 'group' ints in program
    //         let check_output: Vec<i64> = result.output.iter().rev().take(group).rev().cloned().collect();
    //         let expect_output = machine.program[machine.program.len() - group..].to_vec();
    //         assert_eq!(check_output[1..], expect_output[1..]);
    //         if check_output == expect_output {
    //             assert_eq!(check_output, expect_output);
    //             a = test_a;
    //             break;
    //         }
    //     }
    //     // *** TODO!: Need to turn this into a backtracking search?
    // }
    // ***
    // for group in 0..machine.program.len() {
    //     let bit_shift = 3 * group;
    //     dbg!(group, bit_shift, a);
    //     for bits in 0b001..=0b111 {
    //         let test_a = a | (bits << bit_shift);
    //         // if test_a == 0 {
    //         //     continue;
    //         // }
    //         let result = machine.update_a(test_a).run_program();
    //         println!("test_a: {}", byte_groups(test_a));
    //         println!("bits:   {:b}", bits);
    //         dbg!(group, test_a, &result);
    //         assert_eq!(result.output.len(), group + 1);
    //         // Check resulting output against first 'group' ints in program
    //         let expect_output = machine.program[0..group + 1].to_vec();
    //         assert_eq!(result.output[..group], expect_output[..group]);
    //         if result.output == expect_output {
    //             a = test_a;
    //             break;
    //         }
    //     }
    // }

    let mut candidates: VecDeque<i64> = (0..0b1_111_111_111)
        .filter(|a| machine.update_a(*a).run_program().output[0] == machine.program[0])
        .collect();
    for group in 1..machine.program.len() {
        
    }
    dbg!(&candidates, &candidates.len());

    // let part2 = machine.update_a(a).run_program();
    // assert_eq!(part2.program, part2.output);
    // println!("Part 2: {}", a);
}

// Register A: ???
// Register B: 0
// Register C: 0
// Program:
//      Bst 4
//      Bxl 5
//      Cdv 5
//      Bxl 6
//      Adv 3
//      Bxc 3
//      Out 5
//      Jnz 0
// Transcoded:
//  loop {
//      b = a % 8
//      b = b ^ 0b101
//      c = a / (1 << b)
//      b = b ^ 0b110
//      a /= 8
//      b = b ^ c
//      out(b % 8)
//      if a == 0 { break }
//  }
//  loop {
//      b = (a % 8) ^ 0b101
//      c = a >> b
//      b = b ^ 0b110
//      out((b ^ c) % 8)
//      a >>= 3
//      if a == 0 { break }
//  }
// Analysis:
//  - a is divided by 8 for each loop iteration
//  - One int is output for each loop iteration
//  - Program is 16 ints long, hence a must be >= 2^(15*3) and < 2^(16*3)
//  - Each iteration:
//      - b = lowest 3 bits in a, with first and third bits flipped
//          - a == 0b*000 -> b = 0b101 = 5
//          - a == 0b*001 -> b = 0b100 = 4
//          - a == 0b*010 -> b = 0b111 = 7
//          - a == 0b*011 -> b = 0b110 = 6
//          - a == 0b*100 -> b = 0b001 = 1
//          - a == 0b*101 -> b = 0b000 = 0
//          - a == 0b*110 -> b = 0b011 = 3
//          - a == 0b*111 -> b = 0b010 = 2
//      - c = a / 2^b - a divided by some power of 2 ==> right shift
//          - c = a >> b, lowest bits of c is taken from 0-7 higher bits in a 
//      - b ^= 0b110
//          - a == 0b*000 --> b = 0b011 = 3
//          - a == 0b*001 --> b = 0b010 = 2
//          - a == 0b*010 --> b = 0b001 = 1
//          - a == 0b*011 --> b = 0b000 = 0
//          - a == 0b*100 --> b = 0b111 = 7
//          - a == 0b*101 --> b = 0b110 = 6
//          - a == 0b*110 --> b = 0b101 = 5
//          - a == 0b*111 --> b = 0b100 = 4
//      - out((b ^ c) % 8) - output 3 lowest bits of c XORed with b
//          depends on result of complicated division above
//  - Each value that is output depends "mostly" on the lowest 3 bits of a,
//    but _really_ on the lowest 10 bits of a.
//  - We know that there are less that 16*3 bits in a, so the last ~4 values
//    output are bound by min(10, (16 - group) * 3)
//  - The _last_ value output is determined wholly by the top 3 bits of a.
//  - The plan:
//      - Count through the lowest 10 bits of a, keep all values that yield the
//        correct first output value. There should be ~127 in total.
//      - For each subsequent output value, vary the 3 next higher bits until
//        that output value matches. Assert that previous outputs are correct.
//      - Keep going until all output values are correct.
//      - Output minimum candidate value for a
