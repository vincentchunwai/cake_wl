use smithay::{
    desktop::{Space, Window},
    input::{Seat, SeatHandler, SeatState, keyboard::LedState, pointer::CursorImageStatus},
    reexports::wayland_server::{
        Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New,
        backend::ClientData,
        protocol::{
            wl_compositor::WlCompositor, wl_shm::WlShm, wl_subcompositor::WlSubcompositor,
            wl_surface::WlSurface,
        },
    },
    wayland::{
        buffer::BufferHandler,
        compositor::{CompositorClientState, CompositorHandler, CompositorState},
        shell::xdg::{
            PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
        },
        shm::{ShmHandler, ShmPoolUserData, ShmState},
    },
};

pub struct CakeCompositor {
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub shm_state: ShmState,
    pub seat_state: SeatState<Self>,
    pub space: Space<usize>,
}

impl Default for CakeCompositor {
    fn default(display: &DisplayHandle) -> Self {
        Self {
            compositor_state: CompositorState::new::<Self>(display),
            xdg_shell_state: XdgShellState::new::<Self>(display),
            shm_state: ShmState::new::<Self>(display, vec![]),
            seat_state: SeatState::new(),
            space: Space::default(),
        }
    }
}

impl CakeCompositor {
    pub fn new(display: &DisplayHandle) -> Self {
        Self {
            compositor_state: CompositorState::new::<Self>(display),
            xdg_shell_state: XdgShellState::new::<Self>(display),
            shm_state: ShmState::new::<Self>(display, vec![]),
            seat_state: SeatState::new(),
            space: Space::default(),
        }
    }
}

impl GlobalDispatch<WlCompositor, (), CakeCompositor> for CakeCompositor {
    fn bind(
        _state: &mut Self,
        _handle: &DisplayHandle,
        _client: &Client,
        resource: New<WlCompositor>,
        _global_data: &(),
        data_init: &mut DataInit<'_, Self>,
    ) {
        data_init.init(resource, ());
    }
}

impl GlobalDispatch<WlSubcompositor, (), CakeCompositor> for CakeCompositor {
    fn bind(
        _state: &mut Self,
        _handle: &DisplayHandle,
        _client: &Client,
        resource: New<WlSubcompositor>,
        _global_data: &(),
        data_init: &mut DataInit<'_, Self>,
    ) {
        data_init.init(resource, ());
    }
}

impl GlobalDispatch<WlShm, (), CakeCompositor> for CakeCompositor {
    fn bind(
        _state: &mut Self,
        _handle: &DisplayHandle,
        _client: &Client,
        resource: New<WlShm>,
        _global_data: &(),
        data_init: &mut DataInit<'_, Self>,
    ) {
        data_init.init(resource, ());
    }
}

impl Dispatch<WlShm, (), CakeCompositor> for CakeCompositor {
    fn request(
        _state: &mut Self,
        _client: &Client,
        _resource: &WlShm,
        _request: <WlShm as smithay::reexports::wayland_server::Resource>::Request,
        _data: &(),
        _dhandle: &DisplayHandle,
        _data_init: &mut DataInit<'_, Self>,
    ) {
    }
}

impl
    Dispatch<
        smithay::reexports::wayland_server::protocol::wl_shm_pool::WlShmPool,
        ShmPoolUserData,
        CakeCompositor,
    > for CakeCompositor
{
    fn request(
        state: &mut Self,
        client: &Client,
        resource: &smithay::reexports::wayland_server::protocol::wl_shm_pool::WlShmPool,
        request: <smithay::reexports::wayland_server::protocol::wl_shm_pool::WlShmPool as smithay::reexports::wayland_server::Resource>::Request,
        data: &ShmPoolUserData,
        dhandle: &DisplayHandle,
        data_init: &mut DataInit<'_, Self>,
    ) {
        state
            .shm_state
            .request(client, resource, request, data, dhandle, data_init);
    }
}

impl CompositorHandler for CakeCompositor {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client.get_data::<ClientData>().unwrap().compositor_state
    }

    fn new_surface(&mut self, _surface: &WlSurface) {}

    fn new_subsurface(&mut self, _surface: &WlSurface, _parent: &WlSurface) {}

    fn commit(&mut self, _surface: &WlSurface) {}
}

impl XdgShellHandler for CakeCompositor {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, _surface: ToplevelSurface) {
        // Handle new toplevel window
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {
        // Handle new popup window
    }

    fn toplevel_destroyed(&mut self, _surface: ToplevelSurface) {
        // Handle toplevel window destruction
    }

    fn popup_destroyed(&mut self, _surface: PopupSurface) {
        // Handle popup window destruction
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: Seat, _serial: u32) {
        // Handle grab
    }

    fn reposition_request(
        &mut self,
        _surface: PopupSurface,
        _positioner: PositionerState,
        _token: u32,
    ) {
    }
}

impl ShmHandler for CakeCompositor {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

impl SeatHandler for CakeCompositor {
    type KeyboardFocus = Window;
    type PointerFocus = Window;
    type TouchFocus = Window;

    fn seat_state(&mut self) -> &mut SeatState<CakeCompositor> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, _seat: &Seat<Self>, _focused: Option<&Self::KeyboardFocus>) {}

    fn cursor_image(&mut self, _seat: &Seat<Self>, _image: CursorImageStatus) {}

    fn led_state_changed(&mut self, _seat: &Seat<Self>, _led_state: LedState) {}
}

impl BufferHandler for CakeCompositor {
    fn buffer_destroyed(&mut self, _buffer: &smithay::wayland::buffer::) {
        // Handle buffer destruction
    }
}

// Implement SeatHandler
impl SeatHandler for CakeCompositor {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, _seat: &Seat<Self>, _focused: Option<&Self::KeyboardFocus>) {
        // Handle focus changes
    }

    fn cursor_image(
        &mut self,
        _seat: &Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {
        // Handle cursor image changes
    }

    fn led_state_changed(
        &mut self,
        _seat: &Seat<Self>,
        _led_state: smithay::input::keyboard::LedState,
    ) {
        // Handle LED state changes
    }
}

// We need to use macros to implement all the required Dispatch traits
smithay::delegate_compositor!(CakeCompositor);
smithay::delegate_xdg_shell!(CakeCompositor);
smithay::delegate_shm!(CakeCompositor);
