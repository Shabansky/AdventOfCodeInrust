use crate::{Signal, SignalState};

use super::WireId;

mod gate_not;
mod wire;
use gate_not::GateNot;
use std::collections::HashMap;
use wire::{Wire, WireError};

enum CircuitComponent {
    SignalWireSource {
        id: WireId,
        signal: Signal,
    },
    Wire {
        id: WireId,
    },
    NotGate {
        input: WireId,
        output: WireId,
    },
    ShiftGate {
        input: WireId,
        shift: u32,
        output: WireId,
    },
    OrGate {
        input_1: WireId,
        input_2: WireId,
        output: WireId,
    },
    AndGate {
        input_1: WireId,
        input_2: WireId,
        output: WireId,
    },
}

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

    fn get_wire_or_create(&mut self, id: &WireId) -> &Wire {
        //TODO: id.clone()
        self.wire_register.entry(id.clone()).or_insert(Wire::new())
    }

    fn get_wire(&self, id: &WireId) -> Option<&Wire> {
        self.wire_register.get(id)
    }

    fn get_wire_mut(&mut self, id: &WireId) -> Option<&mut Wire> {
        self.wire_register.get_mut(id)
    }

    fn build_component(&mut self, component: CircuitComponent) {
        match component {
            CircuitComponent::SignalWireSource { id, signal } => {
                self.build_source(signal, id);
            }
            _ => {}
        }
    }
    //Corresponds to lines of the type 123 -> a
    fn build_source(&mut self, signal: Signal, id: WireId) -> &Wire {
        self.build_wire(&id);
        let wire = self.get_wire_mut(&id).unwrap();
        wire.set_signal(signal);
        wire
    }

    fn build_wire(&mut self, id: &WireId) {
        //TODO: Consider removing get_wire_or_create. Either read or create. No need for upserts.
        self.get_wire_or_create(&id);
    }

    fn build_wires(&mut self, wire_ids: Vec<&WireId>) {
        for id in wire_ids {
            self.build_wire(id);
        }
    }

    fn build_not_gate(&mut self, input_id: &WireId, output_id: &WireId) {
        self.build_wires(vec![input_id, output_id]);

        let gate = GateNot {
            input: self.get_wire(input_id).unwrap(),
            output: self.get_wire(output_id).unwrap(),
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
        let _ = builder.get_wire_or_create(&"ab".to_string());
        assert_eq!(1, builder.wire_register.len());
    }

    #[test]
    fn build_a_source() {
        let mut builder = CircuitBuilder::new();

        builder.build_component(CircuitComponent::SignalWireSource {
            id: String::from("ab"),
            signal: 1000,
        });

        let wire = builder.get_wire_or_create(&String::from("ab"));
        assert_eq!(SignalState::Signal(1000), *wire.get_signal());
    }
}
