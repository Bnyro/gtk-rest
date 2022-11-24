use gtk::{
    glib,
    glib::clone,
    traits::{BoxExt, ButtonExt, EditableExt, WidgetExt},
};

use crate::preferences::KeyValuePair;

pub struct KvPair {}

impl KvPair {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build<F: Fn(KeyValuePair) + 'static>(
        &mut self,
        parent: &gtk::Box,
        on_change: F,
        on_change_clone: F,
    ) -> gtk::Box {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        container.set_margin_top(5);
        container.set_margin_bottom(5);
        let key = gtk::Entry::new();
        let value = gtk::Entry::new();
        value.set_hexpand(true);
        key.connect_changed(clone!(@weak value as val => move |k| {
            on_change(KeyValuePair { key: k.text().to_string(), value: val.text().to_string() })
        }));
        value.connect_changed(clone!(@weak key as k => move |val| {
            on_change_clone(KeyValuePair { key: k.text().to_string(), value: val.text().to_string() })
        }));
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
