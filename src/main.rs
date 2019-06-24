use std::fs::File;
use std::io::prelude::*;

use flate2::write::GzEncoder;
use flate2::Compression;

use widestring::{U16CStr, U16CString};
use win_events;
use win_events::channel_iter::ChannelIter;
use win_events::errors::WinEvtError;
use win_events::event_iter::WinEventsIter;
use win_events::pub_metadata::PubMetadata;
use win_events::pub_metadata_fields::{META_PUBLISHER_GUID, META_PUBLISHER_RESOURCE_FPATH};
use win_events::renderer::Renderer;

const DUMP: bool = false;
const LEVELS: bool = true;

const TEST_PROVIDER: &str = "PowerShell";

fn dump_chan(chan: &str, rend: &mut Renderer, fh: &mut GzEncoder<File>) -> Result<(), WinEvtError> {
    println!("Processing {}", chan);
    let iter = WinEventsIter::get_logs_for(chan, None)?;

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

    let mut rend = Renderer::new();

    for c in ChannelIter::new().expect("Couldn't build channel iter") {
        match c {
            Err(err) => return Err(err),
            Ok(n) => {
                if let Err(e) = dump_chan(n.as_str(), &mut rend, &mut fh) {
                    eprintln!("Error dumping {}: {}", n, e)
                }
            }
        }
    }
    Ok(())
}

fn print_levels() -> Result<(), WinEvtError> {
    println!("Getting meta");
    let mut meta = PubMetadata::for_publisher(TEST_PROVIDER.to_string())?;

    println!("Getting prop");
    let prop = meta.get_prop(META_PUBLISHER_RESOURCE_FPATH)?;
    println!("{} / {}", prop.Count, prop.Type);

    let guid = unsafe { prop.u.StringVal() };
    println!(
        "{}",
        unsafe { U16CStr::from_ptr_str(*guid) }.to_string_lossy()
    );
    //    println!("{}", unsafe{U16CString::from_ptr_str(*guid)}.to_string_lossy());

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
