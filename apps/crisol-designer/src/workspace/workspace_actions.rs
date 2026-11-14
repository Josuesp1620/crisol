use crisol_engine::arranging::Alignment;
use gpui::{Context, Div, InteractiveElement};

use super::DesignerWorkspace;
use crate::actions::{
    AddForm, BuildProject, CloseMenu, CloseTab, CopySelection, DeleteSelection, DuplicateSelection,
    GenerateCode, NewForm, NewProject, NudgeDown, NudgeDownFar, NudgeLeft, NudgeLeftFar,
    NudgeRight, NudgeRightFar, NudgeUp, NudgeUpFar, OpenForm, OpenPalette, OpenProject,
    PasteClipboard, Quit, Redo, RunPreview, RunProject, SaveAll, SaveForm, SaveFormAs,
    SelectParent, ToggleTheme, Undo, ViewCode, ViewDesigner,
};

/// Connects keyboard actions to the workspace.
impl DesignerWorkspace {
    /// Pixels an arrow key moves a control; with Shift it moves one grid step.
    const NUDGE: i32 = 1;
    const NUDGE_FAR: i32 = 8;

    pub(super) fn with_actions(root: Div, cx: &mut Context<Self>) -> Div {
        root.on_action(cx.listener(|workspace, _: &NewProject, window, cx| {
            workspace.open_new_project(window, cx);
        }))
        .on_action(
            cx.listener(|workspace, _: &OpenProject, window, cx| workspace.open_form(window, cx)),
        )
        .on_action(cx.listener(|workspace, _: &AddForm, _, cx| workspace.add_form(cx)))
        .on_action(cx.listener(|workspace, _: &SaveAll, window, cx| workspace.save_all(window, cx)))
        .on_action(cx.listener(|workspace, _: &ViewCode, _, cx| workspace.view_code(None, cx)))
        .on_action(cx.listener(|workspace, _: &ViewDesigner, _, cx| workspace.view_designer(cx)))
        .on_action(cx.listener(|workspace, _: &BuildProject, _, cx| workspace.build_project(cx)))
        .on_action(cx.listener(|workspace, _: &RunProject, _, cx| workspace.run_project(cx)))
        .on_action(cx.listener(|workspace, _: &NewForm, window, cx| workspace.new_form(window, cx)))
        .on_action(
            cx.listener(|workspace, _: &OpenForm, window, cx| workspace.open_form(window, cx)),
        )
        .on_action(
            cx.listener(|workspace, _: &SaveForm, window, cx| workspace.save_form(window, cx)),
        )
        .on_action(
            cx.listener(|workspace, _: &SaveFormAs, window, cx| workspace.save_form_as(window, cx)),
        )
        .on_action(cx.listener(|workspace, _: &CloseTab, window, cx| {
            workspace.close_tab(workspace.active, window, cx);
        }))
        .on_action(cx.listener(|workspace, _: &GenerateCode, _, cx| workspace.generate_code(cx)))
        .on_action(cx.listener(|workspace, _: &Undo, _, cx| workspace.undo(cx)))
        .on_action(cx.listener(|workspace, _: &Redo, _, cx| workspace.redo(cx)))
        .on_action(
            cx.listener(|workspace, _: &DeleteSelection, _, cx| workspace.delete_selection(cx)),
        )
        .on_action(cx.listener(|workspace, _: &CopySelection, _, cx| workspace.copy_selection(cx)))
        .on_action(
            cx.listener(|workspace, _: &PasteClipboard, _, cx| workspace.paste_clipboard(cx)),
        )
        .on_action(cx.listener(|workspace, _: &DuplicateSelection, _, cx| {
            workspace.duplicate_selection(cx);
        }))
        .on_action(cx.listener(|workspace, _: &SelectParent, _, cx| workspace.select_parent(cx)))
        .on_action(
            cx.listener(|workspace, _: &NudgeLeft, _, cx| workspace.nudge(-Self::NUDGE, 0, cx)),
        )
        .on_action(
            cx.listener(|workspace, _: &NudgeRight, _, cx| workspace.nudge(Self::NUDGE, 0, cx)),
        )
        .on_action(
            cx.listener(|workspace, _: &NudgeUp, _, cx| workspace.nudge(0, -Self::NUDGE, cx)),
        )
        .on_action(
            cx.listener(|workspace, _: &NudgeDown, _, cx| workspace.nudge(0, Self::NUDGE, cx)),
        )
        .on_action(cx.listener(|workspace, _: &NudgeLeftFar, _, cx| {
            workspace.nudge(-Self::NUDGE_FAR, 0, cx);
        }))
        .on_action(cx.listener(|workspace, _: &NudgeRightFar, _, cx| {
            workspace.nudge(Self::NUDGE_FAR, 0, cx);
        }))
        .on_action(cx.listener(|workspace, _: &NudgeUpFar, _, cx| {
            workspace.nudge(0, -Self::NUDGE_FAR, cx);
        }))
        .on_action(cx.listener(|workspace, _: &NudgeDownFar, _, cx| {
            workspace.nudge(0, Self::NUDGE_FAR, cx);
        }))
        .on_action(cx.listener(|workspace, _: &RunPreview, _, cx| workspace.run_preview(cx)))
        .on_action(cx.listener(|workspace, _: &ToggleTheme, _, cx| workspace.toggle_theme(cx)))
        .on_action(cx.listener(|workspace, _: &OpenPalette, window, cx| {
            workspace.toggle_palette(window, cx);
        }))
        .on_action(cx.listener(|workspace, _: &CloseMenu, _, cx| workspace.escape(cx)))
        .on_action(cx.listener(|_, _: &Quit, _, cx| cx.quit()))
    }

    /// Esc closes what is open, or else selects the container of the selection.
    fn escape(&mut self, cx: &mut Context<Self>) {
        if self.new_project.is_some() {
            self.new_project = None;
        } else if self.open_menu.is_some() || self.palette_open || self.object_list_open {
            self.open_menu = None;
            self.palette_open = false;
            self.object_list_open = false;
        } else {
            self.select_parent(cx);
        }
        cx.notify();
    }

    pub fn arrange_selection(&mut self, alignment: Alignment, cx: &mut Context<Self>) {
        let selection = self.session().selection().nodes().to_vec();
        let transaction = crisol_engine::arranging::AlignmentTool::new(self.session().document())
            .arrange(&selection, alignment);
        if let Some(transaction) = transaction {
            self.execute(&transaction, cx);
        } else {
            self.output
                .info("Select two or more controls on an absolute panel to align them.");
            cx.notify();
        }
    }
}
