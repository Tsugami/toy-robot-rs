mod cli;
mod toy_robot;

use clap::Parser;

use crate::cli::{normalize_direction, Cli, Commands};
use crate::toy_robot::{ToyRobot, ToyRobotImpl};

fn main() {
    let mut toy_robot = ToyRobotImpl::new();
    let cli = Cli::parse();

    match cli.command {
        Commands::Place(args) => {
            let direction = normalize_direction(args.facing);
            if let toy_robot::MovimentResult::Failure = toy_robot.place(args.x, args.y, direction) {
                println!("The toy robot was placed out of the table");
            }
        }
        Commands::Move => {
            if let toy_robot::MovimentResult::Failure = toy_robot.move_forward() {
                println!(
                    "The robot can't move forward on that direction, it may fall off the table."
                );
            }
        }
        Commands::Left => {
            toy_robot.turn_left();
        }
        Commands::Right => {
            toy_robot.turn_right();
        }
        Commands::Report => {
            println!("{}", toy_robot.report());
        }
    }

    println!("{}", toy_robot.report());
}
