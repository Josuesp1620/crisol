use crisol_model::layout::SizeMode;
use crisol_ui::layout::Length;

/// Turns the model's size modes into component lengths.
pub struct LengthConverter;

impl LengthConverter {
    #[allow(
        clippy::cast_precision_loss,
        reason = "logical pixels are small integers"
    )]
    pub fn length(mode: SizeMode) -> Length {
        match mode {
            SizeMode::Fixed(pixels) => Length::Fixed(pixels as f32),
            SizeMode::Hug => Length::Hug,
            SizeMode::Fill => Length::Fill,
        }
    }
}
