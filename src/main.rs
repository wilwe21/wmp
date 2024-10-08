use gtk::prelude::*;
use gtk::gdk;

struct Window {
    window: gtk::ApplicationWindow
}

impl Window {
    fn new(app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::builder()
            .title("WMP")
            .application(app)
            .build();
        load_css();
        window.show();
        Self {
            window
        }
    }
}

fn on_active(app: &gtk::Application) {
    let win = Window::new(app);
}

fn load_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    let css_content = include_str!("../css/main.css");
    provider.load_from_data(css_content);
    gtk::StyleContext::add_provider_for_display(&display, &provider, priority);
}


fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.wmp")
        .build();
    app.connect_activate(on_active);
    app.run();
}
