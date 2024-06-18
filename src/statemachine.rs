use std::{iter, prelude::*};

use crate::{
    definitions::SubState::*,
    definitions::*,
    evaluation::{self, *},
    traits::*,
};

impl StateMachine {
    pub fn run(&mut self, data: Vec<u8>) -> Request {
        let mut request = Request::default();
        loop {
            self.offset += 1;
            if self.offset() == data.len() {
                self.next_state();
            }
            match self.state.0 {
                State::Start => self.next_state(),
                State::Scan => match self.state.1 {
                    SubState::METHOD => {
                        self.evaluate(&mut request, data.clone());
                        request.method = self.buf();
                        self.buf.clear();
                    }
                    SubState::VERSION => {
                        self.evaluate(&mut request, data.clone());
                        request.version = self.buf();
                        self.buf.clear();
                    }
                    SubState::HOST => {
                        self.evaluate(&mut request, data.clone());
                        request.host = self.buf();
                        self.buf.clear();
                    }
                    SubState::PARAMS => loop {
                        let mut param = Param::default();
                        match data[self.offset()] {
                            b':' | b' ' => {
                                param.key = self.buf();
                                self.buf.clear();
                                match data[self.offset()] {
                                    b'\n' => {
                                        if self.offset() == data.len() {
                                            self.next_state();
                                            break;
                                        }
                                        param.value = self.buf();
                                        self.buf.clear();
                                    }
                                    _ => {
                                        self.buf.push(data[self.offset()] as char);
                                        self.offset += 1;
                                    }
                                }
                            }
                            _ => {
                                self.buf.push(data[self.offset()] as char);
                            }
                        }
                    },
                },
                State::Complete => return request,
            }
        }
    }
}
