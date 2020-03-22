use log::info;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};

fn main() {
    initlogger();

    info!("ggg");
}

fn initlogger() {
    let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}