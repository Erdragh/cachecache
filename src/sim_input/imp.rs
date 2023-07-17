use gtk::glib;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct SimInput;

#[glib::object_subclass]
impl ObjectSubclass for SimInput {
    const NAME: &'static str = "CacheCacheSimInput";
    type Type = super::SimInput;
    type ParentType = gtk::Box;
}

impl ObjectImpl for SimInput {}

impl WidgetImpl for SimInput {}

impl BoxImpl for SimInput {}
