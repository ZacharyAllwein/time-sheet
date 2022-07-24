use std::error;

#[derive(Debug)]
pub struct Time {

  pub hours: u8,
  pub minutes: u8,
}

pub fn new(hours: u8, minutes: u8) -> Time{
  return Time {hours, minutes};
}

pub fn from_str(time: &str) -> Result<Time, Box<dyn error::Error>> {
  

  let time_list = time
    .split(':')
    .map(|num| num.parse::<u8>().unwrap())
    .collect::<Vec<u8>>();

  if time_list.len() != 2usize {
    
    return Err("from_str only takes strings in the format of 'hours:minutes'".into());
  }
  else {
    return Ok(Time { hours: time_list[0], minutes: time_list[1]});
  }

}

pub fn elapsed(time1: &Time, time2: &Time) -> Time {
  
  let min_dif = time2.minutes as i8 - time1.minutes as i8;
  let hours = (time2.hours) - (time1.hours);
      
  if min_dif < 0 {
      return new(hours - 1, (60 + min_dif) as u8);
  }
  else {
    return new(hours, min_dif as u8);
  }
}

pub fn add(time1: &Time, time2: &Time) -> Time {
  
  let combined_minutes = time1.minutes + time2.minutes;

  println!("{}", time1.minutes);

  return new(time1.hours + time2.hours + ((combined_minutes) / 60),
            (combined_minutes) % 60)
}




