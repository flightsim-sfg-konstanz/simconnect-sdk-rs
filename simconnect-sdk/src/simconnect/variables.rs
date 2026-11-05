use std::ffi::c_void;

use crate::{bindings, success, DataType, SimConnect, SimConnectError, VariableId};

impl SimConnect {
    /// Register a simulation variable, an `L:` variable for example, for writing with [`Self::set_variable`].
    ///
    /// # Errors
    /// - [`crate::SimConnectError::ObjectAlreadyRegistered`] -- The variable is already registered.
    #[tracing::instrument(name = "SimConnect::register_variable", level = "debug", skip(self))]
    pub fn register_variable(
        &mut self,
        name: &str,
        unit: &str,
    ) -> Result<VariableId, SimConnectError> {
        let key = format!("variable {name}");
        let id = self.new_request_id(key.clone(), false)?;

        if let Err(e) = self.add_to_data_definition(id, name, unit, DataType::Float64) {
            self.unregister_request_id_by_type_name(&key);
            return Err(e);
        }

        Ok(VariableId(id))
    }

    /// Set a variable registered with [`Self::register_variable`] on the user aircraft.
    #[tracing::instrument(name = "SimConnect::set_variable", level = "debug", skip(self))]
    pub fn set_variable(&self, variable: VariableId, value: f64) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_SetDataOnSimObject(
                self.handle.as_ptr(),
                variable.0,
                bindings::SIMCONNECT_OBJECT_ID_USER,
                bindings::SIMCONNECT_DATA_SET_FLAG_DEFAULT,
                1,
                std::mem::size_of::<f64>() as u32,
                (&value as *const f64).cast_mut().cast::<c_void>(),
            )
        })
    }
}
