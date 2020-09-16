use crate::hacker_rank::*;

/**

// Complete the repeatedString function below.
    static long repeatedString(String s, long n) {
        String strToCheck = s;
        double countTimes = 1;
        if(n < s.length()) {
            strToCheck = strToCheck.substring(0, (int)(n));
        } else if (n > s.length()){
            // not cast to double so it will be rounded
            countTimes = (n / strToCheck.length());
        }

        long count = strLength(strToCheck);
        count = Math.round(countTimes * count);
        // check the remainder of it
        int remainder = (int)n % s.length();
        strToCheck = s.substring(0,remainder);
        return count + strLength(strToCheck);
    }

**/


// Solution for https://www.hackerrank.com/challenges/repeated-string/problem
fn repeated_string_call(s: &String, n: usize) -> usize {
  let mut str_to_check = s.as_str();
  let mut count_times = 1;

  let str_len = str_to_check.chars().count();
  if n < str_len {
      str_to_check = substring(str_to_check, 0, n);      
  } else if n > str_len {
      count_times = n / str_len;
  }
  
  let count = count_times * str_length(s);
  let remainder = n % str_len;
  str_to_check = substring(s, 0, remainder);
  return count + str_length(str_to_check);
}

fn str_length(str_to_check: &str) -> usize {
    let mut count = 0;
    for c in str_to_check.chars() {
        if c == 'a' {
            count = count+1;
        }
    }
    return count;
}

// Breaks up the arguments and runs the solution to the problem
pub fn run(args: &[String]) {
  if args.len() < 2 {
    println!("Needs 2 args: string to repeat (aba) and length");
    return;
  }
  let string_length = args[1].parse::<usize>().unwrap();
  let string_to_repeat = &args[0];
  
  println!("Repeated string result result is {:?}", repeated_string_call(string_to_repeat, string_length));
}