#![windows_subsystem = "windows"]
mod win;
mod ini;
use anyhow::Result;
fn main(){
    if let Err(e) = run_apps()  {
        let _ = win::message_box("提示", format!("{e}").as_str());
    }
}

fn run_apps() -> Result<()> {
    let conf = ini::AppConfig::load("conf.ini")?;
    win::run_apps(&conf.programs,conf.auto_login,conf.close_first)
}
