pub enum Direction {
    North,
    East,
    South,
    West,
}

pub enum MovimentResult {
    Success,
    Failure,
}

pub trait ToyRobot {
    fn new() -> Self;
    fn place(&mut self, x: i32, y: i32, facing: Direction) -> MovimentResult;
    fn move_forward(&mut self) -> MovimentResult;
    fn turn_left(&mut self);
    fn turn_right(&mut self);
    fn report(&self) -> String;
}

struct Area {
    height: i32,
    width: i32,
}

pub struct ToyRobotImpl {
    x: i32,
    y: i32,
    facing: Direction,
    area: Area,
}

fn direction_to_str(direction: &Direction) -> String {
    String::from(match direction {
        Direction::North => "North",
        Direction::East => "East",
        Direction::South => "South",
        Direction::West => "West",
    })
}

impl ToyRobot for ToyRobotImpl {
    fn new() -> Self {
        ToyRobotImpl {
            x: 0,
            y: 0,
            facing: Direction::North,
            area: Area {
                height: 5,
                width: 5,
            },
        }
    }

    fn report(&self) -> String {
        format!("{},{},{}", self.x, self.y, direction_to_str(&self.facing))
    }

    fn turn_left(&mut self) {
        match self.facing {
            Direction::North => self.facing = Direction::West,
            Direction::East => self.facing = Direction::North,
            Direction::South => self.facing = Direction::East,
            Direction::West => self.facing = Direction::South,
        }
    }

    fn turn_right(&mut self) {
        match self.facing {
            Direction::North => self.facing = Direction::East,
            Direction::East => self.facing = Direction::South,
            Direction::South => self.facing = Direction::West,
            Direction::West => self.facing = Direction::North,
        }
    }

    fn place(&mut self, x: i32, y: i32, facing: Direction) -> MovimentResult {
        if x < 0 || x > self.area.width || y < 0 || y > self.area.height {
            return MovimentResult::Failure;
        }

        self.x = x;
        self.y = y;
        self.facing = facing;

        return MovimentResult::Success;
    }

    fn move_forward(&mut self) -> MovimentResult {
        match self.facing {
            Direction::North => {
                if self.y + 1 < self.area.height {
                    self.y += 1;

                    return MovimentResult::Success;
                }

                return MovimentResult::Failure;
            }
            Direction::East => {
                if self.x + 1 < self.area.width {
                    self.x += 1;

                    return MovimentResult::Success;
                }

                return MovimentResult::Failure;
            }
            Direction::South => {
                if self.y - 1 >= 0 {
                    self.y -= 1;

                    return MovimentResult::Success;
                }

                return MovimentResult::Failure;
            }
            Direction::West => {
                if self.x - 1 >= 0 {
                    self.x -= 1;

                    return MovimentResult::Success;
                }

                return MovimentResult::Failure;
            }
        }
    }
}
