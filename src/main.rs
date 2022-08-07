use std::thread;
use std::time::Duration;

use clap::Parser;
use tomacto::notification::*;
use tomacto::Pomodoro;

fn main() {
    let args = Pomodoro::parse();

    set_default_application();

    loop {
        send_focus_notification(&args.focus);

        thread::sleep(Duration::from_secs((args.focus * 60) as u64));

        send_rest_notification(&args.rest);

        thread::sleep(Duration::from_secs((args.rest * 60) as u64));
    }
}
