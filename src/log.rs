use log;

pub fn init() {
    env_logger::init();
}

pub fn debug(msg: &str) {
    log::debug!("{}", msg);
}

pub fn error(msg: &str) {
    log::error!("{}", msg);
}

pub fn info(msg: &str) {
    log::info!("{}", msg);
}

pub fn warn(msg: &str) {
    log::warn!("{}", msg);
}
