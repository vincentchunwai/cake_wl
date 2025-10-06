use smithay::reexports::{
    calloop::EventLoop,
    wayland_server::{Display, DisplayHandle, ListeningSocket},
};

use crate::compositor::CakeCompositor;
use calloop::LoopSignal;

pub struct CakeDisplay {
    pub display: Display<CakeCompositor>,
    pub display_handle: DisplayHandle,
    pub loop_signal: LoopSignal,
    pub socket: Option<ListeningSocket>,
}

impl CakeDisplay {
    pub fn new() -> Result<(Self, EventLoop<'static, CakeCompositor>), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::try_new()?;
        let loop_signal = event_loop.get_signal();

        let mut _display = Display::new()?;
        let display_handle = _display.handle();

        let _compositor = CakeCompositor::new(&display_handle);

        let socket = ListeningSocket::bind_auto("wayland", 0..=32).ok();

        let cake_display = CakeDisplay {
            display: _display,
            display_handle,
            loop_signal,
            socket,
        };

        Ok((cake_display, event_loop))
    }

    pub fn socket_name(&self) -> Option<&std::ffi::OsStr> {
        self.socket.as_ref()?.socket_name()
    }
}
