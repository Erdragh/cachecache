use std::path::PathBuf;

use std::cell::RefCell;

use gtk::glib::{self, Properties};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::CacheCacheWindow)]
pub struct CacheCacheWindow {
    #[property(get, set)]
    pub path_buf: RefCell<PathBuf>,
}

#[glib::object_subclass]
impl ObjectSubclass for CacheCacheWindow {
    const NAME: &'static str = "CacheCacheWindow";
    type Type = super::CacheCacheWindow;
    type ParentType = gtk::ApplicationWindow;
}

impl ObjectImpl for CacheCacheWindow {
    fn properties() -> &'static [glib::ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        self.derived_property(id, pspec)
    }

    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for CacheCacheWindow {}

impl WindowImpl for CacheCacheWindow {}

impl ApplicationWindowImpl for CacheCacheWindow {}
