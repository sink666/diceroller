use rand::prelude::*;

//
// roll sets of dice with dice notation and output rolls to a text file
//
//
//

struct DiceRoll {
    dietype: u8,
    num_rolls: u8,
    results: Vec<u8>,
}

impl DiceRoll {
    fn do_roll<R: Rng>(&mut self, rng: &mut R) {
        let dx: Vec<u8> = (1..self.dietype).collect();
        for _ in 0..self.num_rolls {
            self.results.push(*dx.choose(rng).unwrap());
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
            4 | 6 | 8 | 10 | 20 => roll_tup.1,
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
    let instr = String::from("1d6 3d6 1d4");
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
        println!("{}d{}: {:?}", roll.num_rolls, roll.dietype, roll.results);
    }
}
