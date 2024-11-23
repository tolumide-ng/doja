use std::{sync::atomic::AtomicBool, thread::JoinHandle};

pub(crate) struct ThreadPool {
    handles: Vec<JoinHandle<()>>,
    pub(crate) stop: AtomicBool,
}