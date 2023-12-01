use std::env;

pub fn get_day_from_env() -> Option<u8> {
  let string_day = env::args()
  .nth(1)
  .unwrap();

  if !string_day.starts_with("day") {
    return None
  }

  if let Ok(day) = string_day[3..].parse::<u8>() {
    return Some(day)
  }

  None
}

pub fn get_test_from_env() -> bool {
  if let Some(test) = env::args().nth(2) {
    return test == "test"
  }

  false
}
