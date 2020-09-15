
// Solution for https://www.hackerrank.com/challenges/cats-and-a-mouse/problem
fn cat_and_mouse_call(cat_a: i32, cat_b: i32, mouse_c: i32) -> String {
  let distance_a = (mouse_c - cat_a).abs();
  let distance_b = (mouse_c - cat_b).abs();
  if distance_a == distance_b {
      return "Mouse C".to_string();
  } else if distance_a > distance_b {
      return  "Cat B".to_string();
  }
  return  "Cat A".to_string();
}

// Breaks up the arguments and runs the solution to the problem
pub fn run(args: &[String]) {
  if args.len() < 3 {
    println!("Needs 3 args");
    return;
  }
  // my_string.parse::<i32>().unwrap()
  let cat_a = args[0].parse::<i32>().unwrap();
  let cat_b = args[1].parse::<i32>().unwrap();
  let mouse_c = args[2].parse::<i32>().unwrap();
  println!("Cat and Mouse result is {:?}", cat_and_mouse_call(cat_a, cat_b, mouse_c));
}