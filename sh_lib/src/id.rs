#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Id {
    _inner: String,
}

impl Id {
    pub fn new() -> Self {
        Self {
            _inner: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub fn with_inner(inner_value: impl Into<String>) -> Self {
        Self {
            _inner: inner_value.into(),
        }
    }

    pub fn from_string(name: impl Into<String>) -> Self {
        Self {
            _inner: gen_id(name.into()),
        }
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self._inner)
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

fn gen_id(name: impl Into<String>) -> String {
    uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_DNS, name.into().as_bytes()).to_string()
}
