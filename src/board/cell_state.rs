use board::Digit;
use bitset::Set;

/// Contains either a digit or all the candidates for an unsolved cell
#[derive(Copy, Clone)]
#[allow(missing_docs)]
pub enum CellState {
    Digit(Digit),
    Candidates(Set<Digit>),
}