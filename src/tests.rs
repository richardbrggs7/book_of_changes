use crate::{
    Changes
};

use std::fs;

#[test]
fn test_books() {
    for translation in fs::read_dir("./translations")
                         .expect("Could not read translations \
                                  directory") {
        let translation = translation.unwrap();
        let book_str = fs::read_to_string(translation.path())
                         .expect(&format!("Could not read {:?}",
                                          translation.file_name()));
        println!("Checking {:?}...", translation.file_name());
        let mut all_utf : Vec<String> = Vec::new();
        let mut all_number : Vec<u16> = Vec::new();
        let mut all_lines : Vec<String> = Vec::new();
        let book : Changes = toml::from_str(&book_str).unwrap();
        for hex in &book.hexagram {
            // First check utf symbols for duplicates
            let current_utf = &hex.utf;
            let mut errors = 0_u8;
            for utf in &all_utf {
                if utf == current_utf {
                    errors += 1;
                }
            }
            assert_eq!(errors, 0);
            all_utf.push(current_utf.to_string());

            // Then check number
            let current_number = &hex.number;
            errors = 0;
            for number in &all_number {
                if number == current_number {
                    errors += 1;
                }
            }
            assert_eq!(errors, 0);
            all_number.push(*current_number);

            // Then check lines
            let current_lines = &hex.lines;
            errors = 0;
            for lines in &all_lines {
                if lines == current_lines {
                    errors += 1;
                }
            }
            assert_eq!(errors, 0);
            all_lines.push(current_lines.to_string());
        }
    }
}
