use std::io;

fn main() {
    let mut digits = String::new();

    println!("Enter a number to get the letter combinations");
    io::stdin()
        .read_line(&mut digits)
        .expect("Failed to get.");

    for digit in digits.trim().chars() {
        if digit == '1' {
            println!("Can't use {} sorry!", digit);
            main();
        }

    }
    println!("Digits: {:?}", &digits);
    return_string_combo(&digits.trim());
}


fn return_string_combo(digits: &str) -> Vec<String> {
    let letters = vec!["abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"];
    let v: Vec<String> = Vec::new();
    
    let digit_usize = digit as usize - 48; // Casting &str to usize makes number n + 48, just hacking it back to the original number.
    let corr_letters = letters[digit_usize -2];
    let mut iter_letters = corr_letters.chars();
    let i,j,k,l = (iter_letters.length(), ) // Assume max of 4 depth

    for digit in digits.trim().chars() {
        println!("duz: {}", digit_usize);
        println!("Letters: {} at number: {}, then seperate: {:?}", corr_letters, digit, iter_letters); // we let digit_usize - 2 in order to pull from letters correctly
        println!("Character: {}", digit);
        // for i <= letters

    }
    v
}

// 2 3 4
// abc def ghi
// adg adh adi aeg aeh aei afg afh afi bdg bdh bdi
// ## Letter Combinations of a Phone Number

// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in **any order**.

// A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.

// <p align="left">
//   <img src="../../assets/Telephone-keypad2.png" alt="telefone keypad">
// </p>

// ### Examples


// **Example 1:**

// ```text
// Input: digits = "23"
// Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
// ```

// **Example 2:**

// ```text
// Input: digits = ""
// Output: []
// ```

// **Example 3:**

// ```text
// Input: digits = "2"
// Output: ["a","b","c"]
// ```

// ### Constraints

// - `0 <= digits.length <= 4`
// - `digits[i] is a digit in the range ['2', '9'].`

// ---

// ### Notes

// - N/A
