use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Signal {
    destination: String,
    source: String,
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
    components: HashMap<String, Box<dyn Receiver>>,
    signal_queue: Rc<RefCell<SignalQueue>>,
}

impl Components {
    fn add_component(&mut self, name: String, component: Box<dyn Receiver>) {
        self.components.insert(name, component);
    }

    fn tick(&mut self) -> bool {
        let mut queue = self.signal_queue.borrow_mut();
        let signal = queue.next();
        drop(queue);
        if let Some(signal) = signal {
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

    fn add_input(&mut self, component: &str, input: String) {
        if let Some(component) = self.components.get_mut(component) {
            (*component).add_input(input);
        }
    }
}

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

    fn broadcast(&mut self, mut signal: Signal) {
        signal.destination = "broadcaster".to_string();
        self.push_signal(signal);
    }
}

trait Receiver {
    fn receive(&mut self, signal: Signal);

    fn add_input(&mut self, input_name: String);
}

struct BroadCaster {
    name: String,
    destinations: Vec<String>,
    signal_queue: Rc<RefCell<SignalQueue>>,
}

impl Receiver for BroadCaster {
    fn receive(&mut self, signal: Signal) {
        let mut signal = signal;
        signal.source = self.name.clone();
        let mut queue = self.signal_queue.borrow_mut();
        for destination in &self.destinations {
            queue.push_signal(Signal {
                destination: destination.clone(),
                source: self.name.clone(),
                signal_type: signal.signal_type.clone(),
            });
        }
    }

    fn add_input(&mut self, _input_name: String) {}
}

struct FlipFlop {
    name: String,
    state: SignalType,
    destinations: Vec<String>,
    signal_queue: Rc<RefCell<SignalQueue>>,
}

impl Receiver for FlipFlop {
    fn receive(&mut self, signal: Signal) {
        if signal.signal_type == SignalType::High {
            return;
        }

        self.state = self.state.toggled();

        let mut queue = self.signal_queue.borrow_mut();
        for destination in &self.destinations {
            queue.push_signal(Signal {
                destination: destination.clone(),
                source: self.name.clone(),
                signal_type: self.state.clone(),
            });
        }
    }

    fn add_input(&mut self, _input_name: String) {}
}

struct Conjuction {
    name: String,
    destinations: Vec<String>,
    inputs: HashMap<String, SignalType>,
    signal_queue: Rc<RefCell<SignalQueue>>,
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
        for destination in &self.destinations {
            queue.push_signal(Signal {
                destination: destination.clone(),
                source: self.name.clone(),
                signal_type: signal_type.clone(),
            });
        }
    }

    fn add_input(&mut self, input_name: String) {
        self.inputs.insert(input_name, SignalType::Low);
    }
}

fn parse_input(s: &str) -> (Components, Rc<RefCell<SignalQueue>>) {
    let signal_queue = SignalQueue {
        signals: Vec::new(),
    };
    let signal_reference = Rc::new(RefCell::new(signal_queue));

    let mut components = Components {
        components: HashMap::new(),
        signal_queue: signal_reference.clone(),
        low_pulses: 0,
        high_pulses: 0,
    };

    let mut inputs = HashMap::new();

    for line in s.lines() {
        let (component, outputs) = line.split_once(" -> ").unwrap();
        let outputs = outputs
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        if component == "broadcaster" {
            let component = BroadCaster {
                name: "broadcaster".to_string(),
                destinations: outputs.clone(),
                signal_queue: signal_reference.clone(),
            };

            let component = Box::new(component);
            components.add_component("broadcaster".to_string(), component);
            inputs.insert("broadcaster".to_string(), outputs);
        } else {
            let component_type = component.chars().next().unwrap();
            let component_name = component[1..].to_string();
            let destination = outputs.clone();
            if component_type == '%' {
                let component = FlipFlop {
                    name: component_name.clone(),
                    state: SignalType::Low,
                    destinations: destination.clone(),
                    signal_queue: signal_reference.clone(),
                };
                components.add_component(component_name.clone(), Box::new(component));
            } else if component_type == '&' {
                let component = Conjuction {
                    name: component_name.clone(),
                    signal_queue: signal_reference.clone(),
                    destinations: destination.clone(),
                    inputs: HashMap::new(),
                };
                components.add_component(component_name.clone(), Box::new(component));
            }
            inputs.insert(component_name, outputs);
        }
    }

    for (source, output_list) in inputs {
        for output in output_list {
            components.add_input(&output, source.clone());
        }
    }

    (components, signal_reference)
}

fn process(s: &str) -> usize {
    let (mut components, queue) = parse_input(s);

    for _ in 0..1000000 {
        queue.borrow_mut().broadcast(Signal {
            destination: "broadcaster".to_string(),
            source: "root".to_string(),
            signal_type: SignalType::Low,
        });
        while components.tick() {}
    }

    components.high_pulses * components.low_pulses
}
