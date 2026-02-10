use crate::elevator_error::ElevatorError;

pub struct Elevator {
    floor: usize
}

pub fn new(floor: usize) -> Result<Elevator, ElevatorError> {
    Elevator::new(floor)
}

impl Elevator {
    pub fn new(floor: usize) -> Result<Self, ElevatorError> {
        if floor > 5 {
            return Err(ElevatorError::InvalidFloor(floor));
        }
        
        Ok(Elevator { floor: floor })
    }

    pub fn state(&self) -> usize {
        self.floor
    } 
}