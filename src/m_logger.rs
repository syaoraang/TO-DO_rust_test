
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

pub fn init_logs(file_output:&str)
{
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {M}[{P}-{I}]: {m}{n}")))
        .build(file_output).unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build(file_output, Box::new(requests)))
        .build(Root::builder().appender(file_output).build(LevelFilter::Trace))
        .unwrap();
    let _ = log4rs::init_config(config).unwrap();
}