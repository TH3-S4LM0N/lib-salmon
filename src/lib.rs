/// The logger
#[cfg(feature = "logger")]
pub mod logger;

/// The module with utils to diff collections
#[cfg(feature = "diff")]
pub mod diff;

/// The module to expand ``std::fs``
#[cfg(feature = "fs")]
pub mod fs;