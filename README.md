# cake_wl

### Description

A Wayland compositor for fun

Phase 1: Core Wayland Server

 - Create wl_display using smithay.

 - Register core globals: wl_compositor, wl_shm, wl_seat.

 - Implement minimal event loop with calloop.

Phase 2: Vulkan Initialization

 - Initialize Vulkan instance + physical device.

 - Create logical device + queues (graphics + present).

 - Setup swapchain for each output (monitor) via DRM/KMS or X11/Wayland testing backend.

Phase 3: Surface Management

 - Accept Wayland surfaces (xdg-shell/wl_surface).

 - If clients provide dmabuf buffers, import them into Vulkan images.

 - Keep track of surface metadata: position, size, scaling.

Phase 4: Rendering

 - Render surfaces to Vulkan framebuffer:

 - Map client buffers → Vulkan image → render quad.

 - Handle double/triple buffering for smooth presentation.

 - Synchronize with DRM/KMS (vblank).

Phase 5: Input Handling

 - Setup seat: keyboard, pointer, touch.

 - Map input to focused surface.

 - Handle modifiers & shortcuts via xkbcommon.

Phase 6: Layout / Window Management

 - Basic stacking or tiling layout.

 - Focus switching, resize/move.

Phase 7: Polishing

 - Cursor rendering (Vulkan quad).

 - Multi-monitor support.

 - decorations, compositor effects.
