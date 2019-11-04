//! # Day 2 of Advent of Code
//!
//! `day_2` contains the solution to the second puzzle of the 2018 AOC edition.

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

/// ## First part of the exercise
///
/// ### Examples
/// ```
/// let input = vec![ "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab" ];
/// assert_eq!( day_2::part_1( &input ), 12 );
/// ```
pub fn part_1(input: &Vec<&str>) -> i32 {
  let (twice, trice) = input.iter().fold((0, 0), |(twice, trice), string| {
    let counts = letter_counts(string);
    let inc_twice = if counts.contains(&2) { 1 } else { 0 };
    let inc_trice = if counts.contains(&3) { 1 } else { 0 };
    (twice + inc_twice, trice + inc_trice)
  });

  twice * trice
}

/// ## Check if there is a character occuring exactly twice in the input string
///
/// ### Example
/// ```
/// let input = "abcda";
/// assert_eq!( day_2::letter_counts( &input ), vec![1, 2].iter().cloned().collect() );
///
/// let input = "abcabcabc";
/// assert_eq!( day_2::letter_counts( &input ), vec![3].iter().cloned().collect() );
///
/// let input = "aba";
/// assert_eq!( day_2::letter_counts( &input ), vec![1, 2].iter().cloned().collect() );
///
/// let input = "ababac";
/// assert_eq!( day_2::letter_counts( &input ), vec![1,2,3].iter().cloned().collect() );
/// ```
pub fn letter_counts(string: &str) -> HashSet<i32> {
  let char_counts = string.chars().fold(HashMap::new(), |mut counts, ch| {
    let count = counts.entry(ch).or_insert(0);
    *count += 1;
    counts
  });

  HashSet::from_iter(char_counts.values().copied())
}

/// ## Second part of the exercise
///
/// Look for a pair of words that only differ in one character at any given place and
/// return the string of common characters between those two words.
///
/// ### Example
///
/// ```
/// let input = vec![ "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz" ];
/// assert_eq!( day_2::part_2( &input ), "fgij" );
/// ```
pub fn part_2(input: &Vec<&str>) -> String {
  let mut pairs = input
    .iter()
    .enumerate()
    .flat_map(|(index, word)| input.iter().skip(index + 1).map(move |other| (word, other)));

  let correct = pairs
    .find(|(first, second)| correct_input(first, second))
    .expect("Could not find correct pair");
  common_string(correct.0, correct.1)
}

/// ### Get the common part of two correct words
///
/// Return the string consisting only of the characters that are identical
/// at each index of the two input strings.
///
/// ```
/// let a = "fghij";
/// let b = "fguij";
///
/// assert_eq!( day_2::common_string( a, b ), "fgij" );
/// assert_eq!( day_2::common_string( b, a ), "fgij" );
/// ```
pub fn common_string(a: &str, b: &str) -> String {
  let chars = a
    .chars()
    .zip(b.chars())
    .filter(|(first, second)| first == second)
    .map(|(first, _)| first);
  String::from_iter(chars)
}

/// ### Check if two strings are considered "close enough"
///
/// Two strings are considered "close enough" if they have
/// differ by only 1 character at a certain index in the string.
///
/// ```
/// let a = "fghij";
/// let b = "fguij";
///
/// assert_eq!( day_2::correct_input( a, b ), true );
/// assert_eq!( day_2::correct_input( b, a ), true );
///
/// let c = "abcde";
/// let d = "axcye";
///
/// assert_eq!( day_2::correct_input( c, d ), false );
/// assert_eq!( day_2::correct_input( d, d ), false );
/// assert_eq!( day_2::correct_input( c, c ), false );
/// assert_eq!( day_2::correct_input( a, c ), false );
/// assert_eq!( day_2::correct_input( a, d ), false );
/// ```
pub fn correct_input(a: &str, b: &str) -> bool {
  if a.len() != b.len() {
    false
  } else {
    let diff = a
      .chars()
      .zip(b.chars())
      .fold(0, |count, (a, b)| if a == b { count } else { count + 1 });
    diff == 1
  }
}
