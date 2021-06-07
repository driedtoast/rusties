use crate::leetcode::*;

/*
1 <= nums.length <= 5000
-104 <= nums[i] <= 104
All values of nums are unique.
nums is guaranteed to be rotated at some pivot.
-104 <= target <= 104
*/
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut idx = -1;
    for x in nums {
        idx += 1;
        if target == x {
            return idx;
        }
    }
    return -1;
}

// Breaks up the arguments and runs the solution to the problem
pub fn run(args: &[String]) {
  if args.len() < 1 {
    println!("Needs 1 args: rotate number");
    return;
  }
  let target = args[0].parse::<usize>().unwrap();
  
  let numbers: Vec<i32> = vec![4,5,6,7,0,1,2];

  println!("Search array result is {:?}", search(numbers, target as i32));
}