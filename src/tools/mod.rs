//! Tool registry — builds the master list of all categories.

pub mod types;
pub mod discovery;
pub mod mapping;
pub mod crawling;
pub mod analysis;
pub mod automation;

use types::Category;

/// Returns all five recon categories in display order.
pub fn build_registry() -> Vec<&'static Category> {
    vec![
        &discovery::DISCOVERY,
        &mapping::MAPPING,
        &crawling::CRAWLING,
        &analysis::ANALYSIS,
        &automation::AUTOMATION,
    ]
}
