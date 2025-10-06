use smithay::wayland::compositor::CompositorClientState;

#[derive(Debug)]
pub struct ClientData {
    pub compositor_state: CompositorClientState,
}

impl ClientData {
    pub fn new() -> Self {
        Self {
            compositor_state: CompositorClientState::default(),
        }
    }
}

impl Default for ClientData {
    fn default() -> Self {
        Self::new()
    }
}
