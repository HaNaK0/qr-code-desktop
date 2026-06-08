use std::fmt::Display;

use log::info;
use ply_engine::{
    align::{
        AlignX::{CenterX, Left},
        AlignY::{Bottom, CenterY, Top},
    },
    fit, fixed, grow,
    id::Id,
    layout::LayoutDirection::TopToBottom,
    Ui,
};

use crate::theme::Theme;

pub(crate) fn button(
    ui: &mut Ui,
    theme: &Theme,
    label: impl AsRef<str>,
    id: impl Into<Id>,
) -> bool {
    let mut clicked = false;
    ui.element()
        .width(fit!())
        .height(fixed!(32.0))
        .corner_radius(6.0)
        .id(id)
        .children(|ui| {
            let bg = if !ui.hovered() {
                theme.accent.1
            } else if ui.pressed() {
                theme.accent.0
            } else {
                theme.accent.2
            };

            clicked = ui.just_pressed();

            ui.element()
                .width(fit!())
                .height(grow!())
                .background_color(bg)
                .corner_radius(6.0)
                .layout(|l| l.padding((0, 16, 0, 16)).align(CenterX, CenterY))
                .children(|ui| {
                    ui.text(label.as_ref(), |t| t.font_size(14).color(theme.text_dark));
                });
        });

    clicked
}

pub(crate) fn dropdown<E: strum::IntoEnumIterator + Display + Clone>(
    ui: &mut Ui,
    theme: &Theme,
    id: impl Into<Id>,
    current_value: &Option<E>,
    open: &bool,
) -> (Option<E>, bool) {
    let mut current_value = current_value.clone();
    let mut open = *open;
    ui.element()
        .width(fit!())
        .height(fixed!(32.0))
        .layout(|l| l.align(Left, CenterY).padding(8))
        .corner_radius(6.0)
        .id(id)
        .background_color(theme.surface.2)
        .children(|ui| {
            if let Some(value) = &current_value {
                ui.text(&value.to_string(), |t| {
                    t.color(theme.text_primary).font_size(14)
                });
            } else {
                ui.text("-", |t| t.color(theme.text_secondary).font_size(14));
            }

            if ui.just_pressed() {
                open = !open;
                info!("Toggled dropdown");
            }

            if !open || ui.focused() {
                return;
            }

            ui.element()
                .floating(|f| f.attach_parent().anchor((Left, Top), (Left, Bottom)))
                .width(fit!())
                .height(fit!())
                .background_color(theme.surface.1)
                .layout(|l| l.direction(TopToBottom).align(Left, CenterY))
                .children(|ui| {
                    for variant in E::iter() {
                        ui.element()
                            .width(grow!())
                            .height(fixed!(32.0))
                            .children(|ui| {
                                let bg = if ui.hovered() {
                                    theme.surface.2
                                } else {
                                    theme.surface.1
                                };

                                ui.element()
                                    .width(grow!())
                                    .height(grow!())
                                    .layout(|l| l.align(Left, CenterY).padding(8))
                                    .background_color(bg)
                                    .children(|ui| {
                                        ui.text(&variant.to_string(), |t| {
                                            t.font_size(14).color(theme.text_primary)
                                        })
                                    });

                                if ui.just_pressed() {
                                    open = false;
                                    current_value = Some(variant);
                                }
                            });
                    }
                });
        });

    (current_value, open)
}

pub(crate) fn text_input<F>(
    ui: &mut Ui,
    theme: &Theme,
    id: impl Into<Id> + Clone,
    placeholder: &str,
    validation: F,
) -> String
where
    F: FnOnce(&str) -> Result<(), anyhow::Error>,
{
    let text = ui.get_text_value(id.clone()).trim().to_string();
    let text_result = validation(&text);
        
    ui
        .element()
        .id(id)
        .width(grow!())
        .height(fixed!(20.0))
        .corner_radius(12.0)
        .background_color(theme.surface.2)
        .text_input(|ti| {
            let col = if text_result.is_ok() {
                theme.text_primary
            } else {
                theme.text_error
            };
            ti.font_size(16)
                .placeholder_color(theme.text_secondary)
                .text_color(col)
                .placeholder(placeholder)
        })
        .empty();

    text
}
