use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::{Theme, ThemeMode};
use gpui::{
    AnyElement, Context, FontWeight, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px, rgba,
};

use crate::chrome::{Segment, SegmentedControl};
use crate::designer_texts::DesignerTexts;
use crate::project::ProjectTemplate;
use crate::workspace::DesignerWorkspace;

/// The New project dialog: template, name, location, platforms and theme.
pub struct NewProjectDialog<'workspace> {
    workspace: &'workspace DesignerWorkspace,
}

impl<'workspace> NewProjectDialog<'workspace> {
    pub fn new(workspace: &'workspace DesignerWorkspace) -> Self {
        Self { workspace }
    }

    pub fn render(self, _window: &mut Window, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        if self.workspace.new_project().is_none() {
            return gpui::Empty.into_any_element();
        }
        let dialog = div()
            .absolute()
            .top(px(90.))
            .w(px(780.))
            .flex()
            .flex_col()
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.line)
            .rounded(px(12.))
            .shadow_lg()
            .occlude()
            .child(Self::title(&theme, cx))
            .child(
                div()
                    .flex()
                    .child(self.templates(&theme, cx))
                    .child(self.form(&theme, cx)),
            )
            .child(Self::footer(&theme, cx));
        div()
            .absolute()
            .inset_0()
            .flex()
            .justify_center()
            .bg(rgba(0x0A0E_1473))
            .occlude()
            .child(dialog)
            .into_any_element()
    }

    fn title(theme: &Theme, cx: &mut Context<DesignerWorkspace>) -> impl IntoElement {
        div()
            .flex()
            .justify_between()
            .items_center()
            .px(px(18.))
            .py(px(14.))
            .border_b_1()
            .border_color(theme.colors.line)
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(14.))
                    .child(DesignerTexts::NEW_PROJECT),
            )
            .child(
                div()
                    .id("close-dialog")
                    .cursor_pointer()
                    .on_click(cx.listener(|workspace, _, _, cx| workspace.close_new_project(cx)))
                    .child(Icon::new(IconName::X).color(theme.colors.muted)),
            )
    }

    fn templates(&self, theme: &Theme, cx: &mut Context<DesignerWorkspace>) -> impl IntoElement {
        let chosen_template = self.workspace.new_project().map(|state| state.template);
        let mut templates = div()
            .flex()
            .flex_col()
            .gap(px(4.))
            .w(px(270.))
            .p(px(12.))
            .border_r_1()
            .border_color(theme.colors.line)
            .child(
                div()
                    .text_size(px(11.))
                    .text_color(theme.colors.muted)
                    .child(DesignerTexts::TEMPLATES),
            );
        for template in ProjectTemplate::ALL {
            let chosen = Some(template) == chosen_template;
            let color = if chosen {
                theme.colors.selection
            } else {
                theme.colors.muted
            };
            templates = templates.child(
                div()
                    .id(template.label())
                    .flex()
                    .gap(px(10.))
                    .px(px(10.))
                    .py(px(9.))
                    .rounded(px(7.))
                    .cursor_pointer()
                    .when(chosen, |item| {
                        item.bg(theme.colors.selection_soft)
                            .border_1()
                            .border_color(theme.colors.selection)
                    })
                    .when(!chosen, |item| {
                        item.hover(|style| style.bg(theme.colors.sunken))
                    })
                    .on_click(cx.listener(move |workspace, _, _, cx| {
                        workspace.choose_template(template, cx);
                    }))
                    .child(Icon::new(template.icon()).color(color))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(template.label()),
                            )
                            .child(
                                div()
                                    .text_size(px(11.))
                                    .text_color(theme.colors.muted)
                                    .child(template.description()),
                            ),
                    ),
            );
        }
        templates
    }

    fn form(&self, theme: &Theme, cx: &mut Context<DesignerWorkspace>) -> impl IntoElement {
        let Some(state) = self.workspace.new_project() else {
            return div();
        };
        let location = div()
            .flex()
            .gap(px(6.))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .child(state.location.clone()),
            )
            .child(
                div()
                    .id("browse")
                    .px(px(10.))
                    .py(px(6.))
                    .rounded(theme.metrics.control_radius)
                    .border_1()
                    .border_color(theme.colors.line)
                    .cursor_pointer()
                    .on_click(cx.listener(|workspace, _, window, cx| {
                        workspace.browse_location(window, cx);
                    }))
                    .child(DesignerTexts::BROWSE),
            );
        div()
            .flex()
            .flex_col()
            .flex_1()
            .gap(px(14.))
            .p(px(18.))
            .child(Self::field(
                theme,
                DesignerTexts::PROJECT_NAME,
                state.name.clone().into_any_element(),
            ))
            .child(Self::field(
                theme,
                DesignerTexts::PROJECT_LOCATION,
                location.into_any_element(),
            ))
            .child(Self::field(
                theme,
                DesignerTexts::PLATFORMS,
                Self::platforms(theme).into_any_element(),
            ))
            .child(Self::field(
                theme,
                DesignerTexts::THEME,
                self.themes(cx).into_any_element(),
            ))
            .when_some(state.error.clone(), |form, error| {
                form.child(div().text_color(theme.colors.danger).child(error))
            })
    }

    fn field(theme: &Theme, label: &'static str, control: AnyElement) -> impl IntoElement {
        div()
            .flex()
            .gap(px(12.))
            .child(
                div()
                    .w(px(96.))
                    .flex_none()
                    .pt(px(6.))
                    .text_color(theme.colors.muted)
                    .child(label),
            )
            .child(div().flex().flex_col().flex_1().child(control))
    }

    fn platforms(theme: &Theme) -> impl IntoElement {
        let mut grid = div().flex().flex_wrap().gap(px(6.));
        for (icon, name, detail, enabled) in [
            (
                IconName::Monitor,
                DesignerTexts::PLATFORM_DESKTOP,
                DesignerTexts::PLATFORM_DESKTOP_DETAIL,
                true,
            ),
            (
                IconName::Globe,
                DesignerTexts::PROFILE_WEB,
                DesignerTexts::PLATFORM_WEB_DETAIL,
                false,
            ),
            (
                IconName::Phone,
                "Android",
                DesignerTexts::PLATFORM_ANDROID_DETAIL,
                false,
            ),
            (
                IconName::Phone,
                "iOS",
                DesignerTexts::PLATFORM_IOS_DETAIL,
                false,
            ),
        ] {
            grid = grid.child(
                div()
                    .w(px(250.))
                    .flex()
                    .gap(px(7.))
                    .p(px(8.))
                    .rounded(px(7.))
                    .border_1()
                    .border_color(theme.colors.line)
                    .when(!enabled, |card| card.opacity(0.55))
                    .child(
                        div()
                            .size(px(16.))
                            .flex_none()
                            .rounded(px(4.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .when(enabled, |mark| {
                                mark.bg(theme.colors.muted).child(
                                    Icon::new(IconName::Check)
                                        .size(px(12.))
                                        .color(theme.colors.on_accent),
                                )
                            })
                            .when(!enabled, |mark| {
                                mark.border(px(1.5)).border_color(theme.colors.muted)
                            }),
                    )
                    .child(Icon::new(icon).color(theme.colors.muted))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().font_weight(FontWeight::SEMIBOLD).child(name))
                            .child(
                                div()
                                    .text_size(px(10.5))
                                    .text_color(theme.colors.muted)
                                    .child(detail),
                            ),
                    ),
            );
        }
        grid
    }

    fn themes(&self, cx: &mut Context<DesignerWorkspace>) -> impl IntoElement {
        let chosen = self
            .workspace
            .new_project()
            .map_or(ThemeMode::Light, |state| state.theme);
        div()
            .flex()
            .gap(px(6.))
            .child(
                div()
                    .id("theme-light")
                    .cursor_pointer()
                    .on_click(cx.listener(|workspace, _, _, cx| {
                        workspace.choose_project_theme(ThemeMode::Light, cx);
                    }))
                    .child(SegmentedControl::new(vec![Segment {
                        icon: IconName::Monitor,
                        label: DesignerTexts::THEME_LIGHT.into(),
                        active: chosen == ThemeMode::Light,
                        available: true,
                    }])),
            )
            .child(
                div()
                    .id("theme-dark")
                    .cursor_pointer()
                    .on_click(cx.listener(|workspace, _, _, cx| {
                        workspace.choose_project_theme(ThemeMode::Dark, cx);
                    }))
                    .child(SegmentedControl::new(vec![Segment {
                        icon: IconName::Monitor,
                        label: DesignerTexts::THEME_DARK.into(),
                        active: chosen == ThemeMode::Dark,
                        available: true,
                    }])),
            )
    }

    fn footer(theme: &Theme, cx: &mut Context<DesignerWorkspace>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(8.))
            .px(px(18.))
            .py(px(12.))
            .bg(theme.colors.chrome)
            .border_t_1()
            .border_color(theme.colors.line)
            .rounded_b(px(12.))
            .child(
                div()
                    .flex_1()
                    .text_size(px(11.5))
                    .text_color(theme.colors.muted)
                    .child(DesignerTexts::PROJECT_FOOTNOTE),
            )
            .child(
                div()
                    .id("cancel-project")
                    .px(px(12.))
                    .py(px(6.))
                    .rounded(theme.metrics.control_radius)
                    .border_1()
                    .border_color(theme.colors.line)
                    .bg(theme.colors.surface)
                    .cursor_pointer()
                    .on_click(cx.listener(|workspace, _, _, cx| workspace.close_new_project(cx)))
                    .child(DesignerTexts::CANCEL),
            )
            .child(
                div()
                    .id("create-project")
                    .px(px(12.))
                    .py(px(6.))
                    .rounded(theme.metrics.control_radius)
                    .bg(theme.colors.accent)
                    .text_color(theme.colors.on_accent)
                    .font_weight(FontWeight::SEMIBOLD)
                    .cursor_pointer()
                    .on_click(cx.listener(|workspace, _, _, cx| workspace.create_project(cx)))
                    .child(DesignerTexts::CREATE_PROJECT),
            )
    }
}
