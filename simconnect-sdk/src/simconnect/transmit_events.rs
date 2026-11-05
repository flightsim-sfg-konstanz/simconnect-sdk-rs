use crate::{bindings, success, FlxClientEvent, SimConnect, SimConnectError};

impl SimConnect {
    /// Associates the ID of a client event with its simulator event name.
    #[tracing::instrument(
        name = "SimConnect::map_client_event_to_sim_event",
        level = "debug",
        skip_all
    )]
    pub fn map_client_event_to_sim_event(
        &self,
        event: impl FlxClientEvent,
    ) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_MapClientEventToSimEvent(
                self.handle.as_ptr(),
                event.event_id(),
                event.event_name().as_ptr(),
            )
        })
    }

    /// Sends a mapped client event with its parameters to the user aircraft.
    #[tracing::instrument(name = "SimConnect::transmit_event", level = "debug", skip_all)]
    pub fn transmit_event(&self, event: impl FlxClientEvent) -> Result<(), SimConnectError> {
        let (data0, data1, data2, data3, data4) = event.data();

        success!(unsafe {
            bindings::SimConnect_TransmitClientEvent_EX1(
                self.handle.as_ptr(),
                bindings::SIMCONNECT_OBJECT_ID_USER,
                event.event_id(),
                bindings::SIMCONNECT_GROUP_PRIORITY_HIGHEST,
                bindings::SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY,
                data0,
                data1,
                data2,
                data3,
                data4,
            )
        })
    }
}
