use std::collections::HashSet;
use std::fs;
use std::io::Error;

pub fn part_1(deltas: &Vec<i32>) -> i32 {
  deltas.iter().fold(0, |freq, delta| freq + delta)
}

pub fn part_2(deltas: &Vec<i32>) -> i32 {
  let mut duplicates = deltas
    .iter()
    .cycle()
    .scan((HashSet::new(), 0), |(acc, freq), delta| {
      let next = *freq + delta;
      let found = acc.contains(&next);

      *freq = next;
      acc.insert(next);

      if found {
        Some(Some(next))
      } else {
        Some(None)
      }
    });

  duplicates.find(|v| v.is_some()).unwrap().unwrap()
}

pub fn load_file(path: &str) -> Result<String, Error> {
  fs::read_to_string(path)
}

pub fn read_integers(input: &Vec<&str>) -> Vec<i32> {
  input
    .iter()
    .map(|line| line.parse::<i32>().unwrap())
    .collect()
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_part_1() {
    let test_data = vec![1, 1, 1];
    assert_eq!(part_1(&test_data), 3);

    let test_data = vec![1, 1, -2];
    assert_eq!(part_1(&test_data), 0);

    let test_data = vec![-1, -2, -3];
    assert_eq!(part_1(&test_data), -6);
  }
}
