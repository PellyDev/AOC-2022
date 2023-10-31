use y2022::new_line_parser;

#[derive(Debug)]
struct Instruction {
    cost: i32,
    adds: i32,
}
fn calculate_strength(i_vec: &Vec<Instruction>, max_cycles: i32) -> i32 {
    let mut cycles = 0;
    let mut register: i32 = 1;

    for inst in i_vec {
        cycles += inst.cost;
        if cycles >= max_cycles {
            break;
        }
        register += inst.adds;
    }
    register
}
pub fn solve() -> () {
    let mut data = new_line_parser(10);
    let instr_vec: Vec<Instruction> = data
        .into_iter()
        .map(|inst| {
            if inst.starts_with("noop") {
                Instruction { cost: 1, adds: 0 }
            } else {
                Instruction {
                    cost: 2,
                    adds: inst.split(" ").last().unwrap().parse().unwrap(),
                }
            }
        })
        .collect();
    let final_val = calculate_strength(&instr_vec, 20) * 20
        + calculate_strength(&instr_vec, 60) * 60
        + calculate_strength(&instr_vec, 100) * 100
        + calculate_strength(&instr_vec, 140) * 140
        + calculate_strength(&instr_vec, 180) * 180
        + calculate_strength(&instr_vec, 220) * 220;
    println!("{}", final_val);
}
