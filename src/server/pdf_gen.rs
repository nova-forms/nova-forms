use leptos::IntoView;
use std::{
    path::{Path, PathBuf},
    process::{ExitStatus, Stdio},
    sync::Arc,
};
use thiserror::Error;
use tokio::{fs::File, io::AsyncWriteExt, process::Command};
use uuid::Uuid;

/// A PDF generator.
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
    /// Creates a new `PdfGen`.
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

    /// Renders a form as a PDF.
    pub async fn render_form<F, IV>(&self, form: F) -> Result<PathBuf, Error>
    where
        F: FnOnce() -> IV + Send + 'static,
        IV: IntoView + 'static,
    {
        use leptos::*;

        let conf = get_configuration(None).await.unwrap();
        let pkg_dir = conf.leptos_options.site_pkg_dir; 
        let site_root = conf.leptos_options.site_root;
        let output_name = conf.leptos_options.output_name;
        
        let print_css_path = format!("{}/print.css", site_root);
        let print_css_path = Path::new(&print_css_path);
        let style_css_path = format!("{}/{}/{}.css", site_root, pkg_dir, output_name);
        let style_css_path = Path::new(&style_css_path);

        let styles = [
            &style_css_path,
            &print_css_path,
        ];

        let base_path = std::env::current_dir().unwrap();
        let mut style = String::new();

        for relative_path in styles {
            let mut css_path = base_path.clone();
            css_path.push(relative_path);
            let contents = tokio::fs::read_to_string(css_path).await?;
            style.push_str(&contents);
            style.push_str("\n");
        }
        
        let html = leptos::ssr::render_to_string(move || {
            view! {
                <style>{style}</style>
                <div id="print">
                    {form()}
                </div>
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
