use clap::Parser;

pub mod notification;

#[derive(Parser)]
pub struct Pomodoro {
    #[clap(short, long, parse(try_from_str))]
    pub focus: u8,
    #[clap(short, long, parse(try_from_str))]
    pub rest: u8,
}
