use crate::{Signal, SignalState};

use super::WireId;

mod wire;

use std::collections::HashMap;
use wire::{Wire, WireError};

struct CircuitBuilder {
    wire_register: HashMap<String, Wire>,
}

struct DummyCircuit {
    input: WireId,
    output: WireId,
}

impl CircuitBuilder {
    fn new() -> Self {
        Self {
            wire_register: HashMap::new(),
        }
    }

    fn get_wire(&mut self, id: &WireId) -> &mut Wire {
        //TODO: id.clone()
        self.wire_register.entry(id.clone()).or_insert(Wire::new())
    }
    fn get_wire_simple(&self, id: &WireId) -> Option<&Wire> {
        self.wire_register.get(id)
    }

    //Corresponds to lines of the type 123 -> a
    fn build_source(&mut self, signal: Signal, id: WireId) -> Result<&mut Wire, WireError> {
        let wire = self.get_wire(&id);
        wire.set_signal(signal)?;
        Ok(wire)
    }

    fn build_multiwire(&mut self, input_id: &WireId, output_id: &WireId) {
        let _ = self.get_wire(input_id);
        let _ = self.get_wire(output_id);
        let gate = DummyCircuit {
            input: input_id.clone(),
            output: output_id.clone(),
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_wire_updates_register() {
        let mut builder = CircuitBuilder::new();

        assert_eq!(0, builder.wire_register.len());
        let _ = builder.get_wire(&"ab".to_string());
        assert_eq!(1, builder.wire_register.len());
    }

    #[test]
    fn multiple_get_wire_does_not_create_different_wires() {
        let mut builder = CircuitBuilder::new();

        let wire = builder.get_wire(&"ab".to_string());
        wire.set_signal(20);
        assert_eq!(
            &SignalState::Signal(20),
            builder.get_wire(&"ab".to_string()).get_signal()
        );
    }
}
