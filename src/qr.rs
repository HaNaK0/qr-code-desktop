use anyhow::anyhow;
use log::{error, warn};
use ply_engine::prelude::{Image, ImageFormat};
use qr_code_styling::{self, CornersDotOptions, DotsOptions, QRCodeStyling};
use rfd::{FileDialog, MessageDialog};
use strum::{Display, EnumIter};

#[derive(PartialEq, Eq, Clone)]
pub(crate) struct QRModel {
    pub data: String,
    pub size: u32,
    pub dot_type: DotType,
    pub dot_color: String,
    pub corner_dot_type: CornerDotType,
    pub corner_dot_color: String,
}

impl Default for QRModel {
    fn default() -> Self {
        Self {
            data: Default::default(),
            size: 300,
            dot_type: Default::default(),
            dot_color: "#000".to_string(),
            corner_dot_type: Default::default(),
            corner_dot_color: "#000".to_string(),
        }
    }
}

#[derive(Debug, EnumIter, Clone, Copy, Display, Default, PartialEq, Eq)]
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

#[derive(Debug, EnumIter, Clone, Copy, Display, Default, PartialEq, Eq)]
pub(crate) enum CornerDotType {
    #[default]
    Dot,
    Square,
}

impl From<qr_code_styling::CornerDotType> for CornerDotType {
    fn from(value: qr_code_styling::CornerDotType) -> Self {
        match value {
            qr_code_styling::CornerDotType::Dot => Self::Dot,
            qr_code_styling::CornerDotType::Square => Self::Square,
        }
    }
}

impl From<CornerDotType> for qr_code_styling::CornerDotType {
    fn from(value: CornerDotType) -> Self {
        match value {
            CornerDotType::Dot => qr_code_styling::CornerDotType::Dot,
            CornerDotType::Square => qr_code_styling::CornerDotType::Square,
        }
    }
}

impl QRModel {
    fn create_qr_code_style(&self) -> anyhow::Result<QRCodeStyling> {
        let dot_color = match qr_code_styling::Color::from_hex(&self.dot_color) {
            Ok(c) => c,
            Err(e) => {
                warn!("incorrect color string. Details: {e}");
                qr_code_styling::Color::BLACK
            }
        };

        Ok(QRCodeStyling::builder()
            .data(self.data.clone())
            .size(self.size)
            .dots_options(DotsOptions::new(self.dot_type.into()).with_color(dot_color))
            .corners_dot_options(
                CornersDotOptions::new(self.corner_dot_type.into()).with_color(
                    qr_code_styling::Color::from_hex(&self.corner_dot_color).unwrap_or_else(|e| {
                        warn!("incorect color string in corner dot color! Reason:{e}");
                        qr_code_styling::Color::BLACK
                    }),
                ),
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
        let output_format = match path
            .extension()
            .ok_or(anyhow!("failed to get file extension"))?
            .to_str()
            .ok_or(anyhow!("failed to convert extension to string"))?
        {
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

pub(crate) fn show_error(error: String) {
    error!("displaying error with reason:{error}");
    MessageDialog::new()
        .set_level(rfd::MessageLevel::Error)
        .set_title("Error!")
        .set_description(error)
        .set_buttons(rfd::MessageButtons::Ok)
        .show();
}
