use ply_engine::{Ui, align::{AlignX::CenterX, AlignY::CenterY}, fit, fixed, grow, id::Id};

use crate::Theme;

pub(crate) fn button(
    ui: &mut Ui,
    theme: &Theme,
    label: impl AsRef<str>,
    id: impl Into<Id>,
) {
    ui.element()
        .width(fit!())
        .height(fixed!(32.0))
        .corner_radius(6.0)
        .id(id)
        .children(|ui| {
            let bg = if ui.pressed() {
                theme.accent
            } else {
                theme.surface
            };

            ui.element().width(fit!()).height(grow!())
                .background_color(bg)
                .corner_radius(6.0)
                .layout(|l| l.padding((0, 16, 0, 16)).align(CenterX, CenterY))
                .children(|ui| {
                    ui.text(label.as_ref(), |t| t.font_size(14).color(theme.text_primary));
                });
        });
}
