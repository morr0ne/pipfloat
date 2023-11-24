use hyprland_ipc::{params::Window, Dispatcher, Event, EventListener, Result};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Started watching for windows");

    let mut event_listener = EventListener::new().await?;
    let dispatcher = Dispatcher::new()?;

    while let Some(event) = event_listener.try_next().await? {
        if let Event::WindowTitle { window_address } = event {
            let clients = dispatcher.clients().await?;

            if let Some(client) = clients
                .iter()
                .find(|client| client.address == format!("0x{window_address}"))
            {
                if client.title == "Picture-in-Picture" && !client.floating {
                    let dispatch_result = dispatcher
                        .toggle_floating(Some(Window::Address(format!("0x{window_address}"))))
                        .await;

                    let mut floated = false;

                    if dispatch_result.is_err() {
                        eprintln!("Failed to float window")
                    } else {
                        println!("Floated window");
                        floated = true;
                    }

                    if floated {
                        let dispatch_result = dispatcher
                            .pin(Some(Window::Address(format!("0x{window_address}"))))
                            .await;

                        if dispatch_result.is_err() {
                            eprintln!("Failed to pin window")
                        } else {
                            println!("Pinned window");
                        }
                    }
                }
            } else {
                eprintln!("Failed to find address")
            }
        }
    }

    Ok(())
}
