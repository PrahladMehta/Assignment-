// question-4
fn is_prime(num: u64) -> bool {
      if num <= 1 {
          return false;
      }
      if num == 2 {
          return true;
      }
      for i in 2..=num / 2 {
          if num % i == 0 {
              return false;
          }
      }
      true
  }
  
  fn main() {
      let num = 17;
      if is_prime(num) {
          println!("{} is a prime number.", num);
      } else {
          println!("{} is not a prime number.", num);
      }
  }
  