use log::info;
use ply_engine::prelude::*;

mod elements;
mod qr;

struct Theme {
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_dark: Color,
    pub surface: (Color, Color, Color),
    pub accent: (Color, Color, Color),
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
    let theme = Theme {
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
    };

    let mut texture = Texture2D::empty();
    let mut current_url = String::new();
    let mut dropdown_open = false;
    let mut model = qr::QRModel { data: "".to_string(), size: 32, dot_type: qr::DotType::Square, hex_color: "#000000".to_string()};

    loop {
        clear_background(theme.surface.0.into());
        if ply.get_text_value("url") != current_url {
            current_url = ply.get_text_value("url").to_string();

            if !current_url.is_empty() {
                let desc = qr::QRModel {
                    data: current_url.clone(),
                    size: 300,
                    dot_type: qr::DotType::Square,
                    hex_color: "#000000".to_string(),
                };
                let image = qr::render_qr(desc).unwrap();
                texture = Texture2D::from_image(&image);
            }
        }

        if ply.is_just_pressed("save_button") {
            let desc = qr::QRModel {
                data: current_url.clone(),
                size: 300,
                dot_type: qr::DotType::Square,
                hex_color: "#000000".to_string(),
            };
            qr::save_qr(desc).unwrap();
        }

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
                        ui.element()
                            .id("url")
                            .width(grow!())
                            .height(fixed!(20.0))
                            .corner_radius(12.0)
                            .background_color(theme.surface.2)
                            .text_input(|ti| {
                                ti.font_size(16)
                                    .placeholder_color(theme.text_secondary)
                                    .text_color(theme.text_primary)
                                    .placeholder("url")
                            })
                            .empty();
                    });
                ui.element().height(fit!()).width(grow!()).children(|ui| {
                    let (dt, open) = elements::dropdown(
                        ui,
                        &theme,
                        "dot_type",
                        &Some(model.dot_type),
                        &dropdown_open,
                    );
                    dropdown_open = open;
                    model.dot_type = dt.unwrap_or_default()
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
                            .image(texture.clone())
                            .empty();
                    });
                ui.element()
                    .width(grow!())
                    .height(fit!())
                    .layout(|l| l.align(Right, CenterY))
                    .children(|ui| elements::button(ui, &theme, "save", "save_button"));
            });

        ui.show(|_| {}).await;

        next_frame().await;
    }
}
