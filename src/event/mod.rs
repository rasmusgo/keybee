use crate::Button;

#[cfg(feature = "gilrs")]
mod gilrs;

#[cfg(feature = "winit")]
mod winit;

/// Represents an input event that can be processed by the library.
///
/// By enabling the `winit` or `gilrs` features, keybee supports converting
/// events from those libraries to this event type through the [`TryFrom`] and
/// [`TryInto`] traits.
#[derive(Debug)]
#[non_exhaustive]
pub enum Event {
    ButtonPressed(Button),
    ButtonReleased(Button),
    CursorMoved(f32, f32),
    MouseMotion(f32, f32),
}
