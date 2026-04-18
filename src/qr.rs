use anyhow::anyhow;
use ply_engine::prelude::{Image, ImageFormat};
use qr_code_styling::{DotsOptions, QRCodeStyling};
use rfd::FileDialog;

pub(crate) struct QRCodeDesc {
    pub data: String,
    pub size: u32,
    pub dot_type: qr_code_styling::DotType,
    pub hex_color: String,
}

impl QRCodeDesc{
   fn into_qr_code_style(self) -> anyhow::Result<QRCodeStyling> {
       Ok(QRCodeStyling::builder()
           .data(self.data)
           .size(self.size)
           .dots_options(
              DotsOptions::new(self.dot_type).with_color(qr_code_styling::Color::from_hex(&self.hex_color)?)
           )
           .build()?)
   }
}

pub(crate) fn render_qr(qr_desc: QRCodeDesc) -> anyhow::Result<Image> {
    let qr = qr_desc.into_qr_code_style()?;
    let image_data = qr.render(qr_code_styling::OutputFormat::Png)?;

    Ok(Image::from_file_with_format(
        &image_data,
        Some(ImageFormat::Png),
    )?)
}

pub(crate) fn save_qr(qr_desc: QRCodeDesc) -> anyhow::Result<()> {
    let qr = qr_desc.into_qr_code_style()?;

    if let Some(path) = FileDialog::new()
        .add_filter("png image", &["png"])
        .add_filter("jpg image", &["jpg", "jpeg"])
        .add_filter("svg Vector graphics", &["svg"])
        .add_filter("Webp image", &["webp"])
        .add_filter("PDF document", &["pdf"])
        .set_directory("./")
        .save_file()
    {
        let output_format = match path.extension().unwrap().to_str().unwrap(){
            "png" => Ok(qr_code_styling::OutputFormat::Png),
            "jpg" | "jpeg" => Ok(qr_code_styling::OutputFormat::Jpeg),
            "svg" => Ok(qr_code_styling::OutputFormat::Svg),
            "webp" => Ok(qr_code_styling::OutputFormat::WebP),
            "pdf" => Ok(qr_code_styling::OutputFormat::Pdf),
            _ => Err(anyhow!("Unsuported file format")),
        }?;

        qr.save(path, output_format)?;
    }
    Ok(())
}
