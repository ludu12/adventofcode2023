#![allow(warnings, unused)]

use std::collections::{HashMap, HashSet, VecDeque};
use std::panic::panic_any;
use std::pin::pin;
use itertools::Itertools;
use crate::utils::{lcm, print_grid};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Type {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Message {
    sender: String,
    receiver: String,
    pulse: Pulse,
}

#[derive(Debug, Clone, PartialEq)]
struct Module {
    class: Type,
    state: bool,
    next_modules: Vec<String>,
    memory: HashMap<String, Pulse>,
}

impl Module {
    fn new(class: Type, next_modules: Vec<String>) -> Self {
        let memory = HashMap::new();
        // if class == Type::Conjunction {
        //     for next_module in next_modules.iter() {
        //         memory.insert(next_module.to_owned(), Pulse::Low);
        //     }
        // }

        return Module {
            class,
            state: false,
            next_modules,
            memory,
        };
    }

    fn receive(&mut self, pulse: Pulse, sender: &str) -> Pulse {
        match self.class {
            Type::FlipFlop => {
                if pulse == Pulse::Low && self.state == false {
                    self.state = true;
                    Pulse::High
                } else if pulse == Pulse::Low && self.state == true {
                    self.state = false;
                    Pulse::Low
                } else {
                    Pulse::None
                }
            }
            Type::Conjunction => {
                self.memory.insert(sender.to_string(), pulse);
                if self.memory.values().all(|x| *x == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                }
            }
            Type::Broadcast => pulse,
        }
    }
}

fn parse_network(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();

    for line in input.lines() {
        let (module, next) = line.split_once(" -> ").unwrap();
        let next_modules = next.split(", ").map(|x| x.to_string()).collect::<Vec<_>>();
        let state = false;

        if module == "broadcaster" {
            modules.insert(module.to_string(), Module::new(Type::Broadcast, next_modules));
        } else if let Some(label) = module.strip_prefix('%') {
            modules.insert(label.to_string(), Module::new(Type::FlipFlop, next_modules));
        } else if let Some(label) = module.strip_prefix('&') {
            modules.insert(label.to_string(), Module::new(Type::Conjunction, next_modules));
        } else {
            println!("{module}");
            unreachable!();
        }
    }

    for (module, details) in modules.clone().iter() {
        for next_module_name in details.next_modules.iter() {
            if !modules.contains_key(next_module_name) {
                continue;
            }
            let next_module = modules.get_mut(next_module_name).unwrap();
            if next_module.class == Type::Conjunction {
                next_module.memory.insert(module.clone(), Pulse::Low);
            }
        }
    }

    return modules;
}

fn press_button(modules: &mut HashMap<String, Module>, stop: Option<&String>) -> (usize, usize, bool) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut unprocessed = VecDeque::new();

    unprocessed.push_back(Message { sender: "button".to_string(), receiver: "broadcaster".to_string(), pulse: Pulse::Low });
    while let Some(message) = unprocessed.pop_front() {
        match message.pulse {
            Pulse::Low => { low_pulses += 1 }
            Pulse::High => { high_pulses += 1 }
            Pulse::None => { continue; }
        };


        // For Part2, I want to find where we can stop before the rx output
        if let Some(s) = stop {
            if message.sender == *s && message.pulse == Pulse::High {
                return (0, 0, true);
            }
        }

        if !modules.contains_key(&message.receiver) { // Messages going to "output"
            continue;
        }
        let receiver = modules.get_mut(&message.receiver).unwrap();
        let result = receiver.receive(message.pulse, &message.sender);

        for module in receiver.next_modules.iter() {
            unprocessed.push_back(Message {
                sender: message.receiver.clone(),
                receiver: module.to_owned(),
                pulse: result,
            });
        }
    }
    (low_pulses, high_pulses, false)
}

fn press_button_until(modules: &mut HashMap<String, Module>, stop: &String) -> i64 {
    let mut counter = 0;
    loop {
        let (_, _, satisfied) = press_button(modules, Some(stop));
        counter += 1;
        if satisfied {
            return counter;
        }
    }
}


fn process(input: &str, part2: bool) -> i64 {
    let mut modules = parse_network(input);

    if part2 {
        let mut cycles = Vec::new();

        for (name, module) in modules.iter() {
            for next_module_name in module.next_modules.iter() {
                if next_module_name == "rx" {
                    let previous_stage: &Module = modules.get(name).unwrap();
                    for feeder_stage in previous_stage.memory.keys() {

                        // Get a new network
                        let mut new_modules: HashMap<String, Module> = parse_network(input);

                        // Want to find out how long it takes to get to this state with a pulse of High
                        // Then we can take the LCM of all these cycles
                        let count = press_button_until(&mut new_modules, feeder_stage);
                        cycles.push(count);
                    }
                    break;
                }
            }
        }
        return cycles.iter().fold(1, |l, val| lcm(l, *val));
    } else {
        let mut low = 0;
        let mut high = 0;
        for _ in 0..1000 {
            let (l, h, _) = press_button(&mut modules, None);
            low += l;
            high += h;
        }


        return (low * high) as i64;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(32000000, process(input, false));
    }

    #[test]
    fn part1b() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let mut network = parse_network(input);
        press_button(&mut network, None);
        assert_eq!(network["a"].state, false);
        assert_eq!(network["b"].state, false);
        assert_eq!(network["c"].state, false);
        assert_eq!(network["inv"].state, false);
    }

    #[test]
    fn part1c() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(11687500, process(input, false));
    }
}
