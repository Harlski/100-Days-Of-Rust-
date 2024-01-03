use std::collections::HashMap;

fn main() {
    let sock_batch: [&str; 6] = [
        "AA",
        "ABABC",
        "CABBACCC",
        "ABCCBCBACABC",
        "AABCBCBCABACAB",
        "ABCJDLKLHWKLBNDMBFBNMNASFKLSLKHASNBFBASFBNNMASDMBASFMBASF"
    ];

    for socks in sock_batch{
        sock_pairs(socks);
    }
}

fn sock_pairs(s: &str){
    let mut count_pairs = HashMap::new();

    for sock in s.chars(){
        let counter = count_pairs.entry(sock).or_insert(0);
        *counter += 1;
    }

    let mut pair_count = 0;

    for &count in count_pairs.values() {
        pair_count += count /2;
    }


    println!("Pair count: {:?} ||  Socks in pile: {:?}", pair_count, count_pairs);
}

// Theory:
// Perhaps the easiest way would be to get amount of pairs would be to count amount of each letter an divide by 2. For odd numbers, we just round down
// If there is 4 socks of A then that's 2 pairs.
// If there is 5 socks of A then that's 2.5 pairs, but we round down to 2 pairs. etc..

// Actual:
// My initial idea wasn't ideal, found reference to hashmap being the better option here.
// Admittedly, I have used ChatGPT here to help explain with a bit more clarity on how this works.
// While the logic was still the same implementation just required some assistance.
// I now understand .entry() and .or_insert() and incrementing the counters - * deferencing is a bit clearer and obviously first time usage of HashMap was nice to learn.
// Ideally I would've like to have been able to sort the hashmap alphabetically, but doing so looks to have required a BTreeMap tool - which appears uneccesary for the completion of this challenge.
// Will experiment with BTreeMap in future though.


// Unused:
// Sort by character -> Split by character -> collect into vector -> get len of vector (vl). vl % 2 


// RESULTS:
// PS N:\100-Days-Of-Rust\Week-01\Day-05_Pair-Of-Socks\my_pair_of_socks> date     
// Wednesday, 3 January 2024 10:31:02 AM

// PS N:\100-Days-Of-Rust\Week-01\Day-05_Pair-Of-Socks\my_pair_of_socks> cargo run
//     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
//      Running `target\debug\my_pair_of_socks.exe`
// Pair count: 1 ||  Socks in pile: {'A': 2}
// Pair count: 2 ||  Socks in pile: {'B': 2, 'A': 2, 'C': 1}
// Pair count: 4 ||  Socks in pile: {'C': 4, 'B': 2, 'A': 2}
// Pair count: 5 ||  Socks in pile: {'A': 3, 'B': 4, 'C': 5}
// Pair count: 6 ||  Socks in pile: {'A': 5, 'B': 5, 'C': 4}
// Pair count: 24 ||  Socks in pile: {'D': 3, 'L': 5, 'M': 5, 'K': 4, 'N': 6, 'C': 1, 'B': 9, 'J': 1, 'W': 1, 'H': 2, 'A': 7, 'F': 6, 'S': 7}

// CHALLENGE INFO:
// ## Pair of Socks

// Joseph is in the middle of packing for a vacation. He's having a bit of trouble finding all of his socks, though.

// Write a function that returns the number of sock pairs he has. A sock pair consists of two of the same letter, such as `"AA"`. The socks are represented as an unordered sequence.

// ### Examples

// ```text
// SockPairs("AA") ➞ 1

// SockPairs("ABABC") ➞ 2

// SockPairs("CABBACCC") ➞ 4
// ```

// ---

// ### Notes

// - If given an empty string (no socks in the drawer), return 0.
// - There can be multiple pairs of the same type of sock, such as two pairs of CC for the last example.
