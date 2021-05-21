mod device;
mod element;
mod floor;
mod gadget;
mod state;

pub(crate) use {device::Device, element::Element, floor::Floor, gadget::Gadget, state::State};

pub fn goalseek(initial: State) -> Option<u32> {
    // the subsequent code is way too complicated and should not be considered trustworthy
    unimplemented!()

    // let mut visited = HashSet::new();
    // let mut queue = VecDeque::new();
    // queue.push_front((0, initial));

    // let mut nsteps = 0;
    // let mut count = 0;

    // while let Some((steps, state)) = queue.pop_front() {
    //     if steps == nsteps {
    //         count += 1;
    //     } else {
    //         println!("visited {} states with {} steps", count, nsteps);
    //         nsteps = steps;
    //         count = 0;
    //     }

    //     if state.is_goal() {
    //         println!("{}", state);
    //         return Some(steps);
    //     }

    //     visited.insert(state.isomorph());

    //     for child in state.next(&visited) {
    //         queue.push_back((steps + 1, child));
    //     }
    // }

    // None
}

pub fn input() -> State {
    use Element::*;
    use Gadget::*;

    let mut s = State::default();

    s.add_device(0, Device::new(Promethium, Generator));
    s.add_device(0, Device::new(Promethium, Microchip));
    s.add_device(1, Device::new(Cobalt, Generator));
    s.add_device(1, Device::new(Curium, Generator));
    s.add_device(1, Device::new(Ruthenium, Generator));
    s.add_device(1, Device::new(Plutonium, Generator));
    s.add_device(2, Device::new(Cobalt, Microchip));
    s.add_device(2, Device::new(Curium, Microchip));
    s.add_device(2, Device::new(Ruthenium, Microchip));
    s.add_device(2, Device::new(Plutonium, Microchip));

    s
}

pub fn part1() -> Result<(), Error> {
    let state = input();
    let steps = goalseek(state).ok_or(Error::NoSolution)?;
    println!("found solution in {} steps", steps);
    Ok(())
}

pub fn part2() -> Result<(), Error> {
    unimplemented!()
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
        use Gadget::*;

        let mut s = State::default();
        s.add_device(0, Device::new(Hydrogen, Microchip));
        s.add_device(0, Device::new(Lithium, Microchip));
        s.add_device(1, Device::new(Hydrogen, Generator));
        s.add_device(2, Device::new(Lithium, Generator));

        s
    }

    #[test]
    fn test_simple_isomorph_equivalence() {
        use Element::*;
        use Gadget::*;

        let mut s1 = State::default();
        let mut s2 = State::default();

        assert_eq!(s1.isomorph(), s2.isomorph());

        s1.add_device(0, Device::new(Hydrogen, Microchip));
        s2.add_device(0, Device::new(Lithium, Microchip));
        assert_eq!(s1.isomorph(), s2.isomorph());

        s1.add_device(1, Device::new(Hydrogen, Generator));
        s2.add_device(1, Device::new(Lithium, Generator));
        assert_eq!(s1.isomorph(), s2.isomorph());
    }

    #[test]
    fn test_isomorph_equivalence() {
        let equiv = {
            use Element::*;
            use Gadget::*;

            let mut s = State::default();
            s.add_device(0, Device::new(Plutonium, Microchip));
            s.add_device(0, Device::new(Cobalt, Microchip));
            s.add_device(1, Device::new(Plutonium, Generator));
            s.add_device(2, Device::new(Cobalt, Generator));

            s
        };

        assert_eq!(example().isomorph(), equiv.isomorph());
    }

    #[test]
    fn test_example() {
        assert_eq!(Some(11), goalseek(example()));
    }
}
