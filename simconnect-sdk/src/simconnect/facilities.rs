use crate::{bindings, success, FacilityType, SimConnect, SimConnectError};

impl SimConnect {
    /// Request a list of all the facilities of a given type currently held in the facilities cache.
    ///
    /// # Errors
    /// - [`crate::SimConnectError::ObjectAlreadyRegistered`] -- Only one request or subscription per facility type is allowed. If you wish to create a new one, unsubscribe first using [`crate::SimConnect::unsubscribe_to_facilities`].
    ///
    /// # Remarks
    /// The simulation keeps a facilities cache of all the airports, waypoints, NDB and VOR stations within a certain radius of the user aircraft.
    /// This radius varies depending on where the aircraft is in the world, but is at least large enough to encompass the whole of the reality bubble for airports and waypoints, and can be over 200 miles for VOR and NDB stations.
    /// As the user aircraft moves facilities will be added to, and removed from, the cache. However, in the interests pf performance, hysteresis is built into the system.
    #[tracing::instrument(
        name = "SimConnect::request_facilities_list",
        level = "debug",
        skip(self)
    )]
    pub fn request_facilities_list(
        &mut self,
        facility_type: FacilityType,
    ) -> Result<(), SimConnectError> {
        let type_name = facility_type.to_type_name();
        let request_id = self.new_request_id(type_name)?;

        success!(unsafe {
            bindings::SimConnect_RequestFacilitiesList(
                self.handle.as_ptr(),
                facility_type.into(),
                request_id,
            )
        })
    }

    /// Request notifications when a facility of a certain type is added to the facilities cache.
    ///
    /// When this function is first called, a full list from the cache will be sent, thereafter just the additions will be transmitted.
    /// No notification is given when a facility is removed from the cache.
    /// To terminate these notifications use the [`crate::SimConnect::unsubscribe_to_facilities`] function.
    ///
    /// # Errors
    /// - [`crate::SimConnectError::ObjectAlreadyRegistered`] -- Only one subscription or request per facility type is allowed. If you wish to create a new one, unsubscribe first using [`crate::SimConnect::unsubscribe_to_facilities`].
    ///
    /// # Remarks
    /// The simulation keeps a facilities cache of all the airports, waypoints, NDB and VOR stations within a certain radius of the user aircraft.
    /// This radius varies depending on where the aircraft is in the world, but is at least large enough to encompass the whole of the reality bubble for airports and waypoints, and can be over 200 miles for VOR and NDB stations.
    /// As the user aircraft moves facilities will be added to, and removed from, the cache. However, in the interests pf performance, hysteresis is built into the system.
    #[tracing::instrument(
        name = "SimConnect::subscribe_to_facilities",
        level = "debug",
        skip(self)
    )]
    pub fn subscribe_to_facilities(
        &mut self,
        facility_type: FacilityType,
    ) -> Result<(), SimConnectError> {
        let type_name = facility_type.to_type_name();
        let request_id = self.new_request_id(type_name)?;

        success!(unsafe {
            bindings::SimConnect_SubscribeToFacilities(
                self.handle.as_ptr(),
                facility_type.into(),
                request_id,
            )
        })
    }

    /// Request that notifications of additions to the facilities cache are not longer sent.
    ///
    /// # Remarks
    /// This is used to terminate notifications generated by the [`crate::SimConnect::request_facilities_list`] or [`crate::SimConnect::subscribe_to_facilities`] functions.
    #[tracing::instrument(
        name = "SimConnect::unsubscribe_to_facilities",
        level = "debug",
        skip(self)
    )]
    pub fn unsubscribe_to_facilities(
        &mut self,
        facility_type: FacilityType,
    ) -> Result<(), SimConnectError> {
        let type_name = facility_type.to_type_name();

        success!(unsafe {
            bindings::SimConnect_UnsubscribeToFacilities(self.handle.as_ptr(), facility_type.into())
        })?;

        self.unregister_request_id_by_type_name(&type_name);

        Ok(())
    }
}
