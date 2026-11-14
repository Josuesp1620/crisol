/// Visual weight of a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Secondary,
    /// The main action of the form.
    Primary,
    /// No border or fill: for actions inside toolbars.
    Ghost,
}
