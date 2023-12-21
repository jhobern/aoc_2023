use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Signal {
    destination: usize,
    source: usize,
    signal_type: SignalType,
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum SignalType {
    High,
    Low,
}

impl SignalType {
    fn toggled(&self) -> SignalType {
        match self {
            SignalType::High => SignalType::Low,
            SignalType::Low => SignalType::High,
        }
    }
}

struct Components {
    low_pulses: usize,
    high_pulses: usize,
    components: HashMap<usize, Box<dyn Receiver>>,
    signal_queue: Rc<RefCell<SignalQueue>>,
    rx_count: usize,
    rx_id: usize,
}

impl Components {
    fn add_component(&mut self, name: usize, component: Box<dyn Receiver>) {
        self.components.insert(name, component);
    }

    fn tick(&mut self) -> bool {
        let mut queue = self.signal_queue.borrow_mut();
        let signal = queue.next();
        drop(queue);
        if let Some(signal) = signal {
            if signal.signal_type == SignalType::Low && signal.destination == self.rx_id {
                self.rx_count += 1;
            }
            match signal.signal_type {
                SignalType::High => self.high_pulses += 1,
                SignalType::Low => self.low_pulses += 1,
            }
            if let Some(destination) = self.components.get_mut(&signal.destination) {
                (*destination).receive(signal);
            }
            true
        } else {
            false
        }
    }

    fn add_input(&mut self, component: &usize, input: usize) {
        if let Some(component) = self.components.get_mut(component) {
            (*component).add_input(input);
        }
    }
}

#[derive(Debug)]
struct SignalQueue {
    signals: Vec<Signal>,
}

impl SignalQueue {
    fn push_signal(&mut self, signal: Signal) {
        self.signals.push(signal);
    }

    fn next(&mut self) -> Option<Signal> {
        if self.signals.is_empty() {
            return None;
        }
        let signal = self.signals.remove(0);
        Some(signal)
    }
}

trait Receiver: std::fmt::Debug {
    fn receive(&mut self, signal: Signal);

    fn add_input(&mut self, input_name: usize);
}

#[derive(Debug)]
struct BroadCaster {
    name: usize,
    destinations: Vec<usize>,
    signal_queue: Rc<RefCell<SignalQueue>>,
}

impl Receiver for BroadCaster {
    fn receive(&mut self, signal: Signal) {
        let mut signal = signal;
        signal.source = self.name;
        let mut queue = self.signal_queue.borrow_mut();
        for &destination in &self.destinations {
            queue.push_signal(Signal {
                destination,
                source: self.name,
                signal_type: signal.signal_type.clone(),
            });
        }
    }

    fn add_input(&mut self, _input_name: usize) {}
}

#[derive(Debug)]
struct FlipFlop {
    name: usize,
    state: SignalType,
    destinations: Vec<usize>,
    signal_queue: Rc<RefCell<SignalQueue>>,
}

impl Receiver for FlipFlop {
    fn receive(&mut self, signal: Signal) {
        if signal.signal_type == SignalType::High {
            return;
        }

        self.state = self.state.toggled();

        let mut queue = self.signal_queue.borrow_mut();
        for &destination in &self.destinations {
            queue.push_signal(Signal {
                destination,
                source: self.name,
                signal_type: self.state.clone(),
            });
        }
    }

    fn add_input(&mut self, _input_name: usize) {}
}

struct Conjuction {
    name: usize,
    destinations: Vec<usize>,
    inputs: HashMap<usize, SignalType>,
    signal_queue: Rc<RefCell<SignalQueue>>,
}

impl std::fmt::Debug for Conjuction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for input in self.inputs.values() {
            if input == &SignalType::High {
                output.push('1');
            } else {
                output.push('0');
            }
        }
        f.write_str(&output)
    }
}

impl Receiver for Conjuction {
    fn receive(&mut self, signal: Signal) {
        *self.inputs.get_mut(&signal.source).unwrap() = signal.signal_type.clone();

        let signal_type = if self.inputs.values().all(|value| *value == SignalType::High) {
            SignalType::Low
        } else {
            SignalType::High
        };
        let mut queue = self.signal_queue.borrow_mut();
        for &destination in &self.destinations {
            queue.push_signal(Signal {
                destination,
                source: self.name,
                signal_type: signal_type.clone(),
            });
        }
    }

    fn add_input(&mut self, input_name: usize) {
        self.inputs.insert(input_name, SignalType::Low);
    }
}

fn parse_input(s: &str) -> (Components, Rc<RefCell<SignalQueue>>, HashMap<String, usize>) {
    let signal_queue = SignalQueue {
        signals: Vec::new(),
    };
    let signal_reference = Rc::new(RefCell::new(signal_queue));

    let mut inputs = HashMap::new();
    let mut names = HashMap::new();
    let mut curr_name = 0;

    for line in s.lines() {
        let (mut component, outputs) = line.split_once(" -> ").unwrap();
        let mut all_names = outputs
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        if component != "broadcaster" {
            component = &component[1..];
        }
        all_names.push(component.to_string());
        for component in all_names {
            if let std::collections::hash_map::Entry::Vacant(e) = names.entry(component) {
                e.insert(curr_name);
                curr_name += 1_usize;
            }
        }
    }

    let rx_id = *names.get("rx").unwrap();

    let mut components = Components {
        components: HashMap::new(),
        signal_queue: signal_reference.clone(),
        low_pulses: 0,
        high_pulses: 0,
        rx_count: 0,
        rx_id,
    };

    for line in s.lines() {
        let (component, outputs) = line.split_once(" -> ").unwrap();
        let outputs = outputs
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let name = if component == "broadcaster" {
            component
        } else {
            &component[1..]
        };

        let name = *names.get(name).unwrap();

        let output_ids = outputs
            .iter()
            .map(|output| *names.get(output.as_str()).unwrap())
            .collect::<Vec<_>>();

        if component == "broadcaster" {
            let component = BroadCaster {
                name,
                destinations: output_ids.clone(),
                signal_queue: signal_reference.clone(),
            };

            let component = Box::new(component);
            components.add_component(name, component);
            inputs.insert(name, output_ids);
        } else {
            let component_type = component.chars().next().unwrap();
            if component_type == '%' {
                let component = FlipFlop {
                    name,
                    state: SignalType::Low,
                    destinations: output_ids.clone(),
                    signal_queue: signal_reference.clone(),
                };
                components.add_component(name, Box::new(component));
            } else if component_type == '&' {
                let component = Conjuction {
                    name,
                    signal_queue: signal_reference.clone(),
                    destinations: output_ids.clone(),
                    inputs: HashMap::new(),
                };
                components.add_component(name, Box::new(component));
            }
            inputs.insert(name, output_ids);
        }
    }

    for (source, output_list) in inputs {
        for output in output_list {
            components.add_input(&output, source);
        }
    }

    (components, signal_reference, names)
}

fn process(s: &str) -> usize {
    //Technically working.... If you let it run forever
    let (mut components, queue, names) = parse_input(s);

    let broadcast_id = *names.get("broadcaster").unwrap();

    for i in 0.. {
        components.rx_count = 0;
        queue.borrow_mut().push_signal(Signal {
            destination: broadcast_id,
            source: 0,
            signal_type: SignalType::Low,
        });
        while components.tick() {}
        if components.rx_count == 1 {
            return i;
        }
    }

    todo!()
}
