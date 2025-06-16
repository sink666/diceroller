use rand::prelude::*;

//
// roll sets of dice with dice notation and output rolls to a text file
//

enum DieType {
    D4, D6, D8, D10, D20
}

struct DiceRoll {
    dietype: DieType,
    count: u8
}

struct DiceVecs {
    d4: Vec<u8>,
    d6: Vec<u8>,
    d8: Vec<u8>,
    d10: Vec<u8>,
    d20: Vec<u8>
}

fn roll_parser(str: &str) -> DiceRoll {

}

fn do_rolls() {
    let mut rng = rand::rng();

    let d4: Vec<u8> = (1..4).collect();
    let d6: Vec<u8> = (1..6).collect();
    let d8: Vec<u8> = (1..8).collect();
    let d10: Vec<u8> = (1..10).collect();
    let d20: Vec<u8> = (1..20).collect();
}



fn main() {
    let mut instr = String::from("1d6 3d6 1d4 2d4 1d20 1d10");
    let roll_list: Vec<&str> = instr.split_whitespace().collect();
    let mut dice_parsed: Vec<DiceRoll> = Vec::new();

    for roll in roll_list {
        dice_parsed.push(
            roll_parser(roll)
        );
    }
}
