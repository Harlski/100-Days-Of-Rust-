use std::io;


fn main() {
    println!("What is your age? ");
    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to get age");

    let u_age = age.trim().parse::<u32>().expect("Please type a positive number! ");

    calc_age(u_age.clone());
    println!("Your age is {age}");
} 

fn calc_age(u_age: u32){
    let age_days = u_age * 365;
    // let mut age_to_days = age * 365;
    println!("Your age in days is: {age_days}");
}


// Harley' Notes:
// Found types interesting, as I've some experience with Python and Javascript - it's unfamiliar to me to declare variable types.
// Though a lot of what I've done is unclear and likely to come with repetition, though I could initially set the .read_line as a u32 - but apprently not possible.
// I ran under the assumption that I had to create a new function, this introduced me to borrowing and ownership. Understood a little bit on this, probably a better way to handle - 
// but I found it workable just by performing .clone() on the u32 variable when passing to the function.

// After completing this task, I've review the supplied answer and can see simiiarties in the way the answer was concluded.
// 1. I've not ran this in a loop, so program immediately ends after providing correct answer
// 2. I've not ran a match to confirm if it is a number and handle the error gracefully.

// RESULTS:
// PS C:\Users\Harlski\100-Days-Of-Rust\Week-01\Day-01_Convert-Ages-To-Days\my_convert_ages_to_days> date
// Monday, 1 January 2024 10:09:19 PM

// PS C:\Users\Harlski\100-Days-Of-Rust\Week-01\Day-01_Convert-Ages-To-Days\my_convert_ages_to_days> cargo run
//    Compiling my_convert_ages_to_days v0.1.0 (C:\Users\Harlski\100-Days-Of-Rust\Week-01\Day-01_Convert-Ages-To-Days\my_convert_ages_to_days)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.51s
//      Running `target\debug\my_convert_ages_to_days.exe`
// What is your age?
// 65
// Your age in days is: 23725
// Your age is 65

// PS C:\Users\Harlski\100-Days-Of-Rust\Week-01\Day-01_Convert-Ages-To-Days\my_convert_ages_to_days> cargo run
//     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
//      Running `target\debug\my_convert_ages_to_days.exe`
// What is your age?
// 0
// Your age in days is: 0
// Your age is 0

// PS C:\Users\Harlski\100-Days-Of-Rust\Week-01\Day-01_Convert-Ages-To-Days\my_convert_ages_to_days> cargo run
//     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
//      Running `target\debug\my_convert_ages_to_days.exe`
// What is your age?
// 20
// Your age in days is: 7300
// Your age is 20


// CHALLENGEE INFO:
// ## Convert Age to Days

// Create a function that takes the age and return the age in days.

// #### Examples

// ```text
// calcAge(65) ➞ 23725

// calcAge(0) ➞ 0

// calcAge(20) ➞ 7300
// ```

// ### Notes

// - Use 365 days as the length of a year for this challenge. 
// - Ignore leap years and days between last birthday and now.
// - Expect only positive integer inputs.


