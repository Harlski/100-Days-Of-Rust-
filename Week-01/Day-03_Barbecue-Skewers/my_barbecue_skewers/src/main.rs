fn main() {
    let skewers: [&str; 15] = [
          "--oooo-ooo--",
          "--xx--x--xx--",
          "--o---o--oo--",
          "--xx--x--ox--",
          "--xx--x--ox--",
          "--xo--x--ox--",
          "--xx--x--xx--",
          "--oo--o--oo--", 
          "--xx--x--ox--",
          "--xx--x--ox--",
          "--oooo-ooo--",
          "--xxxxxxxx--",
          "--o---",
          "-o-----o---x--",
          "--o---o-----"
    ];

    count_skewers(skewers.to_vec());
}

fn count_skewers(skewers: Vec<&str>){
    let mut veg_skewers:i32 = 0;
    let mut non_veg_skewers:i32 = 0;

    for skew in &skewers{
        if skew.contains('x'){
            println!("{skew} is non vegetarian.");
            non_veg_skewers += 1;
        } else {
            println!("{skew} is vegetarian.");
            veg_skewers += 1;
        }
    }

    println!("[{}, {}]", veg_skewers, non_veg_skewers);
}


//  Harley' Notes:
//  Honestly, this didn't take too long. Using some logic from the previous challenge, I was able to meet the requirements quite quickly.
//  This was for the most part coded from memory without being ran, only towards the end was I able to use rust errors to determine what was incorrect.
//  The only real error I encountered was that when passing skewers to count_skewers, I got a type error that indicated that I should convert to_vec().
//  Otherwise it was just a missed semi-colon.
//  It's hard to know if I'm using best pracitces, or if there is a more optimal way to complete these challenge
//  I'll mark this as completed, and potentially come back in the future to rework once my knowledge of Rust improves.


// RESULTS:
// PS N:\100-Days-Of-Rust\Week-01\Day-03_Barbecue-Skewers\my_barbecue_skewers> date
// Tuesday, 2 January 2024 8:00:18 PM

// PS N:\100-Days-Of-Rust\Week-01\Day-03_Barbecue-Skewers\my_barbecue_skewers> cargo run
//    Compiling my_barbecue_skewers v0.1.0 (N:\100-Days-Of-Rust\Week-01\Day-03_Barbecue-Skewers\my_barbecue_skewers)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.52s
//      Running `target\debug\my_barbecue_skewers.exe`
// --oooo-ooo-- is vegetarian.
// --xx--x--xx-- is non vegetarian.
// --o---o--oo-- is vegetarian.
// --xx--x--ox-- is non vegetarian.
// --xx--x--ox-- is non vegetarian.
// --xo--x--ox-- is non vegetarian.
// --xx--x--xx-- is non vegetarian.
// --oo--o--oo-- is vegetarian.
// --xx--x--ox-- is non vegetarian.
// --xx--x--ox-- is non vegetarian.
// --oooo-ooo-- is vegetarian.
// --xxxxxxxx-- is non vegetarian.
// --o--- is vegetarian.
// -o-----o---x-- is non vegetarian.
// --o---o----- is vegetarian.
// [6, 9]




// INFO
// ## Barbecue Skewers

// You are in charge of the barbecue grill. A **vegetarian skewer** is a skewer that has **only vegetables (-o)**. A **non-vegetarian skewer** is a skewer with **at least one piece of meat (-x)**.

// For example, the grill below has **4 non-vegetarian skewers** and **1 vegetarian skewer** (the one in the middle).

// ```text
// ["--xo--x--ox--",
// "--xx--x--xx--",
// "--oo--o--oo--",      <<< vegetarian skewer
// "--xx--x--ox--",
// "--xx--x--ox--"]
// ```

// #### Examples

// Given a BBQ grill, write a function that returns `[# vegetarian skewers, # non-vegetarian skewers]`. For example above, the function should return `[1, 4]`.

// ```text
//  [
//   "--oooo-ooo--",
//   "--xx--x--xx--",
//   "--o---o--oo--",
//   "--xx--x--ox--",
//   "--xx--x--ox--"
// ] ➞ [2, 3]

// [
//   "--oooo-ooo--",
//   "--xxxxxxxx--",
//   "--o---",
//   "-o-----o---x--",
//   "--o---o-----"
// ) ➞ [3, 2]
// ```

// ---

// ### Notes

// - NA
