fn main() {
    println!("Hello, world!");
    let test_case: Vec<u8> = vec![0,6,1,6,4,0];
    assert_eq!(is_trapped(build_array(test_case)), 5); 

}

fn build_array(lengths:Vec<u8>) -> Vec<u8> {
    let mut blocks: Vec<u8> = vec![];
    let mut largest: u8 = 0;
    
    for item in lengths.iter(){ // Get the largest item in the array
        println!("i: {}", item);
        if item > &largest {
            largest = *item;
        }
    }
    println!("Max value in array: {:?}", largest);
    assert_eq!(6, largest);
    // Input array like [0,6,1,2,4,0]
    // Largest == n = 6
    // j == 0
    // for i in [0..6] { if lengths[i] <= n {blocks.push(1)} else {blocks.push(0)} } j += 1
    // keep going til j == n
    // Output Vec like [
        // [0, 1, 0, 0, 0, 0],
        // [0, 1, 0 ,0, 0, 0],
        // [0, 1, 0, 0, 1, 0],
        // [0, 1, 0, 0, 1, 0],
        // [0, 1, 0, 1, 1, 0],
        // [0, 1, 1, 1, 1, 0]
        // ]
    blocks
}

fn is_trapped(blocks:Vec<u8>) -> u8{
    let mut count: u8 = 0;
    // Input vec like         
        // [0, 1, 0, 0, 0, 0],
        // [0, 1, 0 ,0, 0, 0],
        // [0, 1, 0, 0, 1, 0],
        // [0, 1, 0, 0, 1, 0],
        // [0, 1, 0, 1, 1, 0],
        // [0, 1, 1, 1, 1, 0]
        // ]
    // Output count like 5
    count
}

// Method Idea:
// 1. Get the largest number in case of: [4,2,0,3,2,5] being 5
// 2. Vectory a new array length of {5}
// 3. 0 if it doesn't reach height, 1 if it does.  Looks like this:
// [0, 0, 0, 0, 0, 1]
// [1, 0, 0, 0, 0, 1]
// [1, 0, 0, 1, 0, 1]
// [1, 1, 0, 1, 1, 1]
// [1, 1, 0, 1, 1, 1]
// Set variable count = 0
// 4. for i in range[0..len]{ if i == 1 check(next) {count +=1} if end(1) keep count else drop count
// 5. Iterate over each array
// 6. return count

// ## Trapping Rain Water

// Given `n` non-negative integers representing an elevation map where the width of each bar is `1`, compute how much water it can trap after raining.

// ### Examples

// **Example 1:**

// <p align="left">
//   <img src="../../assets/rainwatertrap.png" alt="Rain water trap">
// </p>

// ```text
// Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
// Output: 6
// Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
// ```

// **Example 2:**

// ```text
// Input: height = [4,2,0,3,2,5]

// Output: 9
// ```

// ### Constraints

// - `n == height.length`
// - `0 <= n <= 3 * 104`
// - `0 <= height[i] <= 105`

// ---

// ### Notes

// - N/A
