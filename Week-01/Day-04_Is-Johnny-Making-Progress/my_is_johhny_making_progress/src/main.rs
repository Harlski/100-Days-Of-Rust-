fn main() {
    let saturdays:[i32; 11] = [3,6,7,2,6,3,4,6,6,6,7];
    progress_days(saturdays);
}

fn progress_days(days: [i32; 11]){
    let mut progress_days:i32 = 0;
    let mut prev_day:usize = days[0].try_into().unwrap(); 
    
    for i in 0..days.len() {
        if i == 0{
            println!("No need to compare first day");
            continue
        }

        if days[i] > prev_day.try_into().unwrap(){
            println!("Previously: {} miles to current {} miles! We made progress! ", prev_day, days[i]);
            progress_days += 1;
            prev_day = i;
        } else {
            println!("Previously: {} miles to current {} miles. No progress :( ", prev_day, days[i]);
            prev_day = days[i].try_into().unwrap(); 
        }
    }
    
    // for d in days{
    //     if d > prev_day{
    //         println!("Previously: {} miles to current {} miles! We made progress! ", prev_day, d);
    //         progress_days += 1;
    //         prev_day = d;
    //     } else {
    //         println!("Previously: {} miles to current {} miles. No progress :( ", prev_day, d);
    //         prev_day = d; 
    //     }
    // }

    println!("Amount of progress days: {}!", progress_days);
}

// Harley' Notes:
// Challenge requirements are fulfilled.
// Initially I just performed a for d in days, which  was performing a compare on the first day with itself
// Which is redundant so instead converted to a for i in 0..days.len()
// Initially there was some conversion required to make this work, specifically the compiler pointed me towards where to place the try_into().unwrap().
// Take aways from this challenge I've found is that it seems as though array parameters need to be declared specifically. I was having some confusing with the formatting. But got there in the end.
// Understand the implementation for i in 0..days.len() implementation, while similiar to js and python, it's just a good refresher.
// I still don't fully understand try_into().unwrap() as documentation mentions to use ::from instead. However this may be something for me to pickup as I continue.
// 


// RESULTS:

// PS N:\100-Days-Of-Rust\Week-01\Day-04_Is-Johnny-Making-Progress\my_is_johhny_making_progress> date     
// Tuesday, 2 January 2024 9:20:32 PM


// PS N:\100-Days-Of-Rust\Week-01\Day-04_Is-Johnny-Making-Progress\my_is_johhny_making_progress> cargo run
//    Compiling my_is_johhny_making_progress v0.1.0 (N:\100-Days-Of-Rust\Week-01\Day-04_Is-Johnny-Making-Progress\my_is_johhny_making_progress)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.40s
//      Running `target\debug\my_is_johhny_making_progress.exe`
// No need to compare first day
// Previously: 3 miles to current 6 miles! We made progress! 
// Previously: 1 miles to current 7 miles! We made progress!
// Previously: 2 miles to current 2 miles. No progress :(
// Previously: 2 miles to current 6 miles! We made progress!
// Previously: 4 miles to current 3 miles. No progress :(
// Previously: 3 miles to current 4 miles! We made progress!
// Previously: 6 miles to current 6 miles. No progress :(
// Previously: 6 miles to current 6 miles. No progress :(
// Previously: 6 miles to current 6 miles. No progress :(
// Previously: 6 miles to current 7 miles! We made progress!
// Amount of progress days: 5!


// CHALLENGE INFO:
// ## Is Johnny Making Progress?

// To train for an upcoming marathon, Johnny goes on one long-distance run each Saturday. He wants to track how often the number of miles he runs this Saturday exceeds the number of miles run the **previous** Saturday. This is called a **progress day**.

// Create a function that takes in an array of miles run every Saturday and returns Johnny's total number of progress days.

// ### Examples

// ```text
// progressDays([3, 4, 1, 2]) ➞ 2
// // There are two progress days, (3->4) and (1->2)

// progressDays([10, 11, 12, 9, 10]) ➞ 3

// progressDays([6, 5, 4, 3, 2, 9]) ➞ 1

// progressDays([9, 9])  ➞ 0
// ```

// ---

// ### Notescargo

// - Running the **same number of miles** as last week does not count as a progress day.
