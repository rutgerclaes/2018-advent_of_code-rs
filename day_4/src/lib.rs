extern crate regex;

use regex::Regex;
use std::collections::HashMap;

/// # Accumulate all events in the log per guard and per minute.
//
/// ```
/// let input = vec![
///     "[1518-11-01 00:00] Guard #1 begins shift",
///     "[1518-11-01 00:05] falls asleep",
///     "[1518-11-01 00:06] wakes up",
///     "[1518-11-01 00:08] falls asleep",
///     "[1518-11-01 00:10] wakes up",
///     "[1518-11-01 23:58] Guard #2 begins shift",
///     "[1518-11-02 00:30] falls asleep",
///     "[1518-11-02 00:32] wakes up",
///     "[1518-11-03 00:04] Guard #1 begins shift",
///     "[1518-11-03 00:05] falls asleep",
///     "[1518-11-03 00:10] wakes up"
///   ];
/// let accumulated_events = day_4::accumulate_events(input);
/// let mut first_guard: Vec<(&u8,&u32)> = accumulated_events.get(&1).unwrap().iter().collect();
///
/// first_guard.sort_by_cached_key( |(&min,_)| min );
/// assert_eq!(first_guard, vec!((&5,&2), (&6,&1), (&7,&1),(&8,&2),(&9,&2)));
///
/// second_guard.sort_by_cached_key( |(&min,_)| min );
/// assert_eq!(second_guard, vec!((&30,&1), (&31,&1)));
/// ```
pub fn accumulate_events(mut lines: Vec<&str>) -> HashMap<u32, HashMap<u8, u32>> {
  lines.sort();
  let events = lines.iter().map(|line| GuardEvent::parse(line));
  let mut last_guard = None;
  let mut asleep_since = None;

  let mut accumulator: HashMap<u32, HashMap<u8, u32>> = HashMap::new();
  for event in events {
    match (event, last_guard, asleep_since) {
      (GuardEvent::ShiftStart(next_guard), _, _) => {
        asleep_since = None;
        last_guard = Some(next_guard);
      }
      (GuardEvent::FallsAsleep(minutes), _, _) => asleep_since = Some(minutes),
      (GuardEvent::WakesUp(minutes), Some(guard), Some(since)) => {
        let series = accumulator.entry(guard).or_insert_with(HashMap::new);
        for minute in since..minutes {
          let count = series.entry(minute).or_insert(0);
          *count += 1;
        }
      }
      _ => panic!("Inconsistent state reached"),
    }
  }

  accumulator
}

pub fn part_1(lines: Vec<&str>) -> u32 {
  let accumulated_events = accumulate_events(lines);
  let (guard_number, minutes) = accumulated_events
    .iter()
    .max_by_key(|(_, minutes)| minutes.values().fold(0, |sum, count| sum + count))
    .unwrap();

  let chosen_minute = minutes.iter().max_by_key(|(_, &count)| count).unwrap().0;
  guard_number * u32::from(*chosen_minute)
}

pub fn part_2(lines: Vec<&str>) -> u32 {
  let accumulated_events = accumulate_events(lines);
  let (guard_number, minutes) = accumulated_events
    .iter()
    .max_by_key(|(_, minutes)| minutes.iter().max_by_key(|(_, &count)| count).unwrap().1)
    .unwrap();

  let chosen_minute = minutes.iter().max_by_key(|(_, &count)| count).unwrap().0;
  guard_number * u32::from(*chosen_minute)
}

#[derive(Debug, PartialEq)]
pub enum GuardEvent {
  ShiftStart(u32),
  FallsAsleep(u8),
  WakesUp(u8),
}

impl GuardEvent {
  /// # Parse a problem input line
  ///
  /// ## Example
  /// ```
  /// let input = "[1518-11-01 00:00] Guard #10 begins shift";
  /// let event = day_4::GuardEvent::parse( input );
  /// assert_eq!( event, day_4::GuardEvent::ShiftStart( 10 ) );
  ///
  /// let input = "[1518-11-01 00:05] falls asleep";
  /// let event = day_4::GuardEvent::parse( input );
  /// assert_eq!( event, day_4::GuardEvent::FallsAsleep( 5 ) );
  ///
  /// let input = "[1518-11-01 00:25] wakes up";
  /// let event = day_4::GuardEvent::parse( input );
  /// assert_eq!( event, day_4::GuardEvent::WakesUp(25) );
  /// ```
  pub fn parse(input: &str) -> GuardEvent {
    let guard_regex = Regex::new(r"^.* Guard #([0-9]+) begins shift$").unwrap();
    let asleep_regex = Regex::new(r"^\[[0-9 -]+:([0-9]{2})\] falls asleep$").unwrap();
    let wake_regex = Regex::new(r"^\[[0-9 -]+:([0-9]{2})\] wakes up$").unwrap();

    if guard_regex.is_match(input) {
      let guard_number = guard_regex
        .captures(input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap();
      GuardEvent::ShiftStart(guard_number)
    } else if asleep_regex.is_match(input) {
      let minutes = asleep_regex
        .captures(input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u8>()
        .unwrap();
      GuardEvent::FallsAsleep(minutes)
    } else if wake_regex.is_match(input) {
      let minutes = wake_regex
        .captures(input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u8>()
        .unwrap();
      GuardEvent::WakesUp(minutes)
    } else {
      panic!("Couldn't parse {}", input)
    }
  }
}
