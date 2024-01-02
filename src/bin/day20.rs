use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

// The general idea is to have a HashMap from name to module and a queue for each module with
// incoming pulses. There's also a queue for when to handle which module.
// TODO: Figure out what happens if a conjunction module gets multiple pulses. Do we have to tag
// the queue with some kind of "step" indicator to differentiate between the current step and the
// next step?

#[derive(Debug)]
enum ModuleType {
    Conjunction,
    FlipFlop,
    Broadcaster,
}

#[derive(Debug)]
struct Module {
    name: String,
    kind: ModuleType,
    destination: Vec<String>,
}

#[derive(Debug)]
struct Puzzle {
    configuration: Vec<Module>,
}

impl Puzzle {
    fn get_module(&self, name: &str) -> Option<&Module> {
        self.configuration.iter().find(|c| c.name == name)
    }

    /// Gets the number of inputs for each module
    fn input_counts(&self) -> HashMap<String, usize> {
        let mut result = HashMap::new();

        for module in self.configuration.iter() {
            for source in self.configuration.iter() {
                if source.destination.contains(&module.name) {
                    result
                        .entry(module.name.clone())
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        }

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Amplitude {
    High,
    Low,
}

#[derive(Debug)]
struct Pulse {
    source: String,
    destination: String,
    amplitude: Amplitude,
}

fn parse(input: &str) -> Puzzle {
    let mut configuration = vec![];
    for line in input.lines() {
        let (m, dest) = line.split_once(" -> ").unwrap();
        let destination = dest.split(", ").map(ToOwned::to_owned).collect::<Vec<_>>();

        let (name, kind) = if m.starts_with("%") {
            (m[1..].to_owned(), ModuleType::FlipFlop)
        } else if m.starts_with("&") {
            (m[1..].to_owned(), ModuleType::Conjunction)
        } else {
            (m.to_owned(), ModuleType::Broadcaster)
        };

        configuration.push(Module {
            name,
            kind,
            destination,
        });
    }

    Puzzle { configuration }
}

fn part_1(puzzle: &Puzzle) -> u32 {
    let mut queue: VecDeque<Pulse> = VecDeque::new();
    let mut conjunction_state: HashMap<String, HashMap<String, Amplitude>> = HashMap::new();
    let conjunction_inputs: HashMap<String, usize> = puzzle.input_counts();
    let mut flip_flop_state: HashMap<String, bool> = HashMap::new();
    let mut sent_low_pulses = 0;
    let mut sent_high_pulses = 0;

    for i in 1..=1000 {
        queue.push_back(Pulse {
            destination: "broadcaster".to_owned(),
            amplitude: Amplitude::Low,
            source: "button".to_string(),
        });
        while let Some(pulse) = queue.pop_front() {
            if pulse.destination == "zr" && pulse.amplitude == Amplitude::High {
                println!("zr reached from {} at {}", pulse.source, i);
            }
            if pulse.amplitude == Amplitude::High {
                sent_high_pulses += 1;
            } else {
                sent_low_pulses += 1;
            }
            let module = puzzle.get_module(&pulse.destination);
            if module.is_none() {
                continue;
            }
            let module = module.unwrap();
            match module.kind {
                ModuleType::Conjunction => {
                    let entry = conjunction_state
                        .entry(module.name.clone())
                        .or_insert(HashMap::new());
                    entry.insert(pulse.source, pulse.amplitude);

                    if entry.len() == conjunction_inputs[&module.name]
                        && entry.iter().all(|(_, a)| *a == Amplitude::High)
                    {
                        module.destination.iter().for_each(|d| {
                            queue.push_back(Pulse {
                                destination: d.to_owned(),
                                amplitude: Amplitude::Low,
                                source: module.name.clone(),
                            });
                        });
                    } else {
                        module.destination.iter().for_each(|d| {
                            queue.push_back(Pulse {
                                destination: d.to_owned(),
                                amplitude: Amplitude::High,
                                source: module.name.clone(),
                            });
                        });
                    }
                }
                ModuleType::FlipFlop => {
                    if pulse.amplitude == Amplitude::Low {
                        let on = *flip_flop_state.get(&module.name).unwrap_or(&false);
                        if on {
                            flip_flop_state.insert(module.name.clone(), !on);

                            module.destination.iter().for_each(|d| {
                                queue.push_back(Pulse {
                                    destination: d.to_owned(),
                                    amplitude: Amplitude::Low,
                                    source: module.name.clone(),
                                });
                            });
                        } else {
                            flip_flop_state.insert(module.name.clone(), !on);

                            module.destination.iter().for_each(|d| {
                                queue.push_back(Pulse {
                                    destination: d.to_owned(),
                                    amplitude: Amplitude::High,
                                    source: module.name.clone(),
                                });
                            });
                        }
                    }
                }
                ModuleType::Broadcaster => {
                    module.destination.iter().for_each(|d| {
                        queue.push_back(Pulse {
                            destination: d.to_owned(),
                            amplitude: pulse.amplitude,
                            source: module.name.clone(),
                        });
                    });
                }
            }
        }
    }
    sent_low_pulses * sent_high_pulses
}

fn main() {
    let input = read_to_string("inputs/day20.txt").expect("file not found");

    let puzzle = parse(&input);

    println!("Part 1: {}", part_1(&puzzle));

    // rx is only reachable by the conjunction zr.
    // For rx to receive a high pulse all inputs to zr must be a high pulse as well. If we look at all inputs to zr (gc, xf, cm, sz) and log the amount of clicks it takes until each of them emits a high pulse, we can take the least common multiple of all clicks and get the least amount of clicks until all inputs receive a high pulse and therefore rx receives a high pulse as well.
    // For my input that's:
    // C(gc) = 3853
    // C(xf) = 4073
    // C(cm) = 4091
    // C(sz) = 4093
    // lcm(C(gc), C(xf), C(cm), C(sz)) = 262775362119547
    println!("Part 2: {}", 262775362119547);
}
