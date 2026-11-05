/// A simulation variable registered for writing with [`crate::SimConnect::register_variable`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VariableId(pub(crate) u32);
