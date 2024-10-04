use std::{
    path::PathBuf,
    process::{ExitStatus, Stdio},
    sync::Arc,
};
use thiserror::Error;
use tokio::{fs::File, io::AsyncWriteExt, process::Command};
use uuid::Uuid;

#[derive(Clone)]
pub struct PdfGen {
    settings: Arc<Settings>,
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
    pub fn new() -> Self {
        Self {
            settings: Arc::new(Settings::default()),
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

    pub async fn render_form(
        &self,
        form: impl FnOnce() -> leptos::View + 'static,
    ) -> Result<PathBuf, Error> {
        use leptos::*;
        use leptos_meta::provide_meta_context;
        use tokio::{fs::File, io::AsyncReadExt};

        let mut dir = std::env::current_dir().unwrap();
        dir.push("style");
        dir.push("main.css");

        let mut file = File::open(dir).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        let html = leptos::ssr::render_to_string(move || {
            provide_meta_context();

            view! {
                <style>{contents}</style>
                {form()}
            }
            .into_view()
        })
        .into_owned();

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
