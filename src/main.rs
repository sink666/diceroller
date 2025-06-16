use rand::prelude::*;

//
// roll sets of dice with dice notation and output rolls to a text file
//

enum DieType {
    D4,
    D6,
    D8,
    D10,
    D20,
}

struct DiceRoll {
    dietype: u8,
    num_rolls: u8,
    results: Vec<u8>,
}

impl DiceRoll {
    fn do_roll(&mut self) {
        // let mut rng = rand::rng();
        // let dx: Vec<u8> = (1..x).collect();
        // self.results.push(0)
    }
}

fn roll_parser(dstr: &str) -> DiceRoll {
    let roll_tup: (u8, u8) = {
        let tmp1: Vec<&str> = dstr.split('d').map(|t| { t.parse().unwrap}).collect();
        (tmp1[0].parse().unwrap(), tmp1[1].parse().unwrap())
    };
}

fn do_rolls() {}

fn main() {
    let mut instr = String::from("1d6 3d6 1d4 2d4 1d20 1d10");
    let roll_list: Vec<&str> = instr.split_whitespace().collect();
    let mut dice_parsed: Vec<DiceRoll> = Vec::new();

    for roll in roll_list {
        dice_parsed.push(roll_parser(roll));
    }
}
