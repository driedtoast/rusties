use crate::leetcode::*;
use std::convert::TryInto;
use std::ops::RangeBounds;


pub fn rotate(nums: &mut Vec<i32>, k: i32) -> String {
    let mut idx_start = k;
    let mut start_pos:usize = k.try_into().unwrap();
    if(k as usize) > nums.len() {
        start_pos = (start_pos - nums.len());
        idx_start = start_pos.try_into().unwrap();        
    }
    
    let (l, r) = nums.split_at_mut(start_pos.try_into().unwrap());
    
    //nums.append(&mut home_slice(nums, idx_start)); 
    
    return "ok".to_string();
    //return (&nums.into_iter().map(|item| format!("{:?}", item)).collect::<String>()).to_string();
}

fn home_slice(nums: &mut Vec<i32>, idx_start: i32) -> Vec<i32> {
    let end_idx = idx_start.try_into().unwrap();
    let first_part = nums.drain(std::ops::RangeTo::<usize> { end: end_idx } ); 
    return first_part.as_slice().to_vec()
}


// Breaks up the arguments and runs the solution to the problem
pub fn run(args: &[String]) {
  if args.len() < 1 {
    println!("Needs 1 args: rotate number");
    return;
  }
  let rotate_count = args[0].parse::<usize>().unwrap();
  
  let mut numbers: Vec<i32> = vec![1, 0, 2, 4, 6, 8];

  println!("Rotating array result is {:?}", rotate(&mut numbers, rotate_count as i32));
}