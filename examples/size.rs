fn main() {
  use einstein::SizeReport;
  use std::collections::HashMap;

  // Let's check how much memory does a Hashmap<String, Vec<usize>> use.
  let mut map = HashMap::new();
  
  // Start adding new stuff to it.
  for i in 0..10 {
    let key = format!("{}", i);
    let mut val = Vec::new();
    
    for _ in 0..i {
      val.push(0usize)
    }
    
    map.insert(key, val);
  }
  
  println!("Size of the hashmap: {} bytes", map.fullsize());
  println!("Direct (pointer + size + capacity + misc): {} bytes", HashMap::<String, Vec<usize>>::direct());
  println!("Indirect (Actual size of keys and values): {} bytes", map.indirect());
  println!("Indirect (Size collected of all children): {} bytes", map.children());
}
