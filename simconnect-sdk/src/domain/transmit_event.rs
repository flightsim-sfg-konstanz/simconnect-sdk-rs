use std::ffi::CStr;

/// A client event sent to the simulator with [`crate::SimConnect::transmit_event`].
///
/// Map it first with [`crate::SimConnect::map_client_event_to_sim_event`].
pub trait FlxClientEvent {
    /// The client event ID, unique per simulator event name.
    fn event_id(&self) -> u32;
    /// The simulator event name, e.g. `c"BATTERY1_SET"`.
    fn event_name(&self) -> &'static CStr;
    /// The five event parameters, `[0]` first.
    fn data(&self) -> (u32, u32, u32, u32, u32);
}
