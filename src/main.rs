use win_events;
use win_events::errors::WinEvtError;
use win_events::event_iter::WinEventsIter;
use win_events::renderer::Renderer;

fn main() -> Result<(), WinEvtError>{
    let iter = WinEventsIter::get_logs_for(
        "Security",
        None,
    )?;

    let mut rend = Renderer::new();

    for e in iter {
        println!("{}", rend.render(e.unwrap())?);
    }

    Ok(())
}