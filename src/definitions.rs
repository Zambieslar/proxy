use super::traits::*;
#[derive(Clone, Debug)]
pub enum State {
    Start,
    Scan,
    Complete,
}

#[derive(Clone, Debug)]
pub enum SubState {
    METHOD,
    VERSION,
    HOST,
    PARAMS,
}

pub enum Action {
    ADD,
    DELETE,
}

pub struct StateMachine {
    pub state: (State, SubState),
    pub offset: usize,
    pub mindex: usize,
    pub line: i32,
    pub buf: String,
}

#[derive(Default)]
pub struct Request {
    pub method: String,
    pub hsot: String,
    pub version: String,
    pub host: String,
    pub params: Vec<(String, String)>,
}

impl State {
    pub const STATES: [State; 3] = [Self::Start, Self::Scan, Self::Complete];

    pub const SUBSTATES: [SubState; 4] = [
        SubState::METHOD,
        SubState::VERSION,
        SubState::HOST,
        SubState::PARAMS,
    ];
}

impl Request {
    pub const METHODS: [&'static str; 9] = [
        "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
    ];
    pub const VERSION: [&'static str; 3] = ["HTTP/1.0", "HTTP/1.1", "HTTP/2.0"];
}

impl Machine for StateMachine {
    fn state(&self) -> State {
        self.state.0.clone()
    }

    fn substate(&self) -> SubState {
        self.state.1.clone()
    }

    fn offset(&self) -> usize {
        self.offset.clone()
    }

    fn mindex(&self) -> usize {
        self.mindex.clone()
    }

    fn buf(&self) -> String {
        self.buf.clone()
    }

    fn new() -> Self {
        Self {
            state: (State::Start, SubState::METHOD),
            offset: 0,
            mindex: 0,
            line: 0,
            buf: String::new(),
        }
    }

    fn next_state(&mut self) {
        match self.state() {
            state => {
                self.state.0 = State::STATES[state as usize + 1].clone();
            }
        }
    }

    fn next_substate(&mut self) {
        match self.substate() {
            state => {
                self.state.1 = State::SUBSTATES[state as usize + 1].clone();
            }
        }
    }

    fn reverse(&mut self) {
        match self.state() {
            state => {
                self.state.0 = State::STATES[state as usize - 1].clone();
            }
        }
    }
}
