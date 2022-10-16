use gdnative_doc::{backend::BuiltinBackend, init_logger, Builder, ConfigFile};
use std::path::PathBuf;

fn main() -> Result<(), String> {
    fn real_main() -> Result<(), gdnative_doc::Error> {
        init_logger(gdnative_doc::LevelFilter::Info)?;
        Builder::new()
            .user_config(ConfigFile::load_from_path(PathBuf::from(
                "gdnative-doc.toml",
            ))?)
            .add_backend(
                BuiltinBackend::Markdown,
                PathBuf::from("../addons/dijkstra-map/doc"),
            )
            .add_backend(BuiltinBackend::Gut, PathBuf::from("../Tests/unit"))
            .build()?;

        Ok(())
    }
    real_main().map_err(|err| format!("{}", err))
}
