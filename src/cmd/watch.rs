use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;

use crate::watch::WatchSystem;

/// Watch the Rust WASM app and execute builds as changes are detected.
#[derive(StructOpt)]
#[structopt(name="watch")]
pub struct Watch {
    /// The index HTML file to drive the bundling process.
    #[structopt(parse(from_os_str))]
    target: PathBuf,

    /// Build in release mode.
    #[structopt(long)]
    release: bool,
    /// The output dir for all final assets.
    #[structopt(short, long, default_value="dist", parse(from_os_str))]
    dist: PathBuf,
    /// The public URL from which assets are to be served.
    #[structopt(short, long, default_value="/")]
    public_url: String,
    /// Additional paths to ignore.
    #[structopt(short, long, parse(from_os_str))]
    ignore: Option<Vec<PathBuf>>,
}

impl Watch {
    pub async fn run(self) -> Result<()> {
        let mut system = WatchSystem::new(self.target, self.release, self.dist, self.public_url, self.ignore.unwrap_or_default())
            .await?;
        system.build().await;
        system.run().await;
        Ok(())
    }
}
