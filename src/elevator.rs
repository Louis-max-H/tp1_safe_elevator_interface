use std::collections::VecDeque;

use crate::{elevator_error::ElevatorError, state::State};

pub struct Elevator {
    floor: usize,
    state: State,
    orders: VecDeque<usize>,
}

pub struct Status {
    pub floor: usize,
    pub state: State,
    pub queue: VecDeque<usize>,
}

pub fn new(floor: usize) -> Result<Elevator, ElevatorError> {
    if floor > 5 {
        return Err(ElevatorError::InvalidFloor(floor));
    }

    Ok(Elevator {
        floor: floor,
        state: State::Idle,
        orders: VecDeque::new(),
    })
}

impl Elevator {
    fn update_direction(&mut self) {
        if self.state != State::Idle {
            return;
        }

        if let Some(&destination) = self.orders.get(0) {
            if destination < self.floor {
                self.state = State::MovingDown;
            }

            if destination > self.floor {
                self.state = State::MovingUp;
            }

            if destination == self.floor {
                self.orders.pop_front();
                self.update_direction();
            }
        }
    }

    pub fn call(&mut self, floor: usize) -> Result<(), ElevatorError> {
        if floor > 5 {
            return Err(ElevatorError::InvalidFloor(floor));
        }

        // Check already here
        if floor == self.floor {
            return Ok(());
        }

        // Already in orders
        if self.orders.contains(&floor) {
            return Ok(());
        }

        // Add to order
        self.orders.push_back(floor);

        // If it's the only order, update direction
        if self.orders.len() == 1 {
            self.update_direction();
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), ElevatorError> {
        match self.state {
            State::DoorsOpen => {
                return Err(ElevatorError::CannotMoveDoorsOpen);
            }
            State::MovingUp => {
                self.floor += 1;
            }
            State::MovingDown => {
                self.floor -= 1;
            }
            State::Idle => {
                return Err(ElevatorError::EmptyQueue);
            }
        }

        if Some(&self.floor) == self.orders.get(0) {
            self.state = State::DoorsOpen;
            self.orders.pop_front();
        }

        Ok(())
    }

    pub fn open_doors(&mut self) -> Result<(), ElevatorError> {
        match self.state {
            State::DoorsOpen => Err(ElevatorError::DoorsAlreadyOpen),
            State::MovingDown | State::MovingUp => Err(ElevatorError::CannotOpenWhileMoving),
            State::Idle => {
                self.state = State::DoorsOpen;
                Ok(())
            }
        }
    }

    pub fn close_doors(&mut self) -> Result<(), ElevatorError> {
        if self.state != State::DoorsOpen {
            return Err(ElevatorError::DoorsAlreadyClosed);
        }

        self.state = State::Idle;
        self.update_direction();
        Ok(())
    }

    pub fn state(&self) -> State {
        self.state
    }
    pub fn floor(&self) -> usize {
        self.floor
    }
    pub fn queue(&self) -> &VecDeque<usize> {
        &self.orders
    }
    pub fn status(&self) -> Status {
        Status {
            floor: self.floor,
            state: self.state,
            queue: self.orders.clone(),
        }
    }
}
