mod components;
mod datatypes;
mod query_string;
#[cfg(feature = "ssr")]
mod server;
mod hooks;

pub use components::*;
pub use datatypes::*;
pub use query_string::*;
pub use hooks::*;

#[cfg(feature = "ssr")]
pub use server::*;

pub(crate) const APP_CSS: &str = include_str!("../style/app.css");
pub(crate) const VARIABLES_CSS: &str = include_str!("../style/variables.css");
pub(crate) const PRINT_CSS: &str = include_str!("../style/print.css");
