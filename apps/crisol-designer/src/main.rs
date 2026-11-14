// A window application on Windows: no console window behind the designer.
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

fn main() {
    crisol_designer::designer_application::DesignerApplication::run();
}
