use std::collections::HashMap;


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Create a hash map to store the difference between target and each number in nums
    let mut hm = HashMap::with_capacity(nums.len());
    for (i, &num) in nums.iter().enumerate() {
        // Check if such a difference exists in the hash map
        let xx = hm.get(&num).unwrap_or(&0);
        println!("{}", xx);
        match hm.get(&num) {
            // If it does, return the indices of the current number and the number with the difference
            Some(&j) => return vec![i as i32, j as i32],
            // If it doesn't, add the difference between target and the current number to the hash map
            None => {
                hm.insert(target - num, i);
            },
        }
    }
    unreachable!();
}

fn main() {
    let arr = vec![5, 2, 7, 3, 9, 2];
    let target = 8;

    let rasult = two_sum(arr, target);
    println!("{:?}", rasult)
}