mod compositor;
mod display;

use compositor::CakeCompositor;
use display::CakeDisplay;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut cake_display, mut event_loop) = CakeDisplay::new()?;

    let socket_name = cake_display
        .socket_name()
        .ok_or("Failed to create socket")?;

    println!("Wayland socket: {:?}", socket_name);
    println!("Set WAYLAND_DISPLAY={:?} to connect clients", socket_name);

    let mut compositor_data = CakeCompositor {
        compositor_state: cake_display.compositor_state.clone(),
        xdg_shell_state: cake_display.xdg_shell_state.clone(),
        space: cake_display.space.clone(),
    };

    event_loop.handle().insert_source(
        Generic::new(
            cake_display.display,
            calloop::Interest::READ,
            calloop::Mode::Level,
        ),
        |_, display, data| {
            display.dispatch_clients(data).unwrap();
            Ok(calloop::PostAction::Continue)
        },
    )?;

    event_loop.run(None, &mut compositor_data, |_| {})?;

    Ok(())
}
