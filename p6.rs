// question-6
fn longest_common_prefix(strings: &[&str]) -> String {
      if strings.is_empty() {
          return String::new();
      }
      
      let mut prefix = String::new();
      for (i, c) in strings[0].chars().enumerate() {
          for s in &strings[1..] {
              if let Some(sc) = s.chars().nth(i) {
                  if sc != c {
                      return prefix;
                  }
              } else {
                  return prefix;
              }
          }
          prefix.push(c);
      }
      prefix
  }
  
  fn main() {
      let strings = vec!["flower", "flow", "flight"];
      let lcp = longest_common_prefix(&strings);
      println!("Longest common prefix: {}", lcp);
  }
  