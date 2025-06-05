use super::Node;
use super::{Signal, SignalState};
pub enum WireError {
    SignalAlreadySet,
}

pub struct Wire {
    signal: SignalState,
    input: Node,
    output: Vec<Node>,
}

impl Wire {
    pub fn new() -> Self {
        Self {
            signal: SignalState::NoSignal,
            input: Node::new(SignalState::NoSignal),
            output: vec![],
        }
    }

    pub fn set_signal(&mut self, signal: Signal) -> Result<(), WireError> {
        match self.signal {
            SignalState::Signal(_) => {
                return Err(WireError::SignalAlreadySet);
            }
            SignalState::NoSignal => self.signal = SignalState::Signal(signal),
        }
        Ok(())
    }

    pub fn get_signal(&self) -> &SignalState {
        &self.signal
    }

    pub fn get_input(&self) -> &Node {
        &self.input
    }

    pub fn get_output(&self) -> &Vec<Node> {
        &self.output
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_wire_has_default_property_values() {
        let wire = Wire::new();

        assert_eq!(&SignalState::NoSignal, wire.get_signal());
        assert_eq!(SignalState::NoSignal, wire.get_input().signal);
        assert_eq!(0, wire.get_output().len());
    }

    #[test]
    fn get_signal_as_reference() {
        let mut wire = Wire::new();
        wire.set_signal(42);
        assert_eq!(&SignalState::Signal(42), wire.get_signal());

        assert_eq!(&SignalState::Signal(42), wire.get_signal());
    }

    #[test]
    fn signal_cannot_be_overwritten() {
        let mut wire = Wire::new();
        assert!(wire.set_signal(42).is_ok());
        assert!(wire.set_signal(69).is_err());
    }
}
