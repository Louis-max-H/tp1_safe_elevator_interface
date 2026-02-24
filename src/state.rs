#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    MovingUp,
    DoorsOpen,
    MovingDown,
    Idle,
}
