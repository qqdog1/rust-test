use log::info;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};



fn main() {
    initlogger();

    info!("ggg");
}

fn initlogger() {
    let stdout = ConsoleAppender::builder().build();

    let fileAppender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("logs/test.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("fileAppender", Box::new(fileAppender)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .build(Root::builder().appender("fileAppender").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}