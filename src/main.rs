mod ui;

use ui::MainWindow;

use gtk4::{
    gio::resources_register_include,
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::WidgetExt,
};
use libadwaita::Application;

fn build_ui(application: &Application) {
    let window = MainWindow::new(application);
    window.show();
}

pub fn main() {
    resources_register_include!("86box.gresource")
        .expect("Failed to register resources.");

    let application = Application::builder()
        .application_id("leap._86box.manager")
        .build();

    application.connect_activate(build_ui);
    application.run();
}
