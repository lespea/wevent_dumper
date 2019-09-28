//use std::fs::File;
//use std::io::prelude::*;
//
//use flate2::write::GzEncoder;
//use flate2::Compression;
//
//use win_events;
//use win_events::channel_iter::ChannelIter;
//use win_events::errors::WinEvtError;
//use win_events::event_iter::WinEventsIter;
//use win_events::pub_metadata_fetcher::PubMetadataFetcher;
//use win_events::renderer::Renderer;
//use win_events::vwrapper::WevWrapper;
//
//const DUMP: bool = false;
//const LEVELS: bool = true;
//
//fn dump_chan(chan: &str, rend: &mut Renderer, fh: &mut GzEncoder<File>) -> Result<(), WinEvtError> {
//    println!("Processing {}", chan);
//    let iter = WinEventsIter::get_logs_for(chan, None)?;
//
//    for e in iter {
//        match e {
//            Err(err) => return Err(err),
//            Ok(we) => writeln!(fh, "{}", rend.render(we)?).expect("Couldn't write entry"),
//        }
//    }
//
//    Ok(())
//}
//
//fn print_channels() -> Result<(), WinEvtError> {
//    let fh = File::create("events.xml.gz").expect("Couldn't open out file");
//    //    let mut fh = BufWriter::with_capacity(1024 * 16, fh);
//    let mut fh = GzEncoder::new(fh, Compression::new(3));
//
//    let mut rend = Renderer::new();
//
//    for c in ChannelIter::new().expect("Couldn't build channel iter") {
//        match c {
//            Err(err) => return Err(err),
//            Ok(n) => {
//                if let Err(e) = dump_chan(n.as_str(), &mut rend, &mut fh) {
//                    eprintln!("Error dumping {}: {}", n, e)
//                }
//            }
//        }
//    }
//    Ok(())
//}
//
//fn print_levels() -> Result<(), WinEvtError> {
//    let mut varw = WevWrapper::new().unwrap();
//
//    let out = std::io::stdout();
//    let mut out = out.lock();
//
//    for c in ChannelIter::new().unwrap() {
//        if let Ok(n) = c {
//            if let Ok(mut meta) = PubMetadataFetcher::for_publisher(n.clone()) {
//                let _ = writeln!(out, "{} - {:?}", n, meta.get_metadata(&mut varw));
//            }
//        }
//    }
//
//    Ok(())
//}

use win_events::channel_iter::ChannelIter;
use win_events::errors::Result;

fn main() -> Result<()> {
    //    if DUMP {
    //        print_channels()?;
    //    }
    //
    //    if LEVELS {
    //        print_levels()?;
    //    }
    //

    for c in ChannelIter::new()? {
        println!("{}", c.expect("Bad chan?"));
    }

    Ok(())
}
