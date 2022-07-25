mod time;

use std::env::args;
use std::process;

fn main() {
  
  let times = args()
    .skip(1)
    .map(|arg| time::from_str(&arg))
    .collect::<Result<Vec<time::Time>, _>>();

  let times = match times {
    Ok(ts) => ts,
    _ => process::exit(1)
  };

  let elapsed = time::elapsed_series(times).unwrap();
  
  println!("{:?}", elapsed);
}
