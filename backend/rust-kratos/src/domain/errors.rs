#[derive(Debug)]
pub enum DomainError {
    NotAuthenticated,
    SessionExpired,
    AlreadyLoggedIn,
    InvalidCredentials,
    FlowNotFound,
    InvalidData(String),
    Conflict(String),
    NotFound(String),
    PrivilegedSessionRequired,
    Network(String),
    Unknown(String),
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::NotAuthenticated => write!(f, "Not authenticated"),
            DomainError::SessionExpired => write!(f, "Session expired"),
            DomainError::AlreadyLoggedIn => write!(f, "Already logged in"),
            DomainError::InvalidCredentials => write!(f, "Invalid credentials"),
            DomainError::FlowNotFound => write!(f, "Flow not found"),
            DomainError::InvalidData(e) => write!(f, "Invalid data: {}", e),
            DomainError::Conflict(e) => write!(f, "Conflict: {}", e),
            DomainError::NotFound(e) => write!(f, "Not found: {}", e),
            DomainError::PrivilegedSessionRequired => write!(f, "Privileged session required"),
            DomainError::Network(e) => write!(f, "Network error: {}", e),
            DomainError::Unknown(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl std::error::Error for DomainError {}
