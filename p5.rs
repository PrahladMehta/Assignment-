// question-5

fn find_median(arr: &[i32]) -> f64 {
      let mut sorted_arr = arr.to_vec();
      sorted_arr.sort();
  
      let len = sorted_arr.len();
      if len % 2 == 0 {
          let mid = len / 2;
          let median = (sorted_arr[mid - 1] + sorted_arr[mid]) as f64 / 2.0;
          median
      } else {
          let mid = len / 2;
          sorted_arr[mid] as f64
      }
  }
  
  fn main() {
      let arr = [3, 1, 7, 5, 4, 2, 6];
      let median = find_median(&arr);
      println!("The median of the array is: {}", median);
  }
  