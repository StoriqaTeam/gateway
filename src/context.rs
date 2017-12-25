use juniper;
use settings::Settings;
use tokio_core::reactor::{Remote};

pub struct Context {
    pub config: Settings,
    pub remote: Remote,
}

unsafe impl Sync for Context {}
unsafe impl Send for Context {}

impl Context {
    pub fn new(settings: Settings, remote: Remote) -> Self {
        Context {
            config: settings,
            remote: remote
        }
    }

}

impl juniper::Context for Context {}
