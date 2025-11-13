#[cfg(test)]
pub mod tests;

use clap::Parser;
use inquire::{
    Confirm,
    MultiSelect,
};
use rand::Rng;
use serde::{
    Deserialize,
    Serialize
};
use std::{
    cmp::max,
    cmp::min,
    fs
};
use term_size::dimensions;

// Options for the hexagram display menu
const OPTIONS : [&str; 10] = ["Commentaries", "Preface", "Judgment", "Image",
                              "Line 1", "Line 2", "Line 3", "Line 4", "Line 5",
                              "Line 6"];

#[derive(Parser)]
#[command(about = "Book of Changes hexagram indexer and divination assistance \
                   tool")]
struct Args {
    #[arg(short = 'p', long = "prompt", default_value_t = false, help = "Whether \
            to prompt the user to continue with each section of the hexagram")]
    prompt : bool,

    #[arg(short = 'x', long = "hex", default_value_t = 0, help = "Go directly \
            to this hexagram")]
    hex : u8,

    #[arg(short = 'l', long = "list", default_value_t = false, help = "List all \
            hexagrams")]
    list : bool,
}

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

fn print_within(text : &str, size : Option<i16>) {
    // Print either 'size' columns, or the min between the dimensions of the
    // terminal and 80, but at least 20
    let mut size = if let Some(num) = size {
        num
    }
    else {
        let (w, _) = dimensions().unwrap_or((40, 40));
        min(w, 80).try_into().unwrap()
    };
    size = max(size, 20) - 1;

    // Iterate over the given text and construct a new string, inserting
    // newlines and hyphens when appropriate.
    let mut new_text = String::new();
    let mut count : i16 = 0;
    let text_chars : Vec<_> = text.chars().collect();
    for (i, ch) in text.chars().enumerate() {
        if ch == '\n' {
            count = 0;
            new_text.push(ch);
            continue;
        }
        if count == size - 1 {
            if ch.is_whitespace() {
                count = 0;
                new_text.push('\n');
                continue;
            }
            else if text_chars[i-1].is_whitespace() {
                new_text.push('\n');
                new_text.push(ch);
                count = 1;
                continue;
            }
            else if ch!='.' && ch!=',' && ch!=';' && ch!='-' && ch!='"'
                            && !text_chars[i+1].is_whitespace() {
                new_text.push('-');
                new_text.push('\n');
                new_text.push(ch);
                count = 1;
                continue;
            }
            else {
                new_text.push(ch);
                new_text.push('\n');
                count = 0;
                continue;
            }
        }
        if count == 0 && ch.is_whitespace() {
            continue;
        }
        new_text.push(ch);
        count += 1;
    }
    print!("{new_text}");
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

fn make_hex() -> (String, String, String) {
    let mut lines : String = String::new();
    let mut moving : String = String::new();
    let mut large_hex : String = String::new();
    let mut line : u8;
    for _ in 0..=5 {
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
    }

    for i in [5, 4, 3, 2, 1, 0].into_iter() {
        if lines.chars().nth(i).unwrap() == '1'
        {
            large_hex.push_str("_________\n");
        }
        else {
            large_hex.push_str("____ ____\n");
        }
    }
    (lines, moving, large_hex)
}

fn main() {
    let cli = Args::parse();
    let book_str = fs::read_to_string("./translations/wilhelm_baynes.toml")
                     .expect("Could not open wilhelm_baynes.toml");
    let book : Changes = toml::from_str(&book_str).unwrap();

    if cli.list {
        for hex in book.hexagram {
            println!("{} {}: {}", hex.utf, hex.number, hex.name);
        }
        return;
    }

    let moving : String;
    let large_hex : String;
    let mut moving_str = String::new();
    let hex = if cli.hex == 0 {
        // Construct the hexagram
        let lines : String;
        (lines, moving, large_hex) = make_hex();

        // String showing which lines are moving
        for (digit, num) in [(0,"one"), (1,"two"), (2,"three"), (3,"four"),
                             (4,"five"), (5,"six")].into_iter() {
            if moving.chars().nth(digit).unwrap() == '1' {
                if !moving_str.is_empty() {
                    moving_str.push_str(", ");
                }
                moving_str.push_str(num);
            }
        }
        if moving_str.is_empty() {
            moving_str.push_str("none");
        }

        // Find it
        book.hexagram.into_iter().find(|x| x.lines == lines)
            .unwrap_or_else(|| panic!("Could not find hexagram matching lines \
                                       {lines}"))
    }
    else {
        let temp_hex = book.hexagram.into_iter().find(|x| x.number==cli.hex
                                                                       .into())
                           .unwrap_or_else(|| panic!("Could not find hexagram \
                                                      matching number {}",
                                                     cli.hex));
        moving = "111111".to_string();
        moving_str = "none".to_string();
        let mut temp_large_hex : String = Default::default();
        for i in [5, 4, 3, 2, 1, 0].into_iter() {
            if temp_hex.lines.chars().nth(i).unwrap() == '1'
            {
                temp_large_hex.push_str("_________\n");
            }
            else {
                temp_large_hex.push_str("____ ____\n");
            }
        }
        large_hex = temp_large_hex;
        temp_hex
    };


    // Display info
    println!("Hexagram number {}", hex.number);
    if cli.prompt {
        let cont = Confirm::new("Continue?")
                          .with_default(true)
                          .prompt();
        if !cont.unwrap_or(true) {
            return;
        }
    }

    println!("{large_hex}");
    if cli.prompt {
        let cont = Confirm::new("Continue?")
                          .with_default(true)
                          .prompt();
        if !cont.unwrap_or(true) {
            return;
        }
    }

    println!("{} {}: {}", hex.utf, hex.pinyin, hex.name);
    if cli.hex == 0 {
        println!("Moving lines: {moving_str}");
    }

    let mut default = vec![0, 1, 2, 3];
    for (i, ch) in moving.chars().enumerate() {
        if ch == '1' {
            default.push(i + 4);
        }
    }
    let mut selected = if cli.prompt {
        MultiSelect::new("Display: ", Vec::from(OPTIONS))
                    .with_default(default.leak()).with_page_size(10)
                    .prompt().unwrap()
    }
    else {
        Vec::from(OPTIONS)
    };
    let print_comm = selected.extract_if(..,|x| *x=="Commentaries").count()!=0;

    println!();
    let options_iter = OPTIONS.into_iter();
    let hex_iter = [&hex.name, &hex.preface, &hex.judgment, &hex.image,
                    &hex.line_1, &hex.line_2, &hex.line_3, &hex.line_4,
                    &hex.line_5, &hex.line_6].into_iter();
    let mut parameter_iter = options_iter.zip(hex_iter);
    parameter_iter.next();
    for (option, parameter) in parameter_iter {
        if selected.extract_if(.., |x| *x==option).count() == 1 {
            println!("{option}:");
            print_within(parameter, None);
            if option != "Preface" && print_comm {
                if option == "Judgment" {
                    println!("---");
                    print_within(&hex.judgment_comm, None);
                }
                else if option == "Image" {
                    println!("---");
                    print_within(&hex.image_comm, None);
                }
                else if option == "Line 1" {
                    println!("---");
                    print_within(&hex.line_1_comm, None);
                }
                else if option == "Line 2" {
                    println!("---");
                    print_within(&hex.line_2_comm, None);
                }
                else if option == "Line 3" {
                    println!("---");
                    print_within(&hex.line_3_comm, None);
                }
                else if option == "Line 4" {
                    println!("---");
                    print_within(&hex.line_4_comm, None);
                }
                else if option == "Line 5" {
                    println!("---");
                    print_within(&hex.line_5_comm, None);
                }
                else if option == "Line 6" {
                    println!("---");
                    print_within(&hex.line_6_comm, None);
                }
            }
            println!("---------");
        }
    }
}
