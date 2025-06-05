use crate::{Signal, SignalState};

use super::WireId;

use crate::wire::{Wire, WireError};
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

    //Corresponds to lines of the type 123 -> a
    fn build_source(&mut self, signal: Signal, id: WireId) -> Result<&mut Wire, WireError> {
        let wire = self.get_wire(id);
        wire.set_signal(signal)?;
        Ok(wire)
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
        wire.set_signal(20);
        assert_eq!(
            &SignalState::Signal(20),
            builder.get_wire("ab".to_string()).get_signal()
        );
    }
}
