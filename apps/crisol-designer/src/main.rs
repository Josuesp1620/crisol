mod actions;
mod canvas;
mod chrome;
mod designer_application;
mod designer_settings;
mod designer_texts;
mod panels;
mod session;
mod workspace;

fn main() {
    designer_application::DesignerApplication::run();
}
