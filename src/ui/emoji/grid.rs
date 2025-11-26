use crate::emoji::EmojiItem;
use crate::ui::theme::theme;
use gpui::{Div, ElementId, SharedString, Stateful, div, prelude::*};

/// Render a single emoji cell in the grid.
pub fn render_emoji_cell(emoji: &EmojiItem, selected: bool, index: usize) -> Stateful<Div> {
    let t = theme();

    let bg = if selected {
        t.emoji_cell_selected_bg
    } else {
        gpui::hsla(0.0, 0.0, 0.0, 0.0) // transparent
    };

    div()
        .id(ElementId::NamedInteger("emoji-cell".into(), index as u64))
        .w(t.emoji_cell_size)
        .h(t.emoji_cell_size)
        .flex()
        .items_center()
        .justify_center()
        .bg(bg)
        .rounded(gpui::px(6.0))
        .child(
            div()
                .text_size(t.emoji_font_size)
                .child(SharedString::from(emoji.emoji.clone())),
        )
}

/// Render a row of emoji cells.
pub fn render_emoji_row(
    emojis: &[EmojiItem],
    start_index: usize,
    selected_index: Option<usize>,
    _row_index: usize,
) -> Div {
    let t = theme();

    let mut row = div()
        .w_full()
        .flex()
        .flex_row()
        .justify_center()
        .gap(gpui::px(2.0));

    for (i, emoji) in emojis.iter().enumerate() {
        let global_idx = start_index + i;
        let selected = selected_index == Some(global_idx);
        row = row.child(render_emoji_cell(emoji, selected, global_idx));
    }

    // Pad with empty cells if row is not full
    let remaining = t.emoji_columns - emojis.len();
    for _ in 0..remaining {
        row = row.child(div().w(t.emoji_cell_size).h(t.emoji_cell_size));
    }

    row
}
