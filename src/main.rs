use win_events;
use win_events::errors::WinEvtError;
use win_events::event_iter::WinEventsIter;
use win_events::renderer::Renderer;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::prelude::*;
use win_events::channel_iter::ChannelIter;

const DUMP: bool = false;
const LEVELS: bool = false;

fn dump_chan(chan: &str, fh: &mut GzEncoder<File>) -> Result<(), WinEvtError> {
    println!("Processing {}", chan);
    let iter = WinEventsIter::get_logs_for(chan, None)?;

    let mut rend = Renderer::new();

    for e in iter {
        match e {
            Err(err) => return Err(err),
            Ok(we) => writeln!(fh, "{}", rend.render(we)?).expect("Couldn't write entry"),
        }
    }

    Ok(())
}

fn print_channels() -> Result<(), WinEvtError> {
    let fh = File::create("events.xml.gz").expect("Couldn't open out file");
    //    let mut fh = BufWriter::with_capacity(1024 * 16, fh);
    let mut fh = GzEncoder::new(fh, Compression::new(3));

    for c in ChannelIter::new().expect("Couldn't build channel iter") {
        match c {
            Err(err) => return Err(err),
            Ok(n) => {
                if let Err(e) = dump_chan(n.as_str(), &mut fh) {
                    eprintln!("Error dumping {}: {}", n, e)
                }
            }
        }
    }
    Ok(())
}

fn print_levels() -> Result<(), WinEvtError> {
    Ok(())
}

fn main() -> Result<(), WinEvtError> {
    if DUMP {
        print_channels()?;
    }

    if LEVELS {
        print_levels()?;
    }

    Ok(())
}
