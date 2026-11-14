use std::collections::HashSet;
use std::rc::Rc;

use crisol_catalog::WidgetCatalog;
use crisol_engine::dragging::DropTarget;
use crisol_model::document::FormDocument;
use crisol_model::identity::NodeId;
use crisol_ui::components::{TextBox, TextBoxEvent};
use crisol_ui::theme::Theme;
use gpui::{
    AppContext, Context, CursorStyle, Entity, FocusHandle, Focusable, InteractiveElement,
    IntoElement, ParentElement, Render, Styled, Subscription, Window, div, prelude::FluentBuilder,
};

use super::{
    BottomTab, CanvasInteraction, ExplorerEntry, NewProjectState, PropertyEditor, PropertyKey,
    PropertyTab,
};
use crate::code::CodeView;
use crate::designer_settings::DesignerSettings;
use crate::panels::{
    DocumentArea, NewProjectDialog, SidePanel, StatusBar, TitleBar, ToolBar, ToolboxPanel,
};
use crate::project::{ProjectSession, RecentProjects};
use crate::session::{DocumentSession, OutputLog};

/// The designer window. It owns the open documents and the state of every panel; the
/// panels only read it and call back into it.
#[allow(
    clippy::struct_excessive_bools,
    reason = "independent flags of separate panels"
)]
pub struct DesignerWorkspace {
    pub(super) catalog: Rc<WidgetCatalog>,
    pub(super) settings: DesignerSettings,
    pub(super) sessions: Vec<DocumentSession>,
    pub(super) active: usize,
    pub(super) output: OutputLog,
    pub(super) bottom_tab: BottomTab,
    pub(super) bottom_collapsed: bool,
    pub(super) property_tab: PropertyTab,
    pub(super) focused_property: Option<PropertyKey>,
    pub(super) property_editor: Option<PropertyEditor>,
    pub(super) open_menu: Option<usize>,
    pub(super) interaction: CanvasInteraction,
    pub(super) canvas_cursor: CursorStyle,
    pub(super) toolbox_hover: Option<DropTarget>,
    pub(super) explorer: Vec<ExplorerEntry>,
    pub(super) recent: RecentProjects,
    pub(super) show_start: bool,
    pub(super) new_project: Option<NewProjectState>,
    pub(super) clipboard: Option<(FormDocument, Vec<NodeId>)>,
    pub(super) toolbox_search: Entity<TextBox>,
    pub(super) collapsed_groups: HashSet<String>,
    pub(super) property_search: Entity<TextBox>,
    pub(super) alphabetical: bool,
    pub(super) collapsed_categories: HashSet<String>,
    pub(super) object_list_open: bool,
    pub(super) palette_open: bool,
    pub(super) palette_search: Entity<TextBox>,
    pub(super) project: Option<ProjectSession>,
    pub(super) code_views: Vec<CodeView>,
    pub(super) active_code: Option<usize>,
    pub(super) building: bool,
    /// The first session is only a stand-in while the start page shows; the first real
    /// document replaces it.
    pub(super) placeholder: bool,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl Focusable for DesignerWorkspace {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl DesignerWorkspace {
    /// A workspace with `first` open; `show_start` shows the start page instead of it.
    pub fn new(
        first: DocumentSession,
        show_start: bool,
        catalog: Rc<WidgetCatalog>,
        cx: &mut Context<Self>,
    ) -> Self {
        let toolbox_search = cx.new(|cx| TextBox::new(cx).placeholder("Search control"));
        let property_search = cx.new(|cx| TextBox::new(cx).placeholder("Search"));
        let palette_search = cx.new(|cx| TextBox::new(cx).placeholder("Type a command"));
        let mut subscriptions = Vec::new();
        for field in [&toolbox_search, &property_search, &palette_search] {
            subscriptions.push(cx.subscribe(field, |_, _, event: &TextBoxEvent, cx| {
                if *event == TextBoxEvent::Changed {
                    cx.notify();
                }
            }));
        }
        let mut workspace = Self {
            catalog,
            settings: DesignerSettings::default(),
            sessions: vec![first],
            active: 0,
            output: OutputLog::default(),
            bottom_tab: BottomTab::Errors,
            bottom_collapsed: false,
            property_tab: PropertyTab::Properties,
            focused_property: None,
            property_editor: None,
            open_menu: None,
            interaction: CanvasInteraction::Idle,
            canvas_cursor: CursorStyle::Arrow,
            toolbox_hover: None,
            explorer: Vec::new(),
            recent: RecentProjects::load(),
            show_start,
            new_project: None,
            clipboard: None,
            toolbox_search,
            collapsed_groups: HashSet::new(),
            property_search,
            alphabetical: false,
            collapsed_categories: HashSet::new(),
            object_list_open: false,
            palette_open: false,
            palette_search,
            project: None,
            code_views: Vec::new(),
            active_code: None,
            building: false,
            placeholder: show_start,
            focus_handle: cx.focus_handle(),
            _subscriptions: subscriptions,
        };
        workspace.refresh_explorer();
        workspace.output.info("Crisol is ready.");
        workspace
    }

    pub fn settings(&self) -> &DesignerSettings {
        &self.settings
    }

    pub fn catalog(&self) -> &Rc<WidgetCatalog> {
        &self.catalog
    }

    pub fn sessions(&self) -> &[DocumentSession] {
        &self.sessions
    }

    pub fn active_index(&self) -> usize {
        self.active
    }

    pub fn session(&self) -> &DocumentSession {
        &self.sessions[self.active]
    }

    pub(super) fn session_mut(&mut self) -> &mut DocumentSession {
        &mut self.sessions[self.active]
    }

    pub fn output(&self) -> &OutputLog {
        &self.output
    }

    pub fn bottom_tab(&self) -> BottomTab {
        self.bottom_tab
    }

    pub fn bottom_collapsed(&self) -> bool {
        self.bottom_collapsed
    }

    pub fn property_tab(&self) -> PropertyTab {
        self.property_tab
    }

    pub fn focused_property(&self) -> Option<&PropertyKey> {
        self.focused_property.as_ref()
    }

    pub fn property_editor(&self) -> Option<&PropertyEditor> {
        self.property_editor.as_ref()
    }

    pub fn open_menu(&self) -> Option<usize> {
        self.open_menu
    }

    pub fn interaction(&self) -> &CanvasInteraction {
        &self.interaction
    }

    pub fn canvas_cursor(&self) -> CursorStyle {
        self.canvas_cursor
    }

    pub fn toolbox_hover(&self) -> Option<&DropTarget> {
        self.toolbox_hover.as_ref()
    }

    pub fn explorer(&self) -> &[ExplorerEntry] {
        &self.explorer
    }

    pub fn recent(&self) -> &RecentProjects {
        &self.recent
    }

    pub fn showing_start(&self) -> bool {
        self.show_start
    }

    pub fn new_project(&self) -> Option<&NewProjectState> {
        self.new_project.as_ref()
    }

    pub fn has_clipboard(&self) -> bool {
        self.clipboard.is_some()
    }

    pub fn toolbox_search(&self) -> &Entity<TextBox> {
        &self.toolbox_search
    }

    pub fn is_group_collapsed(&self, group: &str) -> bool {
        self.collapsed_groups.contains(group)
    }

    pub fn property_search(&self) -> &Entity<TextBox> {
        &self.property_search
    }

    pub fn alphabetical(&self) -> bool {
        self.alphabetical
    }

    pub fn is_category_collapsed(&self, category: &str) -> bool {
        self.collapsed_categories.contains(category)
    }

    pub fn object_list_open(&self) -> bool {
        self.object_list_open
    }

    pub fn palette_open(&self) -> bool {
        self.palette_open
    }

    pub fn palette_search(&self) -> &Entity<TextBox> {
        &self.palette_search
    }

    pub fn project(&self) -> Option<&ProjectSession> {
        self.project.as_ref()
    }

    pub fn code_views(&self) -> &[CodeView] {
        &self.code_views
    }

    /// The code tab shown instead of the designer, if any.
    pub fn active_code(&self) -> Option<&CodeView> {
        self.active_code
            .and_then(|index| self.code_views.get(index))
    }

    pub fn active_code_index(&self) -> Option<usize> {
        self.active_code
    }

    pub fn building(&self) -> bool {
        self.building
    }

    pub fn show_bottom_tab(&mut self, tab: BottomTab, cx: &mut Context<Self>) {
        self.bottom_tab = tab;
        self.bottom_collapsed = false;
        cx.notify();
    }

    pub fn show_property_tab(&mut self, tab: PropertyTab, cx: &mut Context<Self>) {
        self.property_tab = tab;
        self.property_editor = None;
        cx.notify();
    }

    /// Adds a document tab and shows it, replacing the start page's stand-in.
    pub(super) fn open_session(&mut self, session: DocumentSession, cx: &mut Context<Self>) {
        if self.placeholder {
            self.sessions.clear();
            self.placeholder = false;
        }
        self.sessions.push(session);
        self.activate(self.sessions.len() - 1, cx);
    }

    pub fn activate(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.sessions.len() {
            self.active = index;
            self.active_code = None;
            self.show_start = false;
            self.property_editor = None;
            self.interaction = CanvasInteraction::Idle;
            self.refresh_explorer();
            cx.notify();
        }
    }
}

impl Render for DesignerWorkspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let root = div()
            .key_context(crate::actions::DesignerKeyBindings::CONTEXT)
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .flex_col()
            .relative()
            .bg(theme.colors.surface)
            .text_color(theme.colors.ink)
            .font_family(theme.fonts.body.clone())
            .text_size(theme.metrics.text_size);
        let dialog = self.new_project.is_some();
        Self::with_actions(root, cx)
            .child(TitleBar::new(self).render(window, cx))
            .child(ToolBar::new(self).render(window, cx))
            .child(
                div()
                    .flex()
                    .flex_1()
                    .min_h_0()
                    .child(ToolboxPanel::new(self).render(window, cx))
                    .child(DocumentArea::new(self).render(window, cx))
                    .child(SidePanel::new(self).render(window, cx)),
            )
            .child(StatusBar::new(self).render(window, cx))
            .when(dialog, |root| {
                root.child(NewProjectDialog::new(self).render(window, cx))
            })
    }
}
