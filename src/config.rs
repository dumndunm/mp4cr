use super::logger;

pub struct Config {
    pub minrate: &'static str,
    pub maxrate: &'static str,
    pub bufsize: &'static str,
    pub scale: &'static str,
}

static CONFIG_1080P: Config = Config {
    minrate: "4500k",
    maxrate: "9000k",
    bufsize: "9000k",
    scale: "1080",
};

static CONFIG_720P: Config = Config {
    minrate: "1500k",
    maxrate: "4000k",
    bufsize: "5000k",
    scale: "720",
};

static CONFIG_480P: Config = Config {
    minrate: "500k",
    maxrate: "2000k",
    bufsize: "2000k",
    scale: "480",
};

static CONFIG_360P: Config = Config {
    minrate: "400k",
    maxrate: "1000k",
    bufsize: "1500k",
    scale: "360",
};

pub static DEFAULT_CONFIG: &Config = &CONFIG_720P;

pub fn get(quality: &str) -> &Config {
    match quality {
        "1080p" => &CONFIG_1080P,
        "720p" => &CONFIG_720P,
        "480p" => &CONFIG_480P,
        "360p" => &CONFIG_360P,
        other => {
            logger::info(&format!("Not found `{}` quality, used `720p`", other));
            DEFAULT_CONFIG
        }
    }
}
