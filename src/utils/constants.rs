
pub const ENGINE_VERSION_MAJOR: i32 = 0;
pub const ENGINE_VERSION_MINOR: i32 = 0;
pub const ENGINE_VERSION_PATCH: i32 = 1;
#[cfg(debug_assertions)]
pub const ENGINE_VERSION_IN_DEV: bool = true;
#[cfg(not(debug_assertions))]
pub const ENGINE_VERSION_IN_DEV: bool = false;