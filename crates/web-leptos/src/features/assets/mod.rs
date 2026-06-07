pub mod admin_guard;
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod ports;
pub mod ui;

pub use admin_guard::RequireAdmin;
pub use infrastructure::AssetHttpRepository;
pub use ui::{AssetFormPage, AssetListPage};
