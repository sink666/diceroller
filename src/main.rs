use rand::prelude::*;
use std::{
    error, fmt,
    fmt::Write,
    num::{NonZeroU8, ParseIntError},
};

//
// dice roller
// takes space-seperated lines of dice notation and performs the requested rolls
// recieves input from stdin or from a file, outputs to stdout
// example usage:
// input: "1d6 2d4 3d10"
// output:
// 1d6: 3
// 2d4: 1 1
// 3d10: 9 3 3
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

#[derive(Debug)]
enum DiceErrorKind {
    InputSyntax,
    UnrecognizedDieType,
    ParseNumber(ParseIntError),
}

impl fmt::Display for DiceErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DiceErrorKind::InputSyntax => write!(f, "faulty dice syntax"),
            DiceErrorKind::UnrecognizedDieType => write!(f, "unrecognized dice type"),
            DiceErrorKind::ParseNumber(e) => write!(f, "parse number: {}", e),
        }
    }
}

impl error::Error for DiceErrorKind {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DiceErrorKind::InputSyntax => None,
            DiceErrorKind::UnrecognizedDieType => None,
            DiceErrorKind::ParseNumber(ref e) => Some(e),
        }
    }
}

impl From<ParseIntError> for DiceErrorKind {
    fn from(err: ParseIntError) -> DiceErrorKind {
        DiceErrorKind::ParseNumber(err)
    }
}

#[derive(Debug)]
struct DiceParseError {
    e_type: DiceErrorKind,
    context: String,
}

impl fmt::Display for DiceParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, at: {}", self.e_type, self.context)
    }
}

enum DieType {
    D4,
    D6,
    D8,
    D10,
    D20,
}

impl TryFrom<u8> for DieType {
    type Error = DiceParseError;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            4 => Ok(DieType::D4),
            6 => Ok(DieType::D6),
            8 => Ok(DieType::D8),
            10 => Ok(DieType::D10),
            20 => Ok(DieType::D20),
            _ => Err(DiceParseError {
                e_type: DiceErrorKind::UnrecognizedDieType,
                context: item.to_string(),
            }),
        }
    }
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

fn roll_parser(dstr: &str) -> Result<DiceRoll, DiceParseError> {
    // number of dice, dice type
    let roll_tup: (NonZeroU8, u8) = {
        let num = match dstr.split_once('d') {
            Some((a, b)) => Ok((a, b)),
            None => Err(DiceParseError {
                e_type: DiceErrorKind::InputSyntax,
                context: dstr.into(),
            }),
        }?;

        let lval = match num.0.parse::<NonZeroU8>() {
            Ok(r) => r,
            Err(e) => {
                return Err(DiceParseError {
                    e_type: DiceErrorKind::ParseNumber(e),
                    context: dstr.into(),
                });
            }
        };
        let rval = match num.1.parse::<u8>() {
            Ok(r) => r,
            Err(e) => {
                return Err(DiceParseError {
                    e_type: DiceErrorKind::ParseNumber(e),
                    context: dstr.into(),
                });
            }
        };

        (lval, rval)
    };

    Ok(DiceRoll {
        dietype: roll_tup.1.try_into()?,
        num_rolls: roll_tup.0.into(),
        results: Vec::new(),
    })
}

fn fmt_roll(rvec: &Vec<u8>) -> Result<String, fmt::Error> {
    let mut rollstr = String::new();
    let mut peekvec = rvec.iter().peekable();

    while let Some(num) = peekvec.next() {
        write!(&mut rollstr, "{num}")?;
        if peekvec.peek().is_some() {
            write!(&mut rollstr, " ")?;
        }
    }

    Ok(rollstr)
}

fn main() {
    let instr = String::from("1x6 0d4 1d6 1d4 3d6 2d20 -2d9 9d6 20d20");
    let mut rng = rand::rng();
    let roll_list: Vec<&str> = instr.split_whitespace().collect();
    let mut dice_parsed: Vec<DiceRoll> = Vec::new();

    for roll_str in roll_list {
        match roll_parser(roll_str) {
            Ok(x) => dice_parsed.push(x),
            Err(e) => println!("error: {e}. continuing..."),
        }
    }

    for roll in &mut dice_parsed {
        roll.do_roll(&mut rng);
    }

    for roll in dice_parsed {
        println!(
            "{}{}: {}",
            roll.num_rolls,
            roll.dietype,
            fmt_roll(&roll.results).unwrap()
        );
    }
}
