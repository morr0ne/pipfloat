use hyprland::{
    data::Clients,
    dispatch::{Dispatch, DispatchType, WindowIdentifier},
    event_listener::EventListener,
    shared::HyprData,
};

fn main() -> hyprland::Result<()> {
    let mut event_listener = EventListener::new();

    println!("Started watching for windows");

    event_listener.add_window_title_change_handler(|_address| {
        for client in Clients::get().unwrap() {
            if client.title == "Picture-in-Picture" && !client.floating {
                Dispatch::call(DispatchType::ToggleFloating(Some(WindowIdentifier::Title(
                    "Picture-in-Picture",
                ))))
                .unwrap();

                println!("Floated window")
            }
        }
    });

    event_listener.start_listener()
}
