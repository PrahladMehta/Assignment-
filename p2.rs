// question:2
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
      for (index, &num) in arr.iter().enumerate() {
          if num == target {
              return Some(index);
          }
      }
      None
  }
  
  fn main() {
      let arr = [1, 2, 3, 4, 5, 5, 5, 6, 7, 8];
      let target = 5;
      match first_occurrence(&arr, target) {
          Some(index) => println!("The first occurrence of {} is at index {}.", target, index),
          None => println!("{} is not found in the array.", target),
      }
  }
  
  