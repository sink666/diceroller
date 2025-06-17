use std::fmt;

use rand::prelude::*;

//
// roll sets of dice with dice notation and output rolls to a text file
//

struct DiceStruct {
    d4: [u8; 4],
    d6: [u8; 6],
    d8: [u8; 8],
    d10: [u8; 10],
    d20: [u8; 20],
}

const DICE_STRUCT: DiceStruct = DiceStruct {
    d4: [1, 2, 3, 4],
    d6: [1, 2, 3, 4, 5, 6],
    d8: [1, 2, 3, 4, 5, 6, 7, 8],
    d10: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    d20: [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ],
};

enum DieType {
    D4,
    D6,
    D8,
    D10,
    D20,
}

impl fmt::Display for DieType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DieType::D4 => write!(f, "d4"),
            DieType::D6 => write!(f, "d6"),
            DieType::D8 => write!(f, "d8"),
            DieType::D10 => write!(f, "d10"),
            DieType::D20 => write!(f, "d20"),
        }
    }
}

struct DiceRoll {
    dietype: DieType,
    num_rolls: u8,
    results: Vec<u8>,
}

impl DiceRoll {
    fn do_roll<R: Rng>(&mut self, rng: &mut R) {
        for _ in 0..self.num_rolls {
            self.results.push(match self.dietype {
                DieType::D4 => *DICE_STRUCT.d4.choose(rng).unwrap(),
                DieType::D6 => *DICE_STRUCT.d6.choose(rng).unwrap(),
                DieType::D8 => *DICE_STRUCT.d8.choose(rng).unwrap(),
                DieType::D10 => *DICE_STRUCT.d10.choose(rng).unwrap(),
                DieType::D20 => *DICE_STRUCT.d20.choose(rng).unwrap(),
            });
        }
    }
}

fn roll_parser(dstr: &str) -> DiceRoll {
    // number of dice, dice type
    let roll_tup: (u8, u8) = {
        let tmp1: Vec<u8> = dstr.split('d').map(|t| t.parse::<u8>().unwrap()).collect();
        (tmp1[0], tmp1[1])
    };

    DiceRoll {
        dietype: match roll_tup.1 {
            4 => DieType::D4,
            6 => DieType::D6,
            8 => DieType::D8,
            10 => DieType::D10,
            20 => DieType::D20,
            _ => panic!("unknown die type!!"),
        },
        num_rolls: match roll_tup.0 {
            0 => panic!("cannot roll zero dice!!"),
            _ => roll_tup.0,
        },
        results: Vec::new(),
    }
}

fn main() {
    let instr = String::from("20d6 20d4 20d20");
    let mut rng = rand::rng();
    let roll_list: Vec<&str> = instr.split_whitespace().collect();
    let mut dice_parsed: Vec<DiceRoll> = Vec::new();

    for roll_str in roll_list {
        dice_parsed.push(roll_parser(roll_str));
    }

    for roll in &mut dice_parsed {
        roll.do_roll(&mut rng);
    }

    for roll in dice_parsed {
        println!("{}{}: {:?}", roll.num_rolls, roll.dietype, roll.results);
    }
}
