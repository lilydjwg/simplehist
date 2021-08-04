use std::io::BufRead;

use histo::Histogram;

fn main() {
  let mut histogram = Histogram::with_buckets(10);
  let stdin = std::io::stdin();
  for line in stdin.lock().lines() {
    let line = line.unwrap();
    histogram.add(line.parse().expect("integer"));
  }
  println!("{}", histogram);
}
