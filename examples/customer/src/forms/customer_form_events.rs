use gpui::{Context, Window};

use super::customer_form::{CustomerForm, CustomerFormEvents};

impl CustomerFormEvents for CustomerForm {
    fn kind_changed(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let kind = self
            .cmb_kind
            .read(cx)
            .selected_text()
            .unwrap_or_default()
            .to_owned();
        println!("Kind: {kind}");
    }

    fn cancel(&mut self, window: &mut Window, _cx: &mut Context<Self>) {
        window.remove_window();
    }

    fn save(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let tax_id = self.txt_tax_id.read(cx).value().to_owned();
        let business_name = self.txt_business_name.read(cx).value().to_owned();
        let active = self.chk_active.read(cx).is_checked();
        println!("Saved {business_name} ({tax_id}), active: {active}");
    }
}
