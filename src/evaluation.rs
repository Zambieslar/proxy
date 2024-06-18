use crate::definitions::SubState::*;
use crate::{definitions::*, statemachine::*, traits::*};

impl StateMachine {
    pub fn evaluate(&mut self, request: &mut Request, data: Vec<u8>) {
        match data[self.offset()] {
            b':' | b' ' => {
                if matches!(self.substate(), METHOD) {
                    loop {
                        self.offset += 1;
                        match data[self.offset()] {
                            b'\n' => {
                                self.next_substate();
                                break;
                            }
                            _ => {
                                self.buf.push(data[self.offset()] as char);
                                continue;
                            }
                        }
                    }
                }
            }
            b'\n' => {
                request.method = self.buf();
                self.next_substate();
            }
            _ => {}
        }
    }
}
