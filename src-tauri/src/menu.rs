use tauri::{AppHandle, menu::*};

pub fn build_menu(app: &AppHandle) -> Result<Menu<tauri::Wry>, tauri::Error> {
    // Edit menu with standard shortcuts
    let copy = PredefinedMenuItem::copy(app, None)?;
    let cut = PredefinedMenuItem::cut(app, None)?;
    let paste = PredefinedMenuItem::paste(app, None)?;
    let select_all = PredefinedMenuItem::select_all(app, None)?;
    let undo = PredefinedMenuItem::undo(app, None)?;
    let redo = PredefinedMenuItem::redo(app, None)?;
    let edit_menu = SubmenuBuilder::new(app, "Edit")
        .items(&[&undo, &redo, &PredefinedMenuItem::separator(app)?, &cut, &copy, &paste, &PredefinedMenuItem::separator(app)?, &select_all])
        .build()?;

    let export_json = MenuItemBuilder::with_id("export-json", "JSON").build(app)?;
    let export_pdf = MenuItemBuilder::with_id("export-pdf", "PDF").build(app)?;
    let export_markdown = MenuItemBuilder::with_id("export-markdown", "Markdown").build(app)?;
    let export_menu = SubmenuBuilder::new(app, "Export")
        .items(&[&export_json, &export_pdf, &export_markdown])
        .build()?;

    let load_json = MenuItemBuilder::with_id("load-json", "From JSON").build(app)?;
    let load_menu = SubmenuBuilder::new(app, "Load")
        .items(&[&load_json])
        .build()?;

    let open_settings = MenuItemBuilder::with_id("open-settings", "Preferences...")
        .accelerator("CmdOrCtrl+,")
        .build(app)?;
    let settings_menu = SubmenuBuilder::new(app, "Settings")
        .items(&[&open_settings])
        .build()?;

    let menu = MenuBuilder::new(app)
        .items(&[&edit_menu, &export_menu, &load_menu, &settings_menu])
        .build()?;

    Ok(menu)
}
