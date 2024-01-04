fn main() {
    let mut nums1 = vec![1,2,3,0,0,0];
    let mut nums2 = vec![2,5,6];
    let m = 3;
    let n = 3;

    merge_arrays(&mut nums1, m, &mut nums2, n);

    // let NUMS1:[usize; 6] = [1,2,3,0,0,0];
    // const NUMS2:[usize; 3] = [2,5,6];

    // merge_arrays(NUMS1,NUMS2);
}

fn merge_arrays(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32){
    println!("Nums1: {:?}, m: {}, Nums2: {:?}, n: {}", nums1, m, nums2, n);
    // I used ChatGPT here to show me the solution, it met my logical reasoning - however I'm not referring to it to create the code below
    // I'll attempt to action the rest of the code with only my initial glance at the solution - I'll refer to documentation from here on out.
    let mut i = m - 1; // Last index of nums1 of actual numbers
    let mut j = n - 1; // Last index of nums2
    let mut k = m + n - 1; // Actual end of index of nums1

    while i >= 0 && j >= 0 { // Iterates from back to front, stops at 0
        if nums1[i as usize] > nums2[j as usize]{
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        k -= 1;
    }
    println!("After running: {:?}", nums1);
    // Admittedly, I went back to ChatGPT and copied the code verbatim. More in Notes below.
}


// fn merge_arrays(mut n1:[usize; 6], n2:[usize; 3]){
//     let n_len1 = n1.len();
//     let n_len2 = n2.len();
//     println!("n1: {:?}, n2: {:?}, len1: {}, len2: {}", n1,n2,n_len1,n_len2);

//     let mut n2_index:&mut usize = 0;
//     for (index, element) in n1.iter_mut().enumerate() {
//         println!("Index: {} | Element: {}", index, element);
//         if index >= n_len2 {
//             return
//         }

//         if element >= n2[*&mut n2_index]{
//             let mut current_index = index;
//             for e in n1[index..n_len1].iter_mut().enumerate(){
//                 println!("Moving {} into {}", n1[n_len1-1], n1[n_len1]);
//                 n1[n_len1] = n1[n_len1 -1];
//             }
//             n1[index] = n2[index];
//             n2_index += 1
//         }
//         println!("N2 Index: {}", n2[index]);
//     }
//     // for n in n2{
//     //     if n
//     // } 
// }


// fn merge_arrays(nums1:[i32; 3], nums2:[i32; 3]){
//     let n_len1 = nums1.len();
//     let n_len2 = nums2.len();

//     println!("Combined len is {} ({} + {})", n_len1 + n_len2, n_len1, n_len2);
//     print_type_of(&nums1.len());

//     let array_size: i32 = (n_len1 + n_len2).try_into().unwrap();
//     println!("Array_Size: {}", array_size);
//     print_type_of(&array_size);
//     // let mut merged_array:[i32; array_size as usize] = [0; 6];
//     // println!("Merged Array: {}", merged_array);

// }


// fn print_type_of<T>(_: &T){
//     println!("{}", std::any::type_name::<T>())
// }


// Harley' Notes:
// This was a tricky one, it tested me quite a bit and sent me down a rabbit hole.
// While I haven't legitimately completed this challenge, I'm still counting it as a win.
// While I didn't come to the conclusion myself, the initial routes I went down trying to complete this challenge was education enough
// Admittedly, I've never written a while loop like this before. While it simply makes sense (Start in reverse order and compare, as arrays are already sorted).
// I feel like my downfall was changing the scope of the challenge itself, as I wanted to be able to pass any two unknown length arrays and reach challenge conclusion
// I went through various different methods as theory notes below and commented code above that made sense in my head required knowledge I haven't yet learned
// Such as trying to create a new dynamic array inside of the function (but learning that array lengths are defined at compile time and not runtime) so I had to go Vec route.
// While all my writing and theory may not make sense, take from it what you will. It helps to at least cement the core ideas of Rust regardless.


// Theory Notes:
// From what I can tell there is no explicit merge method to call to do this easily
// I may even be going about this challenge the wrong way, I can see that nums1 should contain 6 items.
// But I've been trying to pass 2 x 3 item arrays through a merge_arrays() function.
// The function would then create a new array based on the .len() of both nums1+nums2
// Then iterate over each array to determine which one would be added next

// Now that I'm reviewing, challenge is asking me to start with a 6 item array on nums1 - which seems like it would be a lot easier to iterate over.
// I sort of want to continue proceeding with two arrays that contain only the items necessary in order to merge the array together this way. 
// As this path seems to be encouraging me to learn more about .len() == usize and constants which is my blocker at the moment when tryng to 
// Declare a new Array with the array_size of nums1.len() + nums2.len()

// So it seems as though an Array length must be declared at compile time & that we can't generate a new array with an unknown length via function
// References points me towards using a Vector to create a dynamic length array but the challenge explicitly mentions merge into one sorted array.
// I may instead try to create the initial nums1 array to have the 6 items [1,2,3,0,0,0] and then iterate and push items in as necessary
// I'll try this route...

// I could probably take the easy way out and merge => sort n1 but I think it's more of a challenge to sort while iterating
// It seems as though array may also imply using a Vector, as technically Vec is a dynamic array - so it still satisfies the challenge.
// 
// ## Merge Sorted Array

// Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one sorted array.

// ### Examples

// ```text
// Input:
// nums1 = [1,2,3,0,0,0], m = 3
// nums2 = [2,5,6],       n = 3

// Output: [1,2,2,3,5,6]
// ```

// ### Constraints

// - `-10^9 <= nums1[i], nums2[i] <= 10^9`
// - `nums1.length == m + n`
// - `nums2.length == n`

// ---

// ### Notes

// - The number of elements initialized in nums1 and nums2 are m and n respectively.
// - You may assume that nums1 has enough space (size that is **equal** to m + n) to hold additional elements from nums2.
