use gpui::actions;

actions!(
    crisol_designer,
    [
        NewForm,
        OpenForm,
        SaveForm,
        SaveFormAs,
        GenerateCode,
        Undo,
        Redo,
        DeleteSelection,
        RunPreview,
        ToggleTheme,
        CloseMenu,
        Quit,
    ]
);
