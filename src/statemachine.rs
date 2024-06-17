use std::{iter, prelude::*, rc::Rc};

use crate::{
    definitions::*,
    evaluation::{self, *},
    traits::*,
};

impl StateMachine {
    pub fn run(&mut self, data: &[u8]) -> Request {
        let mut request = Request::default();
        let iter = Rc::new(data);
        for byte in iter.into_iter() {
            match self.state.0 {
                State::Start => self.next_state(),
                State::Scan => match self.state.1 {
                    SubState::METHOD => self.evaluate(*byte, &mut request),
                    SubState::VERSION => match byte {
                        b':' | b' ' => self.next_substate(),
                        b'\n' => self.next_substate(),
                        _ => {
                            request.version = self.buf();
                        }
                    },
                    SubState::HOST => match byte {
                        b':' | b' ' => self.next_substate(),
                        b'\n' => self.next_substate(),
                        _ => {
                            request.host = self.buf();
                        }
                    },
                    SubState::PARAMS => match byte {
                        b':' | b' ' => self.next_substate(),
                        b'\n' => self.next_substate(),
                        _ => {
                            request.method = self.buf();
                        }
                    },
                },
                State::Complete => break,
            }
        }
        request
    }
}
