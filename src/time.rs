use std::error;
use std::fmt;

#[derive(Debug)]
pub struct Time {

  pub hours: u8,
  pub minutes: u8,
}


impl fmt::Display for Time{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut mins = self.minutes.to_string();
    
    if mins.len() < 2{
      mins = format!("0{}", mins);
    }  
    write!(f, "{}:{}", self.hours, mins)
  }
}
pub fn new(hours: u8, minutes: u8) -> Time{
  return Time {hours, minutes};
}

pub fn from_str(time: &str) -> Result<Time, Box<dyn error::Error>> {
  

  let time_list = time
    .split(':')
    .map(|num| num.parse::<u8>())
    .collect::<Result<Vec<u8>, _>>()?;

  if time_list.len() != 2usize {
    
    return Err("from_str only takes strings in the format of 'hours:minutes'".into());
  }
  else {
    return Ok(Time { hours: time_list[0], minutes: time_list[1]});
  }

}

pub fn sub(time1: Time, time2: Time) -> Time {
  
  let min_dif = time2.minutes as i8 - time1.minutes as i8;
  let hours = (time2.hours) - (time1.hours);
      
  if min_dif < 0 {
      return new(hours - 1, (60 + min_dif) as u8);
  }
  else {
    return new(hours, min_dif as u8);
  }
}

pub fn add(time1: Time, time2: Time) -> Time {
  
  let combined_minutes = time1.minutes + time2.minutes;

  return new(time1.hours + time2.hours + ((combined_minutes) / 60),
            (combined_minutes) % 60)
} 

fn calculate_elapsed_series(acc: Time, mut series: impl Iterator<Item=Time>) -> Time {

  return match series.next() {
    Some(time) => calculate_elapsed_series(add(acc, sub(time, series.next().unwrap())), series),
    None => return acc,
 };

}




pub fn elapsed_series(series: Vec<Time>) -> Result<Time, &'static str> {

  if series.len() % 2 != 0 {
    return Err("odd number of times in arguments");
  }

  return Ok(calculate_elapsed_series(new(0, 0), series.into_iter()));
}
