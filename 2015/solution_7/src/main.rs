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

struct Wire {
    id: String,
    signal: Signal,
    input: Node,
    output: Vec<Node>,
}

impl Wire {
    fn new(id: String) -> Self {
        Self {
            id,
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
    use std::collections::HashMap;

    #[derive(PartialEq, Debug)]
    enum RegisterError {
        WireAlreadyExists,
    }
    struct CircuitBuilder {
        wire_register: HashMap<String, Wire>,
    }

    impl CircuitBuilder {
        fn new() -> Self {
            Self {
                wire_register: HashMap::new(),
            }
        }

        fn add_wire_to_register(&mut self, wire: Wire) -> Result<(), RegisterError> {
            let wire_id = wire.id.clone();
            if let Some(_) = self.wire_register.get(&wire_id) {
                Err(RegisterError::WireAlreadyExists)
            } else {
                self.wire_register.insert(wire_id, wire);
                Ok(())
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn add_wire_updates_register() {
            let mut builder = CircuitBuilder::new();

            assert_eq!(0, builder.wire_register.len());
            let _ = builder.add_wire_to_register(Wire::new("ab".to_string()));
            assert_eq!(1, builder.wire_register.len());
        }

        #[test]
        fn duplicate_wire_register_produces_error() {
            let mut builder = CircuitBuilder::new();

            let _ = builder.add_wire_to_register(Wire::new("ab".to_string()));

            let error = builder.add_wire_to_register(Wire::new("ab".to_string()));
            assert_eq!(RegisterError::WireAlreadyExists, error.unwrap_err());
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
        let wire = Wire::new("id".to_string());

        assert_eq!("id".to_string(), wire.id);
        assert_eq!(0, wire.signal);
        assert_eq!(0, wire.input.signal);
        assert_eq!(0, wire.output.len());
    }
}
