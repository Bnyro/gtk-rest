use gtk::{
    glib,
    glib::clone,
    traits::{BoxExt, ButtonExt, WidgetExt},
    Button, Orientation,
};

pub struct RequestRow {
    pub name: String,
}

impl RequestRow {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn build<F: Fn(String, &gtk::Box) + 'static, G: Fn(String, &gtk::Box) + 'static>(
        &self,
        on_click: F,
        on_delete: G,
    ) -> gtk::Box {
        let req_name = self.name.clone();
        let container = gtk::Box::new(Orientation::Horizontal, 10);
        container.set_margin_bottom(10);
        let name = Button::with_label(&self.name.as_str());
        name.set_hexpand(true);
        let delete = Button::from_icon_name("edit-delete");
        name.connect_clicked(clone!(@weak container as cnt => move |button| {
            on_click(button.label().unwrap().to_string(), &cnt);
        }));
        delete.connect_clicked(clone!(@weak container as cnt => move |_button| {
            on_delete(req_name.clone(), &cnt);
        }));
        container.append(&name);
        container.append(&delete);
        container
    }
}
