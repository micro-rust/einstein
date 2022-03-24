# einstein
Time and Size of structs reporting library.

---

This library aims to provide a simple and no dependencies way to track some debugging variables in your projects.
It contains one struct (`TimeReport`) and one trait (`SizeReport`).

## Usage
Add this to you `Cargo.toml`
```
[dependencies]
einstein = "0.1"
```

## `TimeReport`

`TimeReport` is a simple struct that can be used to keep track of the time it takes to complete tasks.
It has a simple API designed to be as out of the way as possible.

Example:
```
use einstein::TimeReport;
use std::time::Duration;

fn main() {
  let mut report = TimeReport::new("main-task");
  let start = report.start();
  
  // Let's do subtasks.
  for i in 0..10 {
      let subreport = do_some_work( report.subtask( format!("{}", i) ) );
      
      report.add(subreport);
  }
  
  // Here ends the report.
  report.end(start);
  
  println!("Time to the end of the report:\n{}", report);
}

fn do_some_work(mut report: TimeReport) -> TimeReport {
  let start = report.start();
  
  std::thread::sleep(Duration::from_secs(1));
  
  report.end(start);
  report
}
```

## `SizeReport`
`SizeReport` is a trait that adds an easy way to check the amount of memory used by different structs.
It comes with batteries included for the `core` and `std` builtin types. User defined types must implement it on their own.

Example:
```
fn main() {
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
```

## License
`einstein` is licensed under the terms of both MIT license and Apache License (Version 2.0).

See [LICENSE-MIT] and [LICENSE-APACHE] for details.
