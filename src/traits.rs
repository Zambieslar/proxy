use super::definitions::*;
use crate::traits::State;

pub trait Machine {
    fn state(&self) -> State;
    fn substate(&self) -> SubState;
    fn offset(&self) -> usize;
    fn mindex(&self) -> usize;
    fn buf(&self) -> String;
    fn new() -> Self;
    fn next_state(&mut self);
    fn next_substate(&mut self);
    fn reverse(&mut self);
}
