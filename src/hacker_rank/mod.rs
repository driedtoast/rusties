pub mod command;
mod cat_and_mouse;
mod repeated_string;


fn substring(s: &str, start: usize, len: usize) -> &str {
   return &s[start.. s.char_indices().nth(len).unwrap().0];
}