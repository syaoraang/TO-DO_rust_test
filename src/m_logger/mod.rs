
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

/*
Load this where we would need to use the logger.
use log::{error, info, warn, debug};
*/

const LOG_OUTPUT: &'static str = "traces.log";
const PATH_LOGS: &'static str = "./logs/";
pub fn init_logs(file_output:String)
{
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {M}[{P}-{I}]: {m}{n}")))
        .build(&file_output).unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build(&file_output, Box::new(requests)))
        .build(Root::builder().appender(&file_output).build(LevelFilter::Trace))
        .unwrap();
    let _ = log4rs::init_config(config).unwrap();
}
pub fn init_logs_default()
{
    let logs_path = std::path::Path::new(PATH_LOGS).join(LOG_OUTPUT).to_str().unwrap().to_string();
    init_logs(logs_path)
}