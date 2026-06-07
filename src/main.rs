use anyhow::Context;
use log::info;
use ply_engine::prelude::*;

mod elements;
mod qr;
mod theme;

struct View {
    pub model: qr::QRModel,
    pub qr_texture: Texture2D,
    pub dots_dropdown: bool,
    pub save_button: bool,
}

impl View {
    fn new() -> Self {
        Self {
            model: qr::QRModel {
                data: "".to_string(),
                size: 300,
                dot_type: qr::DotType::Square,
                dot_color: "#000000".to_string(),
            },
            qr_texture: Texture2D::empty(),
            dots_dropdown: false,
            save_button: false,
        }
    }
}

fn window_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "QR Code Desktop".to_owned(),
            window_width: 500,
            window_height: 600,
            high_dpi: true,
            sample_count: 4,
            platform: miniquad::conf::Platform {
                webgl_version: miniquad::conf::WebGLVersion::WebGL2,
                ..Default::default()
            },
            ..Default::default()
        },
        draw_call_vertex_capacity: 100000,
        draw_call_index_capacity: 100000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    env_logger::init();
    info!("Qr code desktop");

    static DEFAULT_FONT: FontAsset = FontAsset::Path("assets/fonts/lexend.ttf");
    let mut ply = Ply::<()>::new(&DEFAULT_FONT).await;
    let theme = theme::Theme {
        surface: (
            Color::from(0x121212),
            Color::from(0x252525),
            Color::from(0x393939),
        ),
        accent: (
            Color::from(0x91f170),
            Color::from(0x9df381),
            Color::from(0xa9f591),
        ),
        text_primary: Color::from(0xF1F5F9),
        text_secondary: Color::from(0x94A3B8),
        text_dark: Color::from(0x000000),
        text_error: Color::rgb(255.0, 0.0, 0.0),
    };

    let mut view = View::new();

    loop {
        clear_background(theme.surface.0.into());

        let mut next_model = view.model.clone();

        let mut ui = ply.begin();

        ui.element()
            .width(grow!())
            .height(grow!())
            .layout(|l| l.direction(TopToBottom).padding(8).align(Left, Top))
            .id("root")
            .children(|ui| {
                ui.element()
                    .height(fit!())
                    .width(grow!())
                    .layout(|l| l.direction(LeftToRight).padding(8).align(Left, CenterY))
                    .children(|ui| {
                        ui.text("Url: ", |t| t.color(theme.text_primary).font_size(16));
                        next_model.data =
                            elements::text_input(ui, &theme, "url", "url", |_| Ok(()));
                    });
                // Dots options
                ui.element()
                    .height(fit!())
                    .width(grow!())
                    .layout(|l| l.direction(LeftToRight).padding(8).align(Left, CenterY))
                    .children(|ui| {
                        ui.text("Dots: ", |t| t.color(theme.text_primary).font_size(16));
                        let (dt, open) = elements::dropdown(
                            ui,
                            &theme,
                            "dot_type",
                            &Some(view.model.dot_type),
                            &view.dots_dropdown,
                        );
                        view.dots_dropdown = open;
                        next_model.dot_type = dt.unwrap_or_default();

                        ui.text("Color: ", |t| t.color(theme.text_primary).font_size(16));
                        let col = elements::text_input(ui, &theme, "color_input", "color", |s| {
                            qr_code_styling::Color::from_hex(s)
                                .map(|_| ())
                                .with_context(|| "Incorrect color input by user")
                        });

                        next_model.dot_color = col;
                    });
                ui.element()
                    .height(grow!())
                    .width(grow!())
                    .layout(|l| l.align(CenterX, CenterY))
                    .background_color(theme.surface.1)
                    .corner_radius(12.0)
                    .children(|ui| {
                        ui.element()
                            .width(fixed!(300.0))
                            .height(fixed!(300.0))
                            .image(view.qr_texture.clone())
                            .empty();
                    });
                ui.element()
                    .width(grow!())
                    .height(fit!())
                    .layout(|l| l.align(Right, CenterY))
                    .children(|ui| {
                        view.save_button = elements::button(ui, &theme, "save", "save_button");
                    });
            });

        if view.model != next_model {
            view.model = next_model;

            if !view.model.data.is_empty() {
                match qr::render_qr(&view.model) {
                    Ok(image) => view.qr_texture = Texture2D::from_image(&image),
                    Err(e) => qr::show_error(format!("Failed to render the qr code! Reason:{e}")),
                }
            }
        }

        if view.save_button {
            qr::save_qr(&view.model).unwrap_or_else(|e| qr::show_error(format!("Failed to save the qr code! Reason:{e}")));
        }

        ui.show(|_| {}).await;
        next_frame().await;
    }
}
