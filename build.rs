use std::{env, io::Write, fs};

use embuild::{
    self,
    build::{CfgArgs, LinkArgs},
};

fn main() -> anyhow::Result<()> {
    let outdir = env::var("OUT_DIR").unwrap();
    let outfile = format!("{}/timestamp.txt", outdir);

    let mut fh = fs::File::create(&outfile).unwrap();
    write!(fh, r#""{}""#, chrono::Local::now()).ok();

    // Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
    LinkArgs::output_propagated("ESP_IDF")?;

    let cfg = CfgArgs::try_from_env("ESP_IDF")?;

    // if cfg.get("esp32s2").is_some() {

    // }

    cfg.output();

    Ok(())
}
