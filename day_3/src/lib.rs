extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

pub struct Points {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    dx: i32,
    dy: i32,
}

impl Points {
    pub fn from(claim: &Claim) -> Points {
        Points {
            x: claim.x,
            y: claim.y,
            width: claim.width,
            height: claim.height,
            dx: 0,
            dy: 0,
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.dx >= self.width {
            None
        } else if self.dy >= self.height {
            self.dy = 0;
            self.dx += 1;
            self.next()
        } else {
            let result = Point::new(self.x + self.dx, self.y + self.dy);
            self.dy += 1;
            Some(result)
        }
    }
}

impl Claim {
    /// ## Parse an input line into a claim
    ///
    /// ### Example
    /// ```
    /// assert_eq!( day_3::Claim::parse( "#1 @ 1,3: 4x4" ), day_3::Claim::new(1, 1, 3, 4, 4 ) );
    /// assert_eq!( day_3::Claim::parse( "#2 @ 3,1: 4x4" ), day_3::Claim::new(2, 3, 1, 4, 4 ) );
    /// assert_eq!( day_3::Claim::parse( "#3 @ 5,5: 2x3" ), day_3::Claim::new(3, 5, 5, 2, 3 ) );
    /// ```
    /// TODO: add error handling
    pub fn parse(input: &str) -> Claim {
        let regex = Regex::new(
            r"#(?P<id>[0-9]+) @ (?P<x>[0-9]+),(?P<y>[0-9]+): (?P<width>[0-9]+)x(?P<height>[0-9]+)",
        )
        .unwrap();
        let capt = regex.captures(input).unwrap();
        let id = capt.name("id").unwrap().as_str().parse::<i32>().unwrap();
        let x = capt.name("x").unwrap().as_str().parse::<i32>().unwrap();
        let y = capt.name("y").unwrap().as_str().parse::<i32>().unwrap();
        let width = capt.name("width").unwrap().as_str().parse::<i32>().unwrap();
        let height = capt
            .name("height")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        Claim::new(id, x, y, width, height)
    }

    pub fn new(id: i32, x: i32, y: i32, width: i32, height: i32) -> Claim {
        Claim {
            id,
            x,
            y,
            width,
            height,
        }
    }

    /// ### Get the corners of a claim
    ///
    /// #### Example
    /// ```
    /// let claim = day_3::Claim::new( 1, 1, 3, 4, 4 );
    /// let corners = claim.corners();
    /// assert_eq!( corners.len(), 4 );
    /// assert!( corners.contains( &day_3::Point::new( 1, 3 ) ) );
    /// assert!( corners.contains( &day_3::Point::new( 1, 7 ) ) );
    /// assert!( corners.contains( &day_3::Point::new( 5, 7 ) ) );
    /// assert!( corners.contains( &day_3::Point::new( 5, 3 ) ) );
    /// ```
    pub fn corners(&self) -> Vec<Point> {
        vec![
            Point {
                x: self.x,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y + self.height,
            },
            Point {
                x: self.x + self.width,
                y: self.y,
            },
            Point {
                x: self.x + self.width,
                y: self.y + self.height,
            },
        ]
    }

    /// ### Iterator over all points contained by this claim
    ///
    /// #### Example
    /// ```
    /// let claim = day_3::Claim::new( 1, 1, 1, 1, 2 );
    /// let mut points = claim.points();
    /// assert_eq!( points.next().unwrap(), day_3::Point::new( 1, 1 ) );
    /// assert_eq!( points.next().unwrap(), day_3::Point::new( 1, 2 ) );
    /// assert_eq!( points.next(), None );
    ///
    /// let claim = day_3::Claim::new( 1, 1, 3, 4, 4 );
    /// let points:Vec<day_3::Point> = claim.points().collect();
    /// assert_eq!( points.len(), 16 );
    /// ```
    pub fn points(&self) -> Points {
        Points::from(self)
    }

