fn main() {
    println!("Hello, world!");
    println!("Total count: {}", count_bst(3));
}


fn count_bst(n: u8) -> i32 {
    if n == 0 {
        return 0;
    }
    
    let mut total_unique: i32 = 0;

    let mut i = 1;
    println!("Did we crash here 1");
    
    for i in i..n { 
        println!("Count, i: {} n: {}", i, n);
        total_unique += count_bst(i-1)*count_bst(n-i);
        total_unique += 1;
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
