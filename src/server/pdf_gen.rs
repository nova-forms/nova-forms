use leptos::IntoView;
use std::{
    path::PathBuf,
    process::{ExitStatus, Stdio},
    sync::Arc,
};
use thiserror::Error;
use tokio::{fs::File, io::AsyncWriteExt, process::Command};
use uuid::Uuid;
use crate::SiteRoot;

/// A PDF generator.
#[derive(Clone)]
pub struct PdfGen {
    settings: Arc<Settings>,
    site_root: PathBuf,
}

struct Settings {
    working_dir: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            working_dir: std::env::temp_dir(),
        }
    }
}

impl PdfGen {
    /// Creates a new `PdfGen`.
    pub async fn new() -> Self {
        let conf = leptos::get_configuration(None).await.unwrap();
        let site_root = std::env::current_dir().unwrap().join(conf.leptos_options.site_root);

        Self {
            settings: Arc::new(Settings::default()),
            site_root
        }
    }

    async fn render_html<S: AsRef<str>>(&self, html: S) -> Result<PathBuf, Error> {
        let uuid = Uuid::new_v4();
        let name = uuid.to_string();
        let input_path = self
            .settings
            .working_dir
            .as_path()
            .join(format!("{name}.html"));
        let output_path = self
            .settings
            .working_dir
            .as_path()
            .join(format!("{name}.pdf"));

        let mut input_file = File::create(&input_path).await?;
        input_file.write_all(html.as_ref().as_bytes()).await?;

        let exit_status = Command::new("pagedjs-cli")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .current_dir(&self.settings.working_dir)
            .arg(input_path.file_name().unwrap())
            .arg("-o")
            .arg(output_path.file_name().unwrap())
            .status()
            .await?;

        if !exit_status.success() {
            return Err(Error::PdfGenerationError(exit_status));
        }

        Ok(output_path)
    }

    /// Renders a form as a PDF.
    pub async fn render_form<F, IV>(&self, form: F) -> Result<PathBuf, Error>
    where
        F: FnOnce() -> IV + 'static,
        IV: IntoView + 'static,
    {
        use leptos::*;
        use leptos_meta::*;
        use std::sync::{Arc, OnceLock};
        
        let site_root = self.site_root.clone();
        let head_metadata = Arc::new(OnceLock::new());
        
        let head_metadata_clone = head_metadata.clone();
        let html = leptos::ssr::render_to_string(move || {
            provide_context(SiteRoot::from(site_root));

            let view = form();

            head_metadata_clone.set(generate_head_metadata()).unwrap();

            view
        })
        .into_owned();

        let html = format!("<head>{}{}</body>", head_metadata.get().unwrap(), html);

        let output_path = self.render_html(html).await?;

        Ok(output_path)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Generation Error: {0}")]
    PdfGenerationError(ExitStatus),
}
