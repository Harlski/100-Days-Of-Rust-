// DAY 02
// Man what a challenge.
// Still a work in progress
// Feel like I should know how to do this by now, but nothing is sticking.
// Probably a really simple way to do this, but I'm determined not to influence my answer with an existing one
// If it takes me an entire week, I'll keep going at this problem.

// Day 03
// So I am not far off solving this
// As of this commit, I am able to output the correct answer for 4 digits
// However any numbers lower currently produces error
// I'm currently refactoring to ensure that the correct while loop is run depending on the length of the digits
// Don't know if this optimal, however I like to think it looks tidy/clean.
// The issue/problem I'm facing is getting type declared initially for  j_char, j_char_len j_chars (&str, usize, Vec<char>) for (j,k,l)
// I'm not sure what the best practie is to handle this, so I'm just test as I go.
// I'm think the best might be to set placeholder values and overwrite if the digits.len() condition is met
// Because if the values are set and digits.len() isn't met then they're not going to be used anyway.

// Day 04
// So the problem is solved, however I'm trying to fix some issues with the way the digit check is handled.
// If the number is 1 or 0 then we shouldn't pass value through to return_string_combo
// I tried restarting main() if the value was 1 or 0, but this seemed to start a new instance of main.
// Then when submitting a valid number, it would call multiple instances of the R_S_C() function.
// So calling main() isn't a good option, instead I need to find a way to loop over the stdin() 
// And if the it isn't a number from 2-9 then we should ask to enter a number againgit add 

// HES DONE IT
// Finally solved every issue, challenege is complete
// After so many days of trying, no hints or referring to existing code
// While this is only day 8, it has already been such a challenge.
// Considering I'm still here, without referencing ChatGPT and solving all the hurdles by myself
// I consider this a massive win.
// Again, I don't know if this the optimal way to complete this, but it works. GG. Notes at bottom.

use std::io;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str { // Added for visibility of types when it came to assigning before declaration in the let _char, _chars, _char_len block.
    type_name::<T>()
}

fn main() { // This function requests the input of a number to return the string combo.
    let mut digits = "".to_string();
    let valid_input: Vec<char> = vec!['2','3','4','5','6','7','8','9'];
    loop {
        // println!("Digits: {:?}", digits);
        println!("Enter a number to get the letter combinations (1 to 4 digits):");

        io::stdin()
            .read_line(&mut digits)
            .expect("Failed to get.");

        let mut escape = false;
        // println!("Digits length: {} Digits: {}", digits.len(), digits);
        if digits.len() == 1{
            let answer = return_string_combo(&digits.trim());
            println!("Answer: {:?}", answer);
            break
        }
        for digit in digits.clone().trim().chars() {
            if !valid_input.contains(&digit) {
                digits = "".to_string();
                println!("Can't use {:?} sorry! Numbers 2-9 only!", digit);
                break
            }
            escape = true;
        }
        if escape {
            let answer = return_string_combo(&digits.trim());
            println!("Answer: {:?}", answer);
            break
        }
        // println!("Need to get here");
        // let _digits_are_int: &i32 = match &digits.trim().parse() {
        //     Ok(digits) => { println!("Done"); },
        //     Err(_) => continue,
        // };
        // println!("What: {:?}",Ok::<&i32, E>(_digits_are_int));
    }


    // println!("Digits 12: {:?}", &digits.trim());
    // // println!("Answer: {:?}", return_string_combo(&digits.trim()));
    // // println!("Digits: {:?}", &digits);


}



