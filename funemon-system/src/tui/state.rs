/// Estado simple para la TUI
pub struct TuiState {
    pub current_view: TuiView,
    pub selected_index: usize,
}

impl Default for TuiState {
    fn default() -> Self {
        Self {
            current_view: TuiView::Projects,
            selected_index: 0,
        }
    }
}

/// Vistas disponibles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TuiView {
    Projects,
    Sessions,
    Memories,
    Search,
    Help,
}

impl std::fmt::Display for TuiView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TuiView::Projects => write!(f, "Projects"),
            TuiView::Sessions => write!(f, "Sessions"),
            TuiView::Memories => write!(f, "Memories"),
            TuiView::Search => write!(f, "Search"),
            TuiView::Help => write!(f, "Help"),
        }
    }
}
