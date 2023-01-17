use crate::{
    bindings, success, ClientEvent, FlxClientEvent, NotificationGroup, SimConnect, SimConnectError,
    SystemEventRequest,
};

impl SimConnect {
    /// Associates a client defined event with a Microsoft Flight Simulator event name.
    ///
    /// WIP
    #[tracing::instrument(name = "SimConnect::register_event", level = "debug", skip(self))]
    pub fn register_event(
        &self,
        event: ClientEvent,
        notification_group: NotificationGroup,
    ) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_MapClientEventToSimEvent(
                self.handle.as_ptr(),
                event as u32,
                event.into_c_char(),
            )
        })?;

        success!(unsafe {
            bindings::SimConnect_AddClientEventToNotificationGroup(
                self.handle.as_ptr(),
                notification_group as u32,
                event as u32,
                0,
            )
        })?;

        success!(unsafe {
            bindings::SimConnect_SetNotificationGroupPriority(
                self.handle.as_ptr(),
                notification_group as u32,
                1,
            )
        })
    }

    pub fn map_client_event_to_sim_event(
        &self,
        event: impl FlxClientEvent,
    ) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_MapClientEventToSimEvent(
                self.handle.as_ptr(),
                event.event_id(),
                event.event_name(),
            )
        })
    }

    pub fn transmit_event(&self, event: impl FlxClientEvent) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_TransmitClientEvent(
                self.handle.as_ptr(),
                bindings::SIMCONNECT_OBJECT_ID_USER,
                event.event_id(),
                event.data(),
                bindings::SIMCONNECT_GROUP_PRIORITY_HIGHEST,
                bindings::SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY,
            )
        })
    }

    /// Request that a specific system event is notified to the client.
    #[tracing::instrument(
        name = "SimConnect::subscribe_to_system_event",
        level = "debug",
        skip(self)
    )]
    pub fn subscribe_to_system_event(
        &mut self,
        event: SystemEventRequest,
    ) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_SubscribeToSystemEvent(
                self.handle.as_ptr(),
                event as u32,
                event.into_c_char(),
            )
        })
    }

    /// Request that notifications are no longer received for the specified system event.
    #[tracing::instrument(
        name = "SimConnect::unsubscribe_from_system_event",
        level = "debug",
        skip(self)
    )]
    pub fn unsubscribe_from_system_event(
        &mut self,
        event: SystemEventRequest,
    ) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_UnsubscribeFromSystemEvent(self.handle.as_ptr(), event as u32)
        })
    }
}
