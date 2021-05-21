use std::collections::{HashSet, VecDeque};

mod device;
mod element;
mod floor;
mod gadget;
mod state;

pub(crate) use {device::Device, element::Element, floor::Floor, gadget::Gadget, state::State};

pub fn breadth_first_search(initial: State) -> Result<State, Error> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_front(initial);

    while let Some(state) = queue.pop_front() {
        if visited.contains(&state) {
            continue;
        }

        if state.is_goal() {
            return Ok(state);
        }

        for child in state.children(&visited) {
            queue.push_back(child);
        }

        visited.insert(state);
    }

    Err(Error::NoSolution)
}

pub fn input() -> State {
    use Element::*;

    let mut s = State::default();

    s.add_device(0, Device::generator(Promethium));
    s.add_device(0, Device::microchip(Promethium));
    s.add_device(1, Device::generator(Cobalt));
    s.add_device(1, Device::generator(Curium));
    s.add_device(1, Device::generator(Ruthenium));
    s.add_device(1, Device::generator(Plutonium));
    s.add_device(2, Device::microchip(Cobalt));
    s.add_device(2, Device::microchip(Curium));
    s.add_device(2, Device::microchip(Ruthenium));
    s.add_device(2, Device::microchip(Plutonium));

    s
}

pub fn input_part2() -> State {
    use Element::*;

    let mut s = input();

    // here Hydrogen stands in for Elerium
    s.add_device(0, Device::generator(Hydrogen));
    s.add_device(0, Device::microchip(Hydrogen));
    // here Lithium stands in for Dilithium
    s.add_device(0, Device::generator(Lithium));
    s.add_device(0, Device::microchip(Lithium));

    s
}

pub fn part1() -> Result<(), Error> {
    let state = input();
    let steps = breadth_first_search(state)?.steps();
    println!("part1 solution in {} steps", steps);
    Ok(())
}

pub fn part2() -> Result<(), Error> {
    let state = input_part2();
    let steps = breadth_first_search(state)?.steps();
    println!("part2 solution in {} steps", steps);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("could not determine a solution")]
    NoSolution,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> State {
        use Element::*;

        let mut s = State::default();
        s.add_device(0, Device::microchip(Hydrogen));
        s.add_device(0, Device::microchip(Lithium));
        s.add_device(1, Device::generator(Hydrogen));
        s.add_device(2, Device::generator(Lithium));

        s
    }

    fn show_path_to(state: &State) {
        if let Some(parent) = state.parent() {
            show_path_to(parent);
        }

        println!("{}:", state.steps());
        println!("{}", state);
    }

    #[test]
    fn test_example() {
        let goal = breadth_first_search(example()).unwrap();
        show_path_to(&goal);
        assert_eq!(goal.steps(), 11);
    }
}
