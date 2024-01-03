use std::io;

fn main() {
    loop {
        println!("Input a number to find the next Prime: ");

        let mut number = String::new();

            io::stdin()
                .read_line(&mut number)
                .expect("Please use an integer.");

        next_prime(number);
    }
}


fn next_prime(n: String){ 
    let int_n: i64;

    match n.trim().parse::<i64>() {
        Ok(parsed_n) => {
            if parsed_n >= 9223372036854775806 {
                println!("Exceeding i64 byte range, please try a smaller number!");
                return;
            }
            int_n = parsed_n; // Set int_n if number is valid and continue
            // println!("Parsed value: {}", parsed_n);
        }
        Err(e) => {
            println!("Error parsing integer: {}", e);
            return
        }
    }
    // println!("n: {:?}", &n); // N Itself is also including whitespace/trailing characters
    // println!("Int: {}", int_n); // Using .trim() allows us to use only the number and avoid integer parsing errors.

    for count in int_n.. { // Run indefinitely until next prime is found (Or to 9223372036854775806 which is the i64 range limitation)
        if count >= 9223372036854775806{
            println!("Exceeding i64 byte range, returning");
            return
        }

        if count % 2 != 0 {
            if check_prime(&count){
                println!("The next prime is: {}", count);
                return
            }
        }
    }

    fn check_prime(count: &i64) -> bool {
        let stop = ((*count as f64).sqrt() + 1.0) as i64;
        for i in 3..stop {
            if i % 2 != 0 {
                if count % i == 0 {
                    return false
                }
            }
        }
        return true
    }
}

// Harley' Notes:
// Interesting excercise, for reference - I had to re familiarize myself with what a prime was heh.
// I didn't have any concept of how a prime is determined so I've referenced code from this article, with some changes https://findelabs.com/post/calculating-prime-numbers-in-rust/
// As the challenge is to find the NEXT prime, I've adjusted the for loop to continue indefinitely until a prime is found
// However with this, I've found that i64 have a limitation of 9223372036854775806 before it becomes overflowed
// This finding, cause the need for me to impose a hard limit in the next_prime() function so that it doesn't exceed.
// Also discovered at the same time that if I tried to convert a number from String higher than this range limit then it also errors.
// Coded a work around to assign value to int_n if considered Ok() & throw error for any other instance.
// Some interesting discovery work here, maybe there is an easier way or some sort of inbuilt next_prime() function inside of Rust in 2024 - but again may be something worth review
// At a later date if determined there is a better way to handle.

// RESULTS:
// PS N:\100-Days-Of-Rust\Week-01\Day-06_Next-Prime\my_next_prime> date
// Wednesday, 3 January 2024 4:44:14 PM

// PS N:\100-Days-Of-Rust\Week-01\Day-06_Next-Prime\my_next_prime> cargo run
//     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
//      Running `target\debug\my_next_prime.exe`
// Input a number to find the next Prime:
// 12
// The next prime is: 13
// Input a number to find the next Prime:
// 24
// The next prime is: 29
// Input a number to find the next Prime:
// 11
// The next prime is: 11
// Input a number to find the next Prime:
// 1251
// The next prime is: 1259
// Input a number to find the next Prime:
// 123123125
// The next prime is: 123123137
// Input a number to find the next Prime:


// ## Next Prime

// Given an integer, create a function that returns the next prime. If the number is prime, return the number itself.

// ### Examples

// ```text
// NextPrime(12) ➞ 13

// NextPrime(24) ➞ 29

// NextPrime(11) ➞ 11
// // 11 is a prime, so we return the number itself.
// ```

// ---

// ### Notes

// - N/A
