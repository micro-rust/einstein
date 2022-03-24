fn main() {
  use einstein::SizeReport.

  // Let's check how much memory does a Hashmap<String, Vec<usize>> use.
  let mut map = std::collections::Hashmap::new();
  
  // Start adding new stuff to it.
  for i in 0..10 {
    let key = format!("{}", i);
    let mut val = Vec::new();
    
    for j in 0..i {
      val.push(0usize)
    }
    
    map.insert(key, val);
  }
  
  println!("Size of the hashmap:\n{}", map.fullsize());
}
