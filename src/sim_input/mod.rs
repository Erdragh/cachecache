mod imp;

use glib::Object;
use gtk::{
    glib,
    prelude::{EntryBufferExtManual, ObjectExt},
    traits::BoxExt,
    Entry, EntryBuffer,
};

use crate::{extract_filename, window::CacheCacheWindow};

glib::wrapper! {
    pub struct SimInput(ObjectSubclass<imp::SimInput>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SimInput {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn use_window(&self, window: Option<&CacheCacheWindow>) {
        if let Some(window) = window {
            let buffer = EntryBuffer::builder().max_length(100).build();

            buffer.set_text(extract_filename(&window));
            let filename_input = Entry::builder()
                .placeholder_text("Filename")
                .buffer(&buffer)
                .build();
            //self.append(&filename_input);
        }
    }
}
