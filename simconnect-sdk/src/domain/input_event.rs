/// An input event of the user aircraft, listed by [`crate::SimConnect::enumerate_input_events`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputEvent {
    /// The input event name.
    pub name: String,
    /// The hash [`crate::SimConnect::set_input_event`] takes.
    pub hash: u64,
}