    /// ### Check whether a point lies inside a claim
    ///
    /// #### Example
    /// ```
    /// let claim = day_3::Claim::new( 1, 1, 3, 4, 4 );
    /// assert!( claim.contains( &day_3::Point::new( 1, 3 ) ) );
    /// assert!( claim.contains( &day_3::Point::new( 4, 6 ) ) );
    /// assert!( claim.contains( &day_3::Point::new( 1, 6 ) ) );
    /// assert!( claim.contains( &day_3::Point::new( 4, 3 ) ) );
    /// assert!( claim.contains( &day_3::Point::new( 2, 4 ) ) );
    ///
    /// assert!( !claim.contains( &day_3::Point::new( 1, 2 ) ) );
    /// assert!( !claim.contains( &day_3::Point::new( 0, 3 ) ) );
    /// assert!( !claim.contains( &day_3::Point::new( 0, 2 ) ) );
    /// assert!( !claim.contains( &day_3::Point::new( 4, 2) ) );
    /// assert!( !claim.contains( &day_3::Point::new( 5, 3) ) );
    /// assert!( !claim.contains( &day_3::Point::new( 5, 2) ) );
    /// assert!( !claim.contains( &day_3::Point::new( 1, 7) ) );
    /// assert!( !claim.contains( &day_3::Point::new( 0, 6) ) );
    /// assert!( !claim.contains( &day_3::Point::new( 0, 7) ) );
    /// assert!( !claim.contains( &day_3::Point::new( 5, 7) ) );
    /// assert!( !claim.contains( &day_3::Point::new( 5, 6) ) );
    /// assert!( !claim.contains( &day_3::Point::new( 4, 7) ) );
    /// ```
    pub fn contains(&self, point: &Point) -> bool {
        self.x <= point.x
            && self.x + self.width > point.x
            && self.y <= point.y
            && self.y + self.height > point.y
    }

    /// ### Calculate the set of overlapping points
    ///
    /// #### Example
    /// ```
    /// let a = day_3::Claim::new( 1, 1, 3, 4, 4 );
    /// let b = day_3::Claim::new( 2, 3, 1, 4, 4 );
    /// let c = day_3::Claim::new( 3, 5, 5, 2, 2 );
    ///
    /// let ab = a.overlap( &b );
    /// let a_points: Vec<day_3::Point> = a.points().collect();
    /// println!( "a: {:?}", a_points );
    /// let b_points: Vec<day_3::Point> = b.points().collect();
    /// println!( "b: {:?}", b_points );
    /// println!( "ab: {:?}", ab );
    /// assert_eq!( ab.len(), 4 );
    ///
    ///
    /// let ac = a.overlap( &c );
    /// assert_eq!( ac.len(), 0 );
    /// let bc = b.overlap( &c );
    /// assert_eq!( bc.len(), 0 );
    /// ```
    pub fn overlap(&self, other: &Claim) -> HashSet<Point> {
        HashSet::from_iter(self.points().filter(|p| other.contains(p)))
    }
}

/// ## First part of the exercise
///
/// Find all patches that belong to two or more claims.
///
/// ### Example
/// ```
/// let input = vec![ "#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2" ];
/// assert_eq!( day_3::part_1( &input ), 4 );
/// ```
pub fn part_1(input: &Vec<&str>) -> usize {
    let claims: Vec<Claim> = input.iter().map(|line| Claim::parse(line)).collect();
    let pairs = claims.iter().enumerate().flat_map(|(index, claim)| {
        claims
            .iter()
            .skip(index + 1)
            .map(move |other| (claim, other))
    });

    let shared = pairs.fold(HashSet::new(), |shared, (a, b)| {
        let new_points: HashSet<Point> = a.overlap(b);
        // TODO: I don't know how to merge both sets.  Now we clone the points and add them to the
        // second set.  Simply adding them (without cloning them) doesn't work because they are still
        // owned by `new_points`.  As far as I can tell `.union` also doesn't solve the problem because
        // that would create a new HashSet and not modify the `shared` hashset, as is required by `pairs.fold`.
        let result: HashSet<Point> = new_points.iter().fold(shared, move |mut shared, new| {
            shared.insert(new.clone());
            shared
        });
        result
    });

    shared.len()
}

/// ## Second part of the exercise
///
/// Find a claim that doesn't overlap with any other.
///
/// ### Example
/// ```
/// let input = vec![ "#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2" ];
/// assert_eq!( day_3::part_2( &input ), 3 );
/// ```
pub fn part_2(input: &Vec<&str>) -> i32 {
    let claims: Vec<Claim> = input.iter().map(|line| Claim::parse(line)).collect();
    let result = claims.iter().find(|claim| {
        let mut others = claims.iter().filter(|other| other.id != claim.id);
        others.all(|other| other.overlap(claim).is_empty())
    });
    result.expect("No intact claim found").id
}
