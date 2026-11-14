use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};

/// Icons bundled into the binary and served to GPUI by path.
pub struct CrisolAssets;

impl CrisolAssets {
    fn bytes(path: &str) -> Option<&'static [u8]> {
        let bytes: &'static [u8] = match path {
            "icons/al-bottom.svg" => include_bytes!("../../assets/icons/al-bottom.svg"),
            "icons/al-center.svg" => include_bytes!("../../assets/icons/al-center.svg"),
            "icons/al-left.svg" => include_bytes!("../../assets/icons/al-left.svg"),
            "icons/al-right.svg" => include_bytes!("../../assets/icons/al-right.svg"),
            "icons/al-top.svg" => include_bytes!("../../assets/icons/al-top.svg"),
            "icons/az.svg" => include_bytes!("../../assets/icons/az.svg"),
            "icons/bolt.svg" => include_bytes!("../../assets/icons/bolt.svg"),
            "icons/box.svg" => include_bytes!("../../assets/icons/box.svg"),
            "icons/button.svg" => include_bytes!("../../assets/icons/button.svg"),
            "icons/cats.svg" => include_bytes!("../../assets/icons/cats.svg"),
            "icons/check.svg" => include_bytes!("../../assets/icons/check.svg"),
            "icons/checkbox.svg" => include_bytes!("../../assets/icons/checkbox.svg"),
            "icons/code.svg" => include_bytes!("../../assets/icons/code.svg"),
            "icons/combo.svg" => include_bytes!("../../assets/icons/combo.svg"),
            "icons/dot.svg" => include_bytes!("../../assets/icons/dot.svg"),
            "icons/down.svg" => include_bytes!("../../assets/icons/down.svg"),
            "icons/err.svg" => include_bytes!("../../assets/icons/err.svg"),
            "icons/eye.svg" => include_bytes!("../../assets/icons/eye.svg"),
            "icons/file.svg" => include_bytes!("../../assets/icons/file.svg"),
            "icons/folder.svg" => include_bytes!("../../assets/icons/folder.svg"),
            "icons/form.svg" => include_bytes!("../../assets/icons/form.svg"),
            "icons/globe.svg" => include_bytes!("../../assets/icons/globe.svg"),
            "icons/grid.svg" => include_bytes!("../../assets/icons/grid.svg"),
            "icons/group.svg" => include_bytes!("../../assets/icons/group.svg"),
            "icons/history.svg" => include_bytes!("../../assets/icons/history.svg"),
            "icons/hspace.svg" => include_bytes!("../../assets/icons/hspace.svg"),
            "icons/info.svg" => include_bytes!("../../assets/icons/info.svg"),
            "icons/label.svg" => include_bytes!("../../assets/icons/label.svg"),
            "icons/lock.svg" => include_bytes!("../../assets/icons/lock.svg"),
            "icons/max.svg" => include_bytes!("../../assets/icons/max.svg"),
            "icons/min.svg" => include_bytes!("../../assets/icons/min.svg"),
            "icons/monitor.svg" => include_bytes!("../../assets/icons/monitor.svg"),
            "icons/outline.svg" => include_bytes!("../../assets/icons/outline.svg"),
            "icons/panel.svg" => include_bytes!("../../assets/icons/panel.svg"),
            "icons/phone.svg" => include_bytes!("../../assets/icons/phone.svg"),
            "icons/pin.svg" => include_bytes!("../../assets/icons/pin.svg"),
            "icons/play.svg" => include_bytes!("../../assets/icons/play.svg"),
            "icons/plus.svg" => include_bytes!("../../assets/icons/plus.svg"),
            "icons/pointer.svg" => include_bytes!("../../assets/icons/pointer.svg"),
            "icons/props.svg" => include_bytes!("../../assets/icons/props.svg"),
            "icons/radio.svg" => include_bytes!("../../assets/icons/radio.svg"),
            "icons/redo.svg" => include_bytes!("../../assets/icons/redo.svg"),
            "icons/right.svg" => include_bytes!("../../assets/icons/right.svg"),
            "icons/rust.svg" => include_bytes!("../../assets/icons/rust.svg"),
            "icons/same.svg" => include_bytes!("../../assets/icons/same.svg"),
            "icons/save.svg" => include_bytes!("../../assets/icons/save.svg"),
            "icons/search.svg" => include_bytes!("../../assets/icons/search.svg"),
            "icons/send.svg" => include_bytes!("../../assets/icons/send.svg"),
            "icons/spark.svg" => include_bytes!("../../assets/icons/spark.svg"),
            "icons/stack.svg" => include_bytes!("../../assets/icons/stack.svg"),
            "icons/table.svg" => include_bytes!("../../assets/icons/table.svg"),
            "icons/taborder.svg" => include_bytes!("../../assets/icons/taborder.svg"),
            "icons/tabs.svg" => include_bytes!("../../assets/icons/tabs.svg"),
            "icons/templ.svg" => include_bytes!("../../assets/icons/templ.svg"),
            "icons/term.svg" => include_bytes!("../../assets/icons/term.svg"),
            "icons/textbox.svg" => include_bytes!("../../assets/icons/textbox.svg"),
            "icons/undo.svg" => include_bytes!("../../assets/icons/undo.svg"),
            "icons/warn.svg" => include_bytes!("../../assets/icons/warn.svg"),
            "icons/x.svg" => include_bytes!("../../assets/icons/x.svg"),
            _ => return None,
        };
        Some(bytes)
    }
}

impl AssetSource for CrisolAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(Self::bytes(path).map(Cow::Borrowed))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut found = Vec::new();
        for icon in crate::icon::IconName::ALL {
            if icon.path().starts_with(path) {
                found.push(icon.path().into());
            }
        }
        Ok(found)
    }
}
