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
