use std::fs::File;
use std::io::prelude::*;

use flate2::write::GzEncoder;
use flate2::Compression;
use widestring::U16CStr;

use win_events;
use win_events::channel_iter::ChannelIter;
use win_events::errors::WinEvtError;
use win_events::event_iter::WinEventsIter;
use win_events::pub_metadata_fetcher::PubMetadataFetcher;
use win_events::pub_metadata_fields as meta_fields;
use win_events::renderer::Renderer;
use win_events::vwrapper::WevWrapper;
use winapi::um::winevt::{self, EVT_VARIANT_TYPE_ARRAY};

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
    let mut varw = WevWrapper::new().unwrap();

    println!("Getting meta");
    let mut meta = PubMetadataFetcher::for_publisher(TEST_PROVIDER.to_string())?;

    for field in meta_fields::PUB_META_FIELDS.iter() {
        println!("Getting {}", field.name);
        meta.get_prop(field, &mut varw);
        println!("{} / {}", varw.Count, varw.Type);
        if varw.Type == winevt::EvtVarTypeString {
            println!(
                "{}",
                unsafe { U16CStr::from_ptr_str(*unsafe { varw.u.StringVal() }) }.to_string_lossy()
            );
        }
    }
    //    println!("Getting prop");
    //    meta.get_prop(meta_fields::PARAMETER_FILE_PATH, &mut varw)?;
    //    println!("{} / {}", varw.Count, varw.Type);

    //    let guid = unsafe { varw.u.StringVal() };
    //    println!(
    //        "{}",
    //        unsafe { U16CStr::from_ptr_str(*guid) }.to_string_lossy()
    //    );
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
