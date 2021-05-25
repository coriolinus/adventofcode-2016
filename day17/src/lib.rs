use aoclib::{
    geometry::{Direction, Point},
    parse,
};
use crypto::{digest::Digest, md5::Md5};

use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
    path::Path,
    rc::Rc,
};

type Map = aoclib::geometry::Map<()>;

lazy_static::lazy_static! {
    static ref MAP: Map = Map::new(4, 4);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DoorStatus {
    Open,
    Closed,
}

impl Default for DoorStatus {
    fn default() -> Self {
        DoorStatus::Closed
    }
}

impl DoorStatus {
    fn is_open(&self) -> bool {
        *self == DoorStatus::Open
    }
}

#[derive(Default, Debug)]
struct RoomStatus([DoorStatus; 4]);

impl RoomStatus {
    fn direction_idx(direction: Direction) -> usize {
        match direction {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }
}

impl Index<Direction> for RoomStatus {
    type Output = DoorStatus;

    fn index(&self, index: Direction) -> &Self::Output {
        &self.0[Self::direction_idx(index)]
    }
}

impl IndexMut<Direction> for RoomStatus {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        &mut self.0[Self::direction_idx(index)]
    }
}

fn make_path_str(path: &[Direction]) -> String {
    path.iter()
        .map(|direction| match direction {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        })
        .collect()
}

fn make_get_room_status(passcode: &str) -> impl Fn(&[Direction]) -> RoomStatus {
    let mut digest = Md5::new();
    digest.input_str(passcode);
    move |path| {
        let mut digest = digest; // copy it
        let path: String = make_path_str(path);
        digest.input_str(&path);
        let hash = digest.result_str();

        let mut status = RoomStatus::default();
        for idx in 0..4 {
            if (b'b'..=b'f').contains(&hash.as_bytes()[idx]) {
                status.0[idx] = DoorStatus::Open;
            }
        }
        status
    }
}

/// A Transition has a reference to the previous state and the direction moved
/// from that state to get to the current state.
struct Transition {
    parent: Rc<State>,
    direction: Direction,
}

/// A State knows where it is and how it got there.
struct State {
    position: Point,
    parent: Option<Transition>,
}

impl State {
    fn new(position: Point) -> Self {
        State {
            position,
            parent: None,
        }
    }

    // we _could_ implement this to return `Box<dyn Iterator<Item=Direction>>`:
    //
    // `transition.parent.path_to().chain(iter::once(transition.direction))`
    //
    // Haven't benchmarked it, but I bet that using a vector requires less
    // overall allocation / space.
    fn path_to(&self) -> Vec<Direction> {
        match self.parent {
            Some(ref transition) => {
                let mut path = transition.parent.path_to();
                path.push(transition.direction);
                path
            }
            None => Vec::new(),
        }
    }

    fn children(
        self,
        get_room_status: impl Fn(&[Direction]) -> RoomStatus,
    ) -> impl Iterator<Item = State> {
        let parent = Rc::new(self);
        let room_status = get_room_status(&parent.path_to());

        Direction::iter()
            .filter(move |direction| room_status[*direction].is_open())
            .filter_map(move |direction| {
                let parent = parent.clone();
                let position = parent.position + direction;
                let child = State {
                    parent: Some(Transition { parent, direction }),
                    position,
                };
                MAP.in_bounds(position).then(move || child)
            })
    }
}

fn breadth_first_search(
    initial: Point,
    goal: Point,
    get_room_status: impl Fn(&[Direction]) -> RoomStatus,
) -> Option<String> {
    let mut queue = VecDeque::new();
    queue.push_front(State::new(initial));

    // no point keeping a "visited" hashmap because in this crazy room set,
    // "where we are" is almost less important than "how we got there". Since
    // we only ever append to the path, we never see the same state twice,
    // even if it happens that we're in the same room again.

    while let Some(state) = queue.pop_front() {
        if state.position == goal {
            return Some(make_path_str(&state.path_to()));
        }

        queue.extend(state.children(&get_room_status));
    }

    None
}

// be careful with the inputs; this is probably going to terminate eventually,
// but nothing in this code prevents an infinite loop
fn find_longest_path_to(
    initial: Point,
    goal: Point,
    get_room_status: impl Fn(&[Direction]) -> RoomStatus,
) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_front(State::new(initial));

    let mut max_path_len = None;

    while let Some(state) = queue.pop_front() {
        // if we find the goal, update the max found so far but do _not_ return
        // or add children.
        if state.position == goal {
            max_path_len = Some(state.path_to().len().max(max_path_len.unwrap_or_default()));
            continue;
        }

        queue.extend(state.children(&get_room_status));
    }

    max_path_len
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for passcode in parse::<String>(input)? {
        let get_room_status = make_get_room_status(&passcode);
        let path = breadth_first_search(MAP.top_left(), MAP.bottom_right(), get_room_status)
            .ok_or(Error::NotFound)?;
        println!("shortest path to goal: {}", path);
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for passcode in parse::<String>(input)? {
        let get_room_status = make_get_room_status(&passcode);
        let path_len = find_longest_path_to(MAP.top_left(), MAP.bottom_right(), get_room_status)
            .ok_or(Error::NotFound)?;
        println!("longest path to goal: {}", path_len);
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("could not find a path to the goal")]
    NotFound,
}
