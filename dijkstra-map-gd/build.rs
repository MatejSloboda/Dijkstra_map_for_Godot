use gdnative_doc::{init_logger, Backend, Builder};
use std::path::PathBuf;

fn main() -> gdnative_doc::Result<()> {
    init_logger(gdnative_doc::LevelFilter::Info)?;
    Builder::new()
        .user_config(PathBuf::from("gdnative-doc.toml"))
        .add_backend(Backend::Markdown {
            output_dir: PathBuf::from("../addons/dijkstra-map/doc"),
        })
        .add_backend(Backend::Gut {
            output_dir: PathBuf::from("../Tests/unit"),
        })
        .build()?;

    Ok(())
}
