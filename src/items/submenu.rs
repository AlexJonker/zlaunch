/// The layout style for a submenu.
/// Different submenus may have different UI layouts.
#[derive(Clone, Debug, Default)]
pub enum SubmenuLayout {
    /// Standard vertical list (default)
    #[default]
    List,
    /// Grid layout (e.g., for emoji picker)
    Grid { columns: usize },
    /// Custom layout identified by name (e.g., "calculator", "unit-converter")
    Custom(String),
}

/// A submenu item that opens a nested list or custom UI.
/// This is a placeholder for future implementation.
#[derive(Clone, Debug)]
pub struct SubmenuItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon_name: Option<String>,
    pub layout: SubmenuLayout,
}

impl SubmenuItem {
    pub fn new(
        id: String,
        name: String,
        description: Option<String>,
        icon_name: Option<String>,
        layout: SubmenuLayout,
    ) -> Self {
        Self {
            id,
            name,
            description,
            icon_name,
            layout,
        }
    }

    /// Create a simple list submenu
    pub fn list(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            icon_name: None,
            layout: SubmenuLayout::List,
        }
    }

    /// Create a grid submenu
    pub fn grid(id: impl Into<String>, name: impl Into<String>, columns: usize) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            icon_name: None,
            layout: SubmenuLayout::Grid { columns },
        }
    }

    /// Create a custom layout submenu
    pub fn custom(
        id: impl Into<String>,
        name: impl Into<String>,
        layout_name: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            icon_name: None,
            layout: SubmenuLayout::Custom(layout_name.into()),
        }
    }

    /// Builder method to set a description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Builder method to set an icon name.
    pub fn with_icon(mut self, icon_name: impl Into<String>) -> Self {
        self.icon_name = Some(icon_name.into());
        self
    }
}
