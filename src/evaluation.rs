use crate::{definitions::*, statemachine::*, traits::*};

impl StateMachine {
    pub fn evaluate(&mut self, byte: u8, request: &mut Request) {
        match byte {
            b':' | b' ' => {
                request.method = self.buf();
            }
            b'\n' => self.next_substate(),
            _ => {}
        }
    }
}
