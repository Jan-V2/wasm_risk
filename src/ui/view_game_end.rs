use crate::ui::wrap_elem::{WrapDiv, WrapHtml};

#[derive(Clone, Default)]
pub struct StateGameEnd {
    pub active: bool,
    pub armies: u32,
}

pub struct ViewGameEnd {
    state: StateGameEnd,
    template: WrapHtml,
    count_label: WrapDiv,
}