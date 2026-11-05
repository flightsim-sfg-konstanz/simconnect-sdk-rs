use std::ffi::c_void;

use crate::{bindings, success, SimConnect, SimConnectError};

/// The request name of the input event list; one request can be pending at a time.
const INPUT_EVENTS_REQUEST: &str = "input events";

impl SimConnect {
    /// Request the input events of the user aircraft. They arrive in one or more [`crate::Notification::InputEventList`].
    /// A request still unanswered, possibly lost, is sent again under its ID.
    #[tracing::instrument(
        name = "SimConnect::enumerate_input_events",
        level = "debug",
        skip(self)
    )]
    pub fn enumerate_input_events(&mut self) -> Result<(), SimConnectError> {
        let request_id = match self.registered_objects.get(INPUT_EVENTS_REQUEST) {
            Some(request) => request.id,
            None => self.new_request_id(INPUT_EVENTS_REQUEST.to_string(), true)?,
        };

        let result = success!(unsafe {
            bindings::SimConnect_EnumerateInputEvents(self.handle.as_ptr(), request_id)
        });
        if result.is_err() {
            self.unregister_request_id_by_type_name(INPUT_EVENTS_REQUEST);
        }
        result
    }

    /// Set the input event with the hash from [`crate::InputEvent::hash`] to `value`.
    /// Numeric input events only.
    #[tracing::instrument(name = "SimConnect::set_input_event", level = "debug", skip(self))]
    pub fn set_input_event(&self, hash: u64, value: f64) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_SetInputEvent(
                self.handle.as_ptr(),
                hash,
                std::mem::size_of::<f64>() as u32,
                (&value as *const f64).cast_mut().cast::<c_void>(),
            )
        })
    }
}
