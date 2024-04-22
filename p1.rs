// question-1
fn is_palindrome(s: &str) -> bool {
      let s = s.to_lowercase(); 
      let reversed: String = s.chars().rev().collect(); 
      s == reversed
  }
  
  fn main() {
      let word = "level";
      if is_palindrome(word) {
          println!("{} is a palindrome!", word);
      } else {
          println!("{} is not a palindrome.", word);
      }
  }
  