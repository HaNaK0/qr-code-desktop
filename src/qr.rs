use anyhow::anyhow;
use log::warn;
use ply_engine::prelude::{Image, ImageFormat};
use qr_code_styling::{self, DotsOptions, QRCodeStyling};
use rfd::FileDialog;
use strum::{Display, EnumIter};

#[derive(PartialEq, Eq, Clone)]
pub(crate) struct QRModel {
    pub data: String,
    pub size: u32,
    pub dot_type: DotType,
    pub hex_color: String,
}

#[derive(Debug,EnumIter,Clone, Copy, Display, Default, PartialEq, Eq)]
pub(crate) enum DotType {
    #[default]
    Square,
    Dots,
    Rounded,
    Classy,
    ClassyRounded,
    ExtraRounded,
}

impl From<qr_code_styling::DotType> for DotType {
    fn from(value: qr_code_styling::DotType) -> Self {
        match value {
            qr_code_styling::DotType::Square => Self::Square,
            qr_code_styling::DotType::Dots => Self::Dots,
            qr_code_styling::DotType::Rounded => Self::Rounded,
            qr_code_styling::DotType::Classy => Self::Classy,
            qr_code_styling::DotType::ClassyRounded => Self::ClassyRounded,
            qr_code_styling::DotType::ExtraRounded => Self::ExtraRounded,
        }
    }
}

impl From<DotType> for qr_code_styling::DotType {
    fn from(val: DotType) -> Self {
        match val {
            DotType::Square => qr_code_styling::DotType::Square,
            DotType::Dots => qr_code_styling::DotType::Dots,
            DotType::Rounded => qr_code_styling::DotType::Rounded,
            DotType::Classy => qr_code_styling::DotType::Classy,
            DotType::ClassyRounded => qr_code_styling::DotType::ClassyRounded,
            DotType::ExtraRounded => qr_code_styling::DotType::ExtraRounded,
        }
    }
}

impl QRModel {
    fn create_qr_code_style(&self) -> anyhow::Result<QRCodeStyling> {
        Ok(QRCodeStyling::builder()
            .data(self.data.clone())
            .size(self.size)
            .dots_options(
                DotsOptions::new(self.dot_type.into())
                    .with_color(qr_code_styling::Color::from_hex(&self.hex_color)?),
            )
            .build()?)
    }
}

pub(crate) fn render_qr(qr_desc: &QRModel) -> anyhow::Result<Image> {
    let qr = qr_desc.create_qr_code_style()?;
    let image_data = qr.render(qr_code_styling::OutputFormat::Png)?;

    Ok(Image::from_file_with_format(
        &image_data,
        Some(ImageFormat::Png),
    )?)
}

pub(crate) fn save_qr(qr_desc: &QRModel) -> anyhow::Result<()> {
    if qr_desc.data.is_empty() {
        warn!("Data is empty can't save an empty qr code")
    }
    let qr = qr_desc.create_qr_code_style()?;

    if let Some(path) = FileDialog::new()
        .add_filter("png image", &["png"])
        .add_filter("jpg image", &["jpg", "jpeg"])
        .add_filter("svg Vector graphics", &["svg"])
        .add_filter("Webp image", &["webp"])
        .add_filter("PDF document", &["pdf"])
        .set_directory("./")
        .save_file()
    {
        let output_format = match path.extension().unwrap().to_str().unwrap() {
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
