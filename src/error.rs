use enigo::{InputError, NewConError};

pub type Result<T> = std::result::Result<T, JsError>;

pub struct JsError {
    status: &'static str,
    reason: String,
}

impl JsError {
    pub fn new<T: ToString>(status: &'static str, reason: T) -> Self {
        Self {
            status,
            reason: reason.to_string(),
        }
    }

    pub fn invalid_input(key: &str) -> Self {
        Self::new(
            "INVALID_INPUT",
            format!("you tried to simulate invalid input: ({key})"),
        )
    }
}

impl From<NewConError> for JsError {
    fn from(value: NewConError) -> Self {
        match value {
            NewConError::EstablishCon(_) => Self::new("CANT_ESTABLISH_CONNECTION", value),
            NewConError::NoPermission => Self::new("NO_PERMISSION", value),
            NewConError::Reply => Self::new("INVALID_REPLY", value),
            NewConError::NoEmptyKeycodes => Self::new("KEYMAP_FULL", value),
        }
    }
}

impl From<InputError> for JsError {
    fn from(value: InputError) -> Self {
        match value {
            InputError::Mapping(_) => Self::new("INVALID_MAPPING", value),
            InputError::Unmapping(_) => Self::new("INVALID_UNMAPPING", value),
            InputError::NoEmptyKeycodes => Self::new("NO_KEYMAP_SPACE", value),
            InputError::Simulate(_) => Self::new("PROTOCOL_ERROR", value),
            InputError::InvalidInput(_) => Self::new("INVALID_INPUT", value),
        }
    }
}

impl From<JsError> for napi::Error<&'static str> {
    fn from(value: JsError) -> Self {
        napi::Error::new(value.status, value.reason)
    }
}

impl From<JsError> for napi::JsError<&'static str> {
    fn from(value: JsError) -> Self {
        napi::JsError::from(napi::Error::from(value))
    }
}
