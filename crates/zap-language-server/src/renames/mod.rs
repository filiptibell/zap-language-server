mod namespaces;
mod types;

pub use self::namespaces::rename as rename_for_namespaces;
pub use self::types::rename as rename_for_types;

pub use self::namespaces::prepare as rename_prepare_for_namespaces;
pub use self::types::prepare as rename_prepare_for_types;
