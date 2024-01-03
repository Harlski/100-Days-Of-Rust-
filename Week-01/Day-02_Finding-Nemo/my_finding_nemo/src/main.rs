fn main() {

    let nemo_strings: [&str; 7] = [
        "I'm trying to find Nemo !",
        "I'm trying to find Dory !",
        "Nemo is trying to find Dory !",
        "Nemo is trying to find Nemo !",
        "Nemod is trying to find Nemo !",
        "To find Nemo I must !",
        "I'm not Nemo, Nemo is Nemo !",
    ];

    for nemo_string in &nemo_strings {
        find_nemo(nemo_string);
    }
}



fn find_nemo(s: &str){
    println!("Searching line: {}", s);

    let nemo = "Nemo";
    let parts = s.split(" ");  // Split the string by " "
    let collection: Vec<&str> = parts.collect(); // Collect words into a Vector
    
    let mut index = 0; // Only useful for declaring the location Nemo was found
    for i in &collection{
        // println!("{index}. {}", i);
        if i == &nemo{
            index += 1; // Increments index, if Nemo is found as index 0 - per challenge guidelines it should be position 1 - this just satisfys the conditions.
            println!("I found Nemo at {index}");
            println!("");
            return // return out of fn as we don't need to continue searching
        }
        index += 1; 
    }
    println!("I wasn't able to find Nemo :(");
    println!("");
    // dbg!(collection); 
}
    // Harley' Notes:
    // Initially I had intended to search for Nemo in Bytes, while probably still a valid option - though research pointed 
    // me towards spliting the passed string by "spaces" and then adding these to a collection - which I know as just a standard array 
    // then comparing my search word, in this case "Nemo" to each item in the Vector.
    // This appears to satisfy all the conditions of this challlenge.


    // Below logic is what I would've proceeded with if searching by bytes would have been the easier option. I may return to this in the future once I've understood the language a bit better.

    // Todo: I would likely need to still split by spaces (32) to get the index of each word.
    // Probably still add to a collection and perform a search on the Vector to find the match.
    // Though it may even be more efficient to just return the answer when collecting, as once it's found we shouldn't need to continue collecting parts together.
    // Future for me to review and attempt.
    
    // 78, 101, 109, 111 = Nemo in UTF-8 bytes.
    // Should check for sequence 32, 78, 101, 109, 111, 32 = " Nemo "
    // The 32 check ensures previous and preceeding characters are spaces and result actually is Nemo 


    // RESULTS:
    // Searching line: I'm trying to find Nemo !
    // I found Nemo at 5
    
    // Searching line: I'm trying to find Dory !
    // I wasn't able to find Nemo :(
    
    // Searching line: Nemo is trying to find Dory !
    // I found Nemo at 1
    
    // Searching line: Nemo is trying to find Nemo !
    // I found Nemo at 1
    
    // Searching line: Nemod is trying to find Nemo !
    // I found Nemo at 6
    
    // Searching line: To find Nemo I must !
    // I found Nemo at 3
    
    // Searching line: I'm not Nemo, Nemo is Nemo !
    // I found Nemo at 4
    
    // PS N:\100-Days-Of-Rust\Week-01\Day-02_Finding-Nemo\my_finding_nemo> date
    
    // Tuesday, 2 January 2024 2:49:05 PM



    // CHALLENGE INFO:
    // ## Finding Nemo

    // You're given a string of words. You need to find the word "Nemo", and return a string like this: `I found Nemo at [the order of the word you find nemo]!`.
    
    // If you can't find Nemo, return `I can't find Nemo :(`.
    
    // #### Examples
    
    // ```text
    // findNemo("I am finding Nemo !") ➞ "I found Nemo at 4!"
    
    // findNemo("Nemo is me") ➞ "I found Nemo at 1!"
    
    // findNemo("I Nemo am") ➞ "I found Nemo at 2!"
    // ```
    
    // ---
    
    // ### Notes
    
    // - `! , ? .` are always separated from the last word.
    // - Nemo will always look like Nemo, and not NeMo or other capital variations.
    // - Nemo's, or anything that says Nemo with something behind it, doesn't count as Finding Nemo.
    // - If there are multiple Nemo's in the sentence, only return for the first one.
    