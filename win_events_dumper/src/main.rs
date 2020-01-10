#![allow(unused_imports)]

use win_events::channel_iter::ChannelIter;
use win_events::errors::Result;
use win_events::event_iter::WinEventsIter;
use winapi::um::winevt::*;

fn main() -> Result<()> {
    let mut counts = [0u64; 64];

    let p_types = [
        (EvtVarTypeNull, "EvtVarTypeNull"),
        (EvtVarTypeString, "EvtVarTypeString"),
        (EvtVarTypeAnsiString, "EvtVarTypeAnsiString"),
        (EvtVarTypeSByte, "EvtVarTypeSByte"),
        (EvtVarTypeByte, "EvtVarTypeByte"),
        (EvtVarTypeInt16, "EvtVarTypeInt16"),
        (EvtVarTypeUInt16, "EvtVarTypeUInt16"),
        (EvtVarTypeInt32, "EvtVarTypeInt32"),
        (EvtVarTypeUInt32, "EvtVarTypeUInt32"),
        (EvtVarTypeInt64, "EvtVarTypeInt64"),
        (EvtVarTypeUInt64, "EvtVarTypeUInt64"),
        (EvtVarTypeSingle, "EvtVarTypeSingle"),
        (EvtVarTypeDouble, "EvtVarTypeDouble"),
        (EvtVarTypeBoolean, "EvtVarTypeBoolean"),
        (EvtVarTypeBinary, "EvtVarTypeBinary"),
        (EvtVarTypeGuid, "EvtVarTypeGuid"),
        (EvtVarTypeSizeT, "EvtVarTypeSizeT"),
        (EvtVarTypeFileTime, "EvtVarTypeFileTime"),
        (EvtVarTypeSysTime, "EvtVarTypeSysTime"),
        (EvtVarTypeSid, "EvtVarTypeSid"),
        (EvtVarTypeHexInt32, "EvtVarTypeHexInt32"),
        (EvtVarTypeHexInt64, "EvtVarTypeHexInt64"),
        (EvtVarTypeEvtHandle, "EvtVarTypeEvtHandle"),
        (EvtVarTypeEvtXml, "EvtVarTypeEvtXml"),
    ];

    for c in ChannelIter::new()? {
        let chan = c.expect("Bad chan?");
        //        println!("{}", chan);

        match WinEventsIter::get_logs_for(&chan, None) {
            Ok(witer) => {
                for weo in witer {
                    match weo {
                        Ok(mut we) => we.test(&mut counts),
                        Err(e) => eprintln!("Couldn't get event for {}: {}", chan, e),
                    };
                }
            }

            Err(e) => eprintln!("Couldn't get iter for {}: {}", chan, e),
        };
    }

    for (i, name) in p_types.iter() {
        let c = counts[*i as usize];
        if c > 0 {
            println!("{} :: {}", name, c);
        }
    }

    {
        let c = counts[63];
        if c > 0 {
            println!("Array :: {}", c);
        }
    }

    Ok(())
}
