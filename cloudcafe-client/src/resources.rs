use std::cell::{Ref, RefCell, RefMut};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Resources(pub Rc<RefCell<InnerResources>>);
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct AppSettings {
    pub selected_mic: u32,
}
impl Default for AppSettings {
    fn default() -> Self {
        Self {
            selected_mic: 0,
        }
    }
}
pub struct InnerResources {
    pub(crate) app_settings: AppSettings,
}
impl Resources {
    pub fn init() -> Result<Self> {
        std::fs::create_dir_all("resources")?;
        let mut app_settings_buff = String::new();
        let mut app_settings_file = OpenOptions::new()
            .create(true).read(true).write(true)
            .open("resources/app_settings.toml")?;
        app_settings_file.read_to_string(&mut app_settings_buff)?;
        let app_settings: AppSettings = match toml::from_str(app_settings_buff.as_str()) {
            Ok(thing) => {
                thing
            }
            Err(_) => {
                let app_settings = AppSettings::default();
                app_settings_file.write_all(toml::to_string(&app_settings).unwrap().as_bytes()).unwrap();
                app_settings
            }
        };
        Ok(Self(Rc::new(RefCell::new(InnerResources { app_settings }))))
    }
    pub fn update_settings_file(&mut self) {
        let str = self.0.borrow().app_settings.clone();
        let mut app_settings_file = OpenOptions::new()
            .create(true).read(true).write(true).truncate(true)
            .open("resources/app_settings.toml").unwrap();
        app_settings_file.write_all(toml::to_string(&str).unwrap().as_bytes()).unwrap();
    }
}