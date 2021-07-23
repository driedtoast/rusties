use crate::leetcode::*;

 fn fib(n: i32) -> i32 {
    if n <= 1 return n;

    return fib(n - 1) + fib(n - 2);
 }

 pub fn climb_stairs(n: i32) -> i32 {

    
        
 }

// Breaks up the arguments and runs the solution to the problem
pub fn run(args: &[String]) {
  if args.len() < 1 {
    println!("Needs 1 args: stair count");
    return;
  }
  let stair_count = args[0].parse::<usize>().unwrap();
  
  println!("Stair climbing result is {:?}", climb_stairs(stair_count as i32));
}    