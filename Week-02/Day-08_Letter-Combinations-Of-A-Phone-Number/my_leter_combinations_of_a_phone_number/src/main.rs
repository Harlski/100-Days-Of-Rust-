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
    
    return_string_combo(&digits.trim());
}


fn return_string_combo(digits: &str) -> Vec<String> {
    // let age
    // 1 ""

    let letters = vec!["abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"];
    println!("Digits: {:?}", &digits);
    let v: Vec<String> = Vec::new();
    for digit in digits.trim().chars() {
        let i = digit 
        println!("Character: {}", digit);
    }
    v
}

2 3 4
abc def ghi 
adg adh adi aeg aeh aei afg afh afi bdg bdh bdi 
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