fn return_string_combo(digits: &str) -> Vec<String> {
    // println!("Digits: {:?} Len: {:?}", &digits, &digits.len());

    let letters = vec!["abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"];
    let digits = digits.trim().chars().collect::<Vec<_>>();
    let mut v: Vec<String> = Vec::new();
    if digits.len() == 0 {
        return v;
    }
    // The variables used for tracking each character for assigned letter.
    let mut i = 0; // First Digit
    let mut j = 0; // Second Digit
    let mut k = 0; // Third Digit
    let mut l = 0; // Fourth Digit

    // println!("Type of i: {}", type_of(&i));
    // 

    let i_char = letters[digits[0] as usize - 50]; // Get the correct number at the index of letters. We remove 48 for the usize conversion and then 2 to match the number to the index
    let i_chars = i_char.chars().collect::<Vec<_>>(); // Places each letter in its own index.
    let i_char_len = i_char.len(); // This just helped make it so much easier for readability when writing the looping code. My brain wasn't pieceing it together when trying to do it without this.

    // println!("Type i_char, i_chars, i_char_len: {} {:?} {}", type_of(&i_char), type_of(&i_chars), type_of(&i_char_len));

    // This block is declared as empty, then if the digits.len() condition is met - the correct values will be filled.
    
    let (mut j_char, mut k_char, mut l_char) = ("","",""); 
    let (mut j_chars, mut k_chars, mut l_chars): (Vec<char>, Vec<char>, Vec<char>) = (vec![],vec![],vec![]);
    let (mut j_char_len, mut k_char_len, mut l_char_len) = (0, 0, 0);

    if digits.len() >= 2 {
        // println!("Should be assigning j");
        j_char = letters[digits[1] as usize - 50];
        j_chars = j_char.chars().collect::<Vec<_>>();
        j_char_len = j_char.len();
    }  
    if digits.len() >= 3 {
        // println!("Should be assigning k");
        k_char = letters[digits[2] as usize - 50];
        k_chars = k_char.chars().collect::<Vec<_>>();
        k_char_len = k_char.len();
    }
    if digits.len() >= 4 {
        // println!("Should be assigning l");
        l_char = letters[digits[3] as usize - 50];
        l_chars = l_char.chars().collect::<Vec<_>>();
        l_char_len = l_char.len();
    }
    //
    
    // This code runs the appropriate logic based on the length of digits
    if digits.len() >= 5 {
        println!("Digits over 5 in length are not supported, though logically it can be added.");
    }
    if digits.len() == 4 {         
        println!("Length 4" );
        while i <= i_char_len -1 && j <= j_char_len && k <= k_char_len && l <= l_char_len {
            v.push(i_chars[i].to_string() + &j_chars[j].to_string() + &k_chars[k].to_string() + &l_chars[l].to_string()); // Pushes the string to vector
            if l <= l_char_len { l += 1; if l == l_char_len { k += 1; l = 0; } } 
            if k == k_char_len { j += 1; k = 0}   
            if j == j_char_len { i += 1; j = 0}  
        }
    }
    if digits.len() == 3 { println!("Length 3" );
        while i <= i_char_len -1 && j <= j_char_len && k <= k_char_len {
            v.push(i_chars[i].to_string() + &j_chars[j].to_string() + &k_chars[k].to_string()); // // Pushes the string to vector
            if k <= k_char_len { k += 1; if k == k_char_len { j += 1; k = 0; } } 
            if j == j_char_len { i += 1; j = 0}  
        }
    }
    if digits.len() == 2 { println!("Length 2" );
        while i <= i_char_len -1 && j <= j_char_len {
            v.push(i_chars[i].to_string() + &j_chars[j].to_string()); // // Pushes the string to vector
            if j <= j_char_len { j += 1; if j == j_char_len { i += 1; j = 0; } } 
        }
    }
    if digits.len() == 1 { println!("Length 1" );
        while i <= i_char_len -1 {
            v.push(i_chars[i].to_string()); // // Pushes the string to vector
            if i <= i_char_len { i += 1;}
        }
    }
    v // Returns the vector once while loop concludes.
}

    // println!("i: {}, j:{}, k:{}, l{}", i, j, k ,l);
    // println!("What the digits: {:?}", digits);
    // let char_at: Vec<_> = letters[i as usize].chars().collect();

    // println!("i: {}, j:{}, k:{}, l{}", i, j, k ,l);
    // println!("i_char: {}, j_char: {}, k_char: {}, l_char: {}", i_char, j_char, k_char, l_char);
    // println!("Length: i_char: {:?}", i_char.chars().collect::<Vec<_>>());
    // println!("Length i_char: {}, Length j_char: {}, Length k_char: {}, Length l_char: {}", i_char_len, j_char_len, k_char_len, l_char_len);
        // println!("string: {}", i_chars[i].to_string() + &j_chars[j].to_string() + &k_chars[k].to_string() + &l_chars[l].to_string());

        // println!("We in here");

        // if i == i_char_len && j == j_char_len && k == k_char_len && l == l_char_len { println!("Should exit here"); return v; }

        // if l <= l_char_len { l += 1; if l == l_char_len { k += 1; l = 0; } } 
        // if k == k_char_len { j += 1; k = 0}   
        // if j == j_char_len { i += 1; j = 0}  
   
    // let mut current = "";

    // while i <= digits.len() {
    //     if digits.len() >= 4 {
    //         v.push(digits[i].to_string() + &digits[j].to_string() + &digits[k].to_string() + &digits[l].to_string() ); // This pushes current
    //     }
    //     if digits.len() >= 3 {
    //         v.push(digits[i].to_string() + &digits[j].to_string() + &digits[k].to_string() ); // This pushes current
    //     }
    //     if digits.len() >= 2 {
    //         v.push(digits[i].to_string() + &digits[j].to_string() ); // This pushes current
    //     } 
    //     if digits.len() >=1 {
    //         v.push(char_at[digits[0] as usize -48].to_string()); // This pushes current
    //         i += 1;

    //     }
    // }
    // if i >= digits[0].len() && j >= digits[1].len() && k >= digits[2].len() && l >= digits[3].len() {
    //     return v;
    // }


    // println!("What I expect to be g: {:?} ", char_at[0]);
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
