//! AI item representing a query to be sent to Gemini.

use crate::assets::PhosphorIcon;

/// An AI item representing a query to be answered by Gemini.
#[derive(Clone, Debug)]
pub struct AiItem {
    /// Unique identifier for this item (e.g., "ai-what-is-rust")
    pub id: String,
    /// Display name (e.g., "Ask Gemini")
    pub name: String,
    /// The user's query
    pub query: String,
}

impl AiItem {
    /// Create a new AI item for a query.
    pub fn new(query: String) -> Self {
        let id = format!("ai-{}", query.replace(' ', "-").to_lowercase());
        let name = "Ask Gemini".to_string();
        Self { id, name, query }
    }

    /// Get the icon for this AI item.
    pub fn icon(&self) -> PhosphorIcon {
        PhosphorIcon::Brain
    }

    /// Get the action label.
    pub fn action_label(&self) -> &'static str {
        "Ask"
    }

    /// Get a description for this item.
    pub fn description(&self) -> &'static str {
        "Use Gemini to answer your question"
    }
}
