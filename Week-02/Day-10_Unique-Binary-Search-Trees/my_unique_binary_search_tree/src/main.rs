// 16/08 - I feel like it should be pretty straight forward, 
// But it seems like it's meant to be more difficult than it should
// I don't know if it's necessary for me to implement code for a left / right structure
// Just need to iterate over every number combination. 
// Say Input n is 3
// for range 1 to 3 
// i = 1 n = 3 ++1
// call count_bst(2)
// i = 1 n = 2 ++1
// call count_bst(1)
// i = 1 n = 1 ++1
// call count_bst(0)
// return 0
// i = 2 n = 2 ++1
// call count_bst(0)
// i = 2 n = 2  /// 
////// Am thinking it should be a while loop, but really not too sure. Back to work today so not much time to sit down and think about it
////// Will attempt to tackle again tomorrow 17/01
fn main() {
    println!("Hello, world!");
    println!("Total count: {}", count_bst(3));
}


fn count_bst(n: u8) -> i32 {
    if n == 0 {
        return 0;
    }
    
    let mut total_unique: i32 = 0;

    println!("Did we crash here 1");
    let mut i = 0;
    while i < n{ 
        total_unique += 1;
        i += 1;
        println!("Count, i: {} n: {}", i, n);
        total_unique += count_bst(n-1);
    }
    
    // for num in i..n {
        
        // println!("Did we crash here 2 i: {} n:{} nu, i, n);

    // }
    total_unique
}

///////Theory////////
// I had to learn about BST, but my understanding is that a BST is built up based on the order of input
// So would I essentially, similiar to day 08 - output how many unique orders there are of numbers 1 to n
// Using example below [ [1,3,2], [3,2,1], [3,1,2], [2,1,3], [1,2,3] ] ??

// I'm only required the number output, not actually building a BST - so should I just iterate over every number order? Is this fair?
// Unless there is an algorithm that can output the amount, without me actually generating each combo and counting..
// Likely an algo exists, ill investigate that route.

// So it looks like, rather than actually generating these into an array or vec, we just create a fn(n) function that takes n and recurses over 1 to n
// Makes sense, lets see if I come up with something in rust.
///////////////
// ## Unique Binary Search Trees

// Given `n`, how many structurally unique **BST's** (binary search trees) that store values 1 ... `n`?

// ### Examples

// ```text
// Input: 3
// Output: 5
// Explanation:
// Given n = 3, there are a total of 5 unique BST's:

//    1         3     3      2      1
//     \       /     /      / \      \
//      3     2     1      1   3      2
//     /     /       \                 \
//    2     1         2                 3
// ```

// ### Constraints

// - `1 <= n <= 19`

// ---

// ### Notes

// - N/A
