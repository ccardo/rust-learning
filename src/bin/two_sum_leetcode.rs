fn main() {
    let x = solution(vec![1, 4, 5, 3, 7], 7).unwrap();
    println!("{:?}", x)
}

fn solution(nums: Vec<i32>, target: i32) -> Option<Vec<i32>> {

    let mut soln: Vec<i32> = vec![];
    for (i, x) in nums.iter().enumerate() {
        if nums.contains(&(target - x)) {
            soln = vec![*x, nums.iter().position(|&e| e == target - x).unwrap() as i32];
        } else { return None; }
    };
    Some(soln)
}