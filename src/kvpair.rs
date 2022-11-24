use gtk::{
    glib,
    glib::clone,
    traits::{BoxExt, ButtonExt, WidgetExt},
};

pub struct KvPair {
    pub key: String,
    pub value: String,
}

impl KvPair {
    pub fn new() -> Self {
        Self {
            key: String::from(""),
            value: String::from(""),
        }
    }

    pub fn build(&mut self, parent: &gtk::Box) -> gtk::Box {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        container.set_margin_top(5);
        container.set_margin_bottom(5);
        let key = gtk::Entry::new();
        let value = gtk::Entry::new();
        value.set_hexpand(true);
        let delete = gtk::Button::from_icon_name("edit-delete");
        delete.connect_clicked(
            clone!(@weak container as cnt, @weak parent as pnt => move |_| {
                pnt.remove(&cnt);
            }),
        );
        container.append(&key);
        container.append(&value);
        container.append(&delete);
        container
    }
}
