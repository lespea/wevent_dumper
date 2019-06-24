use win_events;
use win_events::errors::WinEvtError;
use win_events::event_iter::WinEventsIter;
use win_events::renderer::Renderer;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use win_events::channel_iter::ChannelIter;

fn dump_security() -> Result<(), WinEvtError> {
    println!("Getting the events");
    let iter = WinEventsIter::get_logs_for("Security", None)?;

    println!("Building the renderer");
    let mut rend = Renderer::new();

    let fh = File::create("events.xml").expect("Couldn't open out file");
    let mut fh = BufWriter::with_capacity(1024 * 16, fh);

    for e in iter {
        match e {
            Err(err) => return Err(err),
            Ok(we) => writeln!(fh, "{}", rend.render(we)?).expect("Couldn't write entry"),
        }
    }

    Ok(())
}

fn print_channels() -> Result<(), WinEvtError> {
    for c in ChannelIter::new().expect("Couldn't build channel iter") {
        match c {
            Err(err) => return Err(err),
            Ok(n) => println!("{}", n),
        }
    }
    Ok(())
}

fn main() -> Result<(), WinEvtError> {
    print_channels()
}
