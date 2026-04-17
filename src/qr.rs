use ply_engine::prelude::{Image, ImageFormat};
use qr_code_styling::{DotsOptions, QRCodeStyling};

pub(crate) struct QRCodeDesc {
    pub data: String,
    pub size: u32,
    pub dot_type: qr_code_styling::DotType,
    pub hex_color: String,
}

pub(crate) fn render_qr(qr_desc: QRCodeDesc) -> anyhow::Result<Image> {
    let qr = QRCodeStyling::builder()
        .data(qr_desc.data)
        .size(qr_desc.size)
        .dots_options(
            DotsOptions::new(qr_desc.dot_type)
                .with_color(qr_code_styling::Color::from_hex(&qr_desc.hex_color).unwrap()),
        )
        .build()
        .unwrap();

    let image_data = qr.render(qr_code_styling::OutputFormat::Png)?;

    Ok(Image::from_file_with_format(
        &image_data,
        Some(ImageFormat::Png),
    )?)
}
