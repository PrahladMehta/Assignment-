// question-7

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
      if k == 0 || k > arr.len() {
          return None;
      }
  
      let mut sorted_arr = arr.to_vec();
      sorted_arr.sort();
  
      Some(sorted_arr[k - 1])
  }
  
  fn main() {
      let arr = [3, 1, 7, 5, 4, 2, 6];
      let k = 3;
      match kth_smallest(&arr, k) {
          Some(smallest) => println!("The {}th smallest element is: {}", k, smallest),
          None => println!("Invalid value of k."),
      }
  }
  