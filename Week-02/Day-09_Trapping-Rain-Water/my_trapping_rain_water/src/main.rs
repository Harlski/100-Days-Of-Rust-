// Answer for this one came fairly quickly
// Compared to Day 08, I had more confidence coming into this
// While I still feel as though my answer may not be idiomatic
// I feel as though I may be becoming more familiar with what should come next
// The only thing I could probably change here is changing the order in which 
// Build Array pushes the array, but regardless the solution is still reachable without this correct ordering.

fn main() {
    let test_case: Vec<u16> = vec![4,2,0,3,2,5];

    println!("Answer: {}", is_trapped(build_array(test_case.clone())));
    println!("Input: {:?}", test_case);

}

fn build_array(list:Vec<u16>) -> Vec<Vec<u16>> {
    
    let mut largest: u16 = 0;
    for item in list.iter(){ // Get the largest item in the array
        // println!("i: {}", item);
        if item > &largest {
            largest = *item;
        }
    }
    // println!("Max value in array: {:?}", largest);
    // assert_eq!(6, largest);
    
      
    
    let mut blocks: Vec<Vec<u16>> = vec![];
    for i in 0..largest {
        // println!("First loop: {:?}", i);
        let mut temp_v: Vec<u16> = vec![];
        for j in &list {
            if j >= &(i + 1) {
                // println!("1 - j is equal or greater than current height {} < {}", j, &(i+1));
                temp_v.push(1); 
            } else { 
                // println!("0 - j is less than current height {} > {}", j, &(i+1));
                temp_v.push(0);  
            }
        }
        blocks.push(temp_v.clone());
        // println!("Temp_v: {: ?}", temp_v);
    }
    blocks
    }
    
    fn is_trapped(blocks:Vec<Vec<u16>>) -> u16{
        let mut count: u16 = 0; // Total count of captured blocks
        // println!("Blocks: {:?}", blocks);
        
        for item in blocks.iter() {
            let mut may_trap = false; // If true, we start counting the potential trapped blocks
            let mut temp_count: u16 = 0; // Temporary measure of captured blocks

            for number in item.iter(){
                let mut none_met = true; // If none of the conditions below are met, then we acknowledge via print. Purely for logging asthetics. 
                if *number == (1 as u16) && !may_trap { // This condition basically declares that there is a wall and potential for next blocks to be "caught"
                    may_trap = true;
                    none_met = false;
                    println!("{:?} {:?} May Now Trap", item, number);
                }
                if may_trap && *number == (0 as u16) { // If there was a wall increase count by one
                    temp_count += 1;
                    none_met = false;
                    println!("{:?} {:?} Increasing Temp Count + 1", item, number);
                }
                if *number == (1 as u16) && temp_count > 0 { // If another wall is found with a temp count above 0, then it's considered captured - add temp to count
                    count += temp_count;
                    temp_count = 0;
                    none_met = false;
                    println!("{:?} {:?} Setting count to Temp Count, Temp_Count to 0, May_Trap now false Count now: {}", item, number, count);
                } 
                if none_met {
                    println!("{:?} {:?} No conditions met.", item, number); // Purely for asthetic logging.
                }
            }
            println!(" "); // Purely for logging asthetics.
            temp_count = 0; // Reset temp count and may_trap if array ends and no other wall is found.
            may_trap = false; 
        }
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


// // Results:
// Answer: 9
// Input: [4, 2, 0, 3, 2, 5]
// harlski@harlski-ub:~/Documents/100-Days-Of-Rust-/Week-02/Day-09_Trapping-Rain-Water/my_trapping_rain_water$ date
// Sun 14 Jan 2024 10:32:03 AM AEDT


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