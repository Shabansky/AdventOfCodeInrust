type Signal = u16;

struct Node {
    signal: Signal,
}

impl Node {
    fn new(signal: Signal) -> Self {
        Self { signal }
    }
}

type NodePair = (Node, Node);
type WireId = String;

struct Wire {
    signal: Signal,
    input: Node,
    output: Vec<Node>,
}

impl Wire {
    fn new() -> Self {
        Self {
            signal: 0,
            input: Node::new(0),
            output: vec![],
        }
    }
}

#[allow(dead_code)]
struct GateShift {
    input: Node,
    output: Node,
}

#[allow(dead_code)]
struct GateAnd {
    input: NodePair,
    signal: Signal,
    output: Node,
}

#[allow(dead_code)]
struct GateOr {
    input: NodePair,
    signal: Signal,
    output: Node,
}

#[allow(dead_code)]
struct GateNot {
    input: NodePair,
    signal: Signal,
    output: Node,
}

mod circuit {
    use super::Wire;
    use super::WireId;
    use std::collections::HashMap;

    struct CircuitBuilder {
        wire_register: HashMap<String, Wire>,
    }

    impl CircuitBuilder {
        fn new() -> Self {
            Self {
                wire_register: HashMap::new(),
            }
        }

        fn get_wire(&mut self, id: WireId) -> &mut Wire {
            self.wire_register.entry(id).or_insert(Wire::new())
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn add_wire_updates_register() {
            let mut builder = CircuitBuilder::new();

            assert_eq!(0, builder.wire_register.len());
            let _ = builder.get_wire("ab".to_string());
            assert_eq!(1, builder.wire_register.len());
        }

        #[test]
        fn multiple_get_wire_does_not_create_different_wires() {
            let mut builder = CircuitBuilder::new();

            let wire = builder.get_wire("ab".to_string());
            wire.signal = 20;
            assert_eq!(20, builder.get_wire("ab".to_string()).signal);
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_wire_has_default_property_values() {
        let wire = Wire::new();

        assert_eq!(0, wire.signal);
        assert_eq!(0, wire.input.signal);
        assert_eq!(0, wire.output.len());
    }
}
