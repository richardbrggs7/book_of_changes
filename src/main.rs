#[cfg(test)]
pub mod tests;

// use rand::distr::Uniform;
use rand::Rng;
use serde::{
    Deserialize,
    Serialize
};
use std::fs;
use toml;

#[derive(Deserialize, Serialize)]
struct Hex {
    utf : String,
    number : u16,
    lines : String,
    name : String,
    pinyin : String,
    preface : String,
    judgment : String,
    judgment_comm : String,
    image : String,
    image_comm : String,
    line_1 : String,
    line_1_comm : String,
    line_2 : String,
    line_2_comm : String,
    line_3 : String,
    line_3_comm : String,
    line_4 : String,
    line_4_comm : String,
    line_5 : String,
    line_5_comm : String,
    line_6 : String,
    line_6_comm : String
}

#[derive(Deserialize, Serialize)]
struct Changes {
    hexagram : Vec<Hex>
}

fn yarrow() -> u8 {
    let mut rng = rand::rng();
    let draw = rng.random_range(1..=16);
    if draw == 1 {
        6
    }
    else if draw > 1 && draw <= 8 {
        8
    }
    else if draw > 8 && draw <= 11 {
        9
    }
    else if draw > 11 {
        7
    }
    else {
        0
    }
}

fn make_hex() -> (String, String) {
    let mut lines : String = String::new();
    let mut moving : String = String::new();
    let mut line = yarrow();
    if line == 6 {
        lines.push('2');
        moving.push('1');
    }
    else if line == 8 {
        lines.push('2');
        moving.push('0');
    }
    else if line == 9 {
        lines.push('1');
        moving.push('1');
    }
    else {
        lines.push('1');
        moving.push('0');
    }
    line = yarrow();
    if line == 6 {
        lines.push('2');
        moving.push('1');
    }
    else if line == 8 {
        lines.push('2');
        moving.push('0');
    }
    else if line == 9 {
        lines.push('1');
        moving.push('1');
    }
    else {
        lines.push('1');
        moving.push('0');
    }
    line = yarrow();
    if line == 6 {
        lines.push('2');
        moving.push('1');
    }
    else if line == 8 {
        lines.push('2');
        moving.push('0');
    }
    else if line == 9 {
        lines.push('1');
        moving.push('1');
    }
    else {
        lines.push('1');
        moving.push('0');
    }
    line = yarrow();
    if line == 6 {
        lines.push('2');
        moving.push('1');
    }
    else if line == 8 {
        lines.push('2');
        moving.push('0');
    }
    else if line == 9 {
        lines.push('1');
        moving.push('1');
    }
    else {
        lines.push('1');
        moving.push('0');
    }
    line = yarrow();
    if line == 6 {
        lines.push('2');
        moving.push('1');
    }
    else if line == 8 {
        lines.push('2');
        moving.push('0');
    }
    else if line == 9 {
        lines.push('1');
        moving.push('1');
    }
    else {
        lines.push('1');
        moving.push('0');
    }
    line = yarrow();
    if line == 6 {
        lines.push('2');
        moving.push('1');
    }
    else if line == 8 {
        lines.push('2');
        moving.push('0');
    }
    else if line == 9 {
        lines.push('1');
        moving.push('1');
    }
    else {
        lines.push('1');
        moving.push('0');
    }
    (lines, moving)
}

fn main() {
    let book_str = fs::read_to_string("./translations/wilhelm_baynes.toml")
                     .expect("Could not open wilhelm_baynes.toml");
    let book : Changes = toml::from_str(&book_str).unwrap();

    // Construct the hexagram
    let lines : String;
    let moving : String;
    (lines, moving) = make_hex();
    let mut moving_str = String::new();
    if moving.chars().nth(0).unwrap() == '1' {
        moving_str.push_str("one");
    }
    if moving.chars().nth(1).unwrap() == '1' {
        if !moving_str.is_empty() {
            moving_str.push_str(", ");
        }
        moving_str.push_str("two");
    }
    if moving.chars().nth(2).unwrap() == '1' {
        if !moving_str.is_empty() {
            moving_str.push_str(", ");
        }
        moving_str.push_str("three");
    }
    if moving.chars().nth(3).unwrap() == '1' {
        if !moving_str.is_empty() {
            moving_str.push_str(", ");
        }
        moving_str.push_str("four");
    }
    if moving.chars().nth(4).unwrap() == '1' {
        if !moving_str.is_empty() {
            moving_str.push_str(", ");
        }
        moving_str.push_str("five");
    }
    if moving.chars().nth(5).unwrap() == '1' {
        if !moving_str.is_empty() {
            moving_str.push_str(", ");
        }
        moving_str.push_str("six");
    }
    if moving_str.is_empty() {
        moving_str.push_str("none");
    }
    println!("Lines: {}\nMoving: {}", lines, moving);

    // Find it
    let hex = book.hexagram.into_iter().find(|x| x.lines == lines)
                  .expect(&format!("Could not find hexagram matching lines {}",
                                   lines));

    // Display info
    println!("Hexagram number {}", hex.number);

    println!("{} ({}): {}", hex.utf, hex.pinyin, hex.name);
    println!("Moving lines: {}", moving_str);

}
