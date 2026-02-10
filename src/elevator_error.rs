#[derive(Debug, Clone, PartialEq)]
pub enum ElevatorError {
    InvalidFloor(usize),
    DoorsAlreadyOpen,
    DoorsAlreadyClosed,
    CannotOpenWhileMoving,
    CannotMoveDoorsOpen,
    EmptyQueue,
}