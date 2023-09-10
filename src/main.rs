use hyprland::{
    data::Clients,
    dispatch::{Dispatch, DispatchType, WindowIdentifier},
    event_listener::EventListener,
    shared::HyprData,
};

fn main() -> hyprland::Result<()> {
    let mut event_listener = EventListener::new();

    println!("Started watching for windows");

    event_listener.add_window_title_change_handler(|address| {
        if let Ok(Some(client)) = Clients::get().map(|mut clients| {
            clients.find(|client| client.address.to_string() == format!("0x{address}"))
        }) {
            if client.title == "Picture-in-Picture" && !client.floating {
                let dispatch_result = Dispatch::call(DispatchType::ToggleFloating(Some(
                    WindowIdentifier::Title("Picture-in-Picture"),
                )));

                if dispatch_result.is_err() {
                    eprintln!("Failed to float window")
                } else {
                    println!("Floated window")
                }
            }
        } else {
            eprintln!("Failed to read clients")
        }
    });

    event_listener.start_listener()
}
