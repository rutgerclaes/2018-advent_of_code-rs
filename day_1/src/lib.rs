//! # Day 1 of Advent of Code
//!
//! `day_1` contains the solution to the first puzzle of the 2018 edition of
//! [Advent of Code](https://adventofcode.com/2018/day/1)

use std::collections::HashSet;

/// ## Part 1
/// Given a list of frequency changes, and starting from 0, calculate the
/// resulting frequency.
///
/// ### Example
///
/// ```
/// let input = vec![1, -2, 3, 1 ];
/// assert_eq!( day_1::part_1( &input ), 3 );
/// ```
pub fn part_1(deltas: &Vec<i32>) -> i32 {
  deltas.iter().fold(0, |freq, delta| freq + delta)
}

/// ## Part 2
/// Given a list of frequency changes, and starting from 0, find the first
/// frequency that occurs twice.
///
/// ### Example
/// ```
/// let input = vec![1, -2, 3, 1 ];
/// assert_eq!( day_1::part_2( &input ), 2 );
/// ```
pub fn part_2(deltas: &Vec<i32>) -> i32 {
  let mut initial = HashSet::new();
  initial.insert(0);
  let mut duplicates = deltas
    .iter()
    .cycle()
    .scan((initial, 0), |(acc, freq), delta| {
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

/// # Read a vector of strings as integers
///
/// ### Example
/// ```
/// let input = vec![ "-1", "1", "+1", "-10" ];
/// assert_eq!( day_1::read_integers( &input ), vec![ -1, 1, 1, -10 ] );
/// ```
///
/// ### Panics
/// Panics if one of the input values cannot be parsed as `i32`.
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
    let test_data = vec![1, -2, 3, 1];
    assert_eq!(part_1(&test_data), 3);

    let test_data = vec![1, 1, 1];
    assert_eq!(part_1(&test_data), 3);

    let test_data = vec![1, 1, -2];
    assert_eq!(part_1(&test_data), 0);

    let test_data = vec![-1, -2, -3];
    assert_eq!(part_1(&test_data), -6);
  }

  #[test]
  fn test_part_2() {
    let test_data = vec![1, -2, 3, 1];
    assert_eq!(part_2(&test_data), 2);

    let test_data = vec![1, -1];
    assert_eq!(part_2(&test_data), 0);

    let test_data = vec![3, 3, 4, -2, -4];
    assert_eq!(part_2(&test_data), 10);

    let test_data = vec![-6, 3, 8, 5, -6];
    assert_eq!(part_2(&test_data), 5);

    let test_data = vec![7, 7, -2, -7, -4];
    assert_eq!(part_2(&test_data), 14);
  }

  #[test]
  fn test_read_integers() {
    let test_data = vec!["+1", "-2", "3", "1"];
    assert_eq!(read_integers(&test_data), vec![1, -2, 3, 1]);
  }

}
