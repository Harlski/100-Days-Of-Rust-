// Man what a challenge.
// Still a work in progress
// Feel like I should know how to do this by now, but nothing is sticking.
// Probably a really simple way to do this, but I'm determined not to influence my answer with an existing one
// If it takes me an entire week, I'll keep going at this problem.
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
    println!("Answer: {:?}", return_string_combo(&digits.trim()));
}


fn return_string_combo(digits: &str) -> Vec<String> {
    let letters = vec!["abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"];
    let digits = digits.trim().chars().collect::<Vec<_>>();
    let mut v: Vec<String> = Vec::new();

    let mut i = 0; // First Digit
    let mut j = 1; // Second Digit
    let mut k = 2; // Third Digit
    let mut l = 3; // Fourth Digit
    // let mut current = "";
    let char_g: Vec<_> = letters[2].chars().collect();
    println!("What the digits: {:?}", digits);
    if 
    if digits.len() >= 4 {
        v.push(digits[i].to_string() + &digits[j].to_string() + &digits[k].to_string() + &digits[l].to_string() ); // This pushes current
    }
    if digits.len() >= 3 {
        v.push(digits[i].to_string() + &digits[j].to_string() + &digits[k].to_string() ); // This pushes current
    }
    if digits.len() >= 2 {
        v.push(digits[i].to_string() + &digits[j].to_string() ); // This pushes current
    }
    // if i >= digits[0].len() && j >= digits[1].len() && k >= digits[2].len() && l >= digits[3].len() {
    //     return v;
    // }


    // println!("What I expect to be g: {:?} ", char_g[0]);
    // while i <= digits.len() {
    //     v.push(letters )
    // }
    // // let d
    // for letter in letters[digits[0 as usize - 48]] {        
    //     for letter in letters[digits[1 as usize - 48]] {
    //         for letter in letters[digits[2 as usize - 48]] {
    //             if &digits.len() <= '3' { v.push(current).to_string(); k += 1; }
    //             for letter in letters[digits[3 as usize - 48]]{
    //                 // v.push(current);
    //                 l += 1;
    //             }
    //             // v.push(current);
    //             k += 1;
    //         }
    //         // v.push(current);
    //         j += 1;
    //     }
    //     // v.push(current);
    //     i += 1;
    // }
    // for digits[i]
    // println!("FSTF: {} {} {} {}", first, second, third, fourth);
    // for digit in &digits {
    //     println!("Digits: {:?} i: {}", digit, i);

    //     i += 1;
    // }
    // let characters = Vec::new();
    // for digit in digits {
    //     let digit_usize = digit as usize - 48; // Casting &str to usize makes number n + 48, just hacking it back to the original number.
    //     let corr_letters = letters[digit_usize -2];
    //     characters.push(corr_letters);
    //     // let mut iter_letters = corr_letters.chars();
    // }
        
    // println!("Digits: {:?} & Characters", digits);
    // for digit in digits.trim().chars() {
        
    //     println!("duz: {}", digit_usize);
    //     println!("Letters: {} at number: {}, then seperate: {:?}", corr_letters, digit, iter_letters); // we let digit_usize - 2 in order to pull from letters correctly
    //     println!("Character: {}", digit);
    //     // for i <= letters

    // }
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
