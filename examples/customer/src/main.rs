// A window application on Windows: no console window behind the form.
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod customer_window;
mod forms;

fn main() {
    customer_window::CustomerWindow::run();
}
