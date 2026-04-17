use log::info;
use ply_engine::engine;
use ply_engine::prelude::*;
use qr_code_styling::{DotsOptions, QRCodeStyling};
use rfd::FileDialog;

struct Theme {
    pub backgorund: Color,
    pub surface: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub accent: Color,
}

fn window_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "Hello Ply!".to_owned(),
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

fn render_qr(data: impl Into<String>) -> anyhow::Result<Image> {
    let qr = QRCodeStyling::builder()
        .data(data)
        .size(300)
        .dots_options(
            DotsOptions::new(qr_code_styling::DotType::Square)
                .with_color(qr_code_styling::Color::from_hex("#000000").unwrap()),
        )
        .build()
        .unwrap();

    let image_data = qr.render(qr_code_styling::OutputFormat::Png)?;

    Ok(Image::from_file_with_format(
        &image_data,
        Some(ImageFormat::Png),
    )?)
}

fn button(
    ui: &mut Ui,
    theme: &Theme,
    label: impl AsRef<str>,
    on_click: impl FnMut(Id, engine::PointerData) + 'static,
) {
    ui.element()
        .width(fit!())
        .height(fixed!(32.0))
        .corner_radius(6.0)
        .on_press(on_click)
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

#[macroquad::main(window_conf)]
async fn main() {
    static DEFAULT_FONT: FontAsset = FontAsset::Path("assets/fonts/lexend.ttf");
    let mut ply = Ply::<()>::new(&DEFAULT_FONT).await;
    let theme = Theme {
        backgorund: Color::from(0x0F172A),
        surface: Color::from(0x1E293B),
        accent: Color::from(0x38BDF8),
        text_primary: Color::from(0xF1F5F9),
        text_secondary: Color::from(0x94A3B8),
    };

    let mut texture = Texture2D::empty();
    let mut current_url = String::new();

    loop {
        clear_background(theme.backgorund.into());
        if ply.get_text_value("url") != current_url {
            current_url = ply.get_text_value("url").to_string();

            let image = render_qr(&current_url).unwrap();
            texture = Texture2D::from_image(&image)
        }

        if is_key_pressed(KeyCode::F5) {
            if let Some(path) = FileDialog::new()
                .add_filter("png image", &["png"])
                .set_directory("./")
                .save_file()
            {
                let qr = QRCodeStyling::builder()
                    .data(ply.get_text_value("url"))
                    .size(300)
                    .dots_options(
                        DotsOptions::new(qr_code_styling::DotType::Square)
                            .with_color(qr_code_styling::Color::from_hex("#000000").unwrap()),
                    )
                    .build()
                    .unwrap();

                qr.save(path, qr_code_styling::OutputFormat::Png).unwrap();
            }
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
                            .background_color(theme.surface)
                            .text_input(|ti| {
                                ti.font_size(16)
                                    .placeholder_color(theme.text_secondary)
                                    .text_color(theme.text_primary)
                                    .placeholder("url")
                            })
                            .empty();
                    });
                ui.element()
                    .height(grow!())
                    .width(grow!())
                    .layout(|l| l.align(CenterX, CenterY))
                    .background_color(theme.surface)
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
                    .children(|ui| {
                        button(ui, &theme, "save", |_, _| {info!("going to save")})
                    });
            });

        ui.show(|_| {}).await;

        next_frame().await;
    }
}
