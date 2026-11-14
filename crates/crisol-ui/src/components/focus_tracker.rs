use gpui::{Context, FocusHandle, Subscription, Window};

/// Raises focus events for a control. GPUI needs a window to watch focus, which a control
/// only has once it renders, so the watch starts on the first render.
#[derive(Default)]
pub struct FocusTracker {
    subscriptions: Vec<Subscription>,
}

impl FocusTracker {
    pub fn watch<T: 'static>(
        &mut self,
        handle: &FocusHandle,
        window: &mut Window,
        cx: &mut Context<T>,
        on_gained: impl Fn(&mut T, &mut Context<T>) + 'static,
        on_lost: impl Fn(&mut T, &mut Context<T>) + 'static,
    ) {
        if !self.subscriptions.is_empty() {
            return;
        }
        self.subscriptions
            .push(cx.on_focus(handle, window, move |control, _, cx| on_gained(control, cx)));
        self.subscriptions
            .push(cx.on_blur(handle, window, move |control, _, cx| on_lost(control, cx)));
    }
}
