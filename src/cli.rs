use crate::toy_robot;

#[derive(clap::Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    Place(Place),
    Move,
    Left,
    Right,
    Report,
}

#[derive(clap::Args)]
pub struct Place {
    pub x: i32,
    pub y: i32,
    #[clap(arg_enum)]
    pub facing: CliDirection,
}

#[derive(clap::ArgEnum, Clone)]
pub enum CliDirection {
    North,
    East,
    South,
    West,
}

pub fn normalize_direction(cli_direction: CliDirection) -> toy_robot::Direction {
    match cli_direction {
        CliDirection::North => toy_robot::Direction::North,
        CliDirection::East => toy_robot::Direction::East,
        CliDirection::South => toy_robot::Direction::South,
        CliDirection::West => toy_robot::Direction::West,
    }
}
