use std::{fmt::Display, str::FromStr};

use leptos::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Icon, QueryString};
use serde::de::Error;
use server_fn::codec::{MultipartData, MultipartFormData};
use web_sys::{wasm_bindgen::JsCast, FormData, HtmlInputElement};

// See this for reference: https://github.com/leptos-rs/leptos/blob/96e2b5cba10d2296f262820be19cac9b615b0d23/examples/server_fns_axum/src/app.rs

#[component]
pub fn FileUpload(#[prop(into)] bind: QueryString) -> impl IntoView {
    let (qs, _form_data) = bind.form_context();

    let (file_info, set_file_info) = create_signal(Vec::new());

    let on_input = move |ev: web_sys::Event| {
        let target = ev
            .target()
            .expect("target must exist")
            .unchecked_into::<HtmlInputElement>();

        if let Some(files) = target.files() {
            let form_data: FormData = FormData::new().expect("can create form data");

            for i in 0..files.length() {
                let file = files.get(i).expect("file must exist");
                let file_name = file.name();

                form_data
                    .append_with_blob_and_filename(&file_name, &file, &file_name)
                    .expect("appending file to form data must be successful");
            }

            spawn_local(async move {
                let mut new_file_infos = upload_file(form_data.into())
                    .await
                    .expect("couldn't upload file");

                set_file_info.update(|file_info| {
                    file_info.append(&mut new_file_infos);
                });
            });
        }
    };

    view! {
        <label class="button icon-button" for=qs.to_string()><Icon label="Upload" icon="upload" /></label>
        <input id=qs.to_string() type="file" class="sr-hidden" on:input=on_input/>
        <ul>
            <For
                each=move || file_info.get().into_iter().enumerate()
                key=|(_, file_info)| file_info.file_id()
                // renders each item to a view
                children=move |(i, file_info)| {
                    let (qs, _file_name) = bind.clone().add_index(i).form_value::<String>();

                    view! {
                        <li>
                            { format!("{}", file_info.file_name) }
                            <input type="hidden" name=qs value=file_info.file_id.to_string()></input>
                        </li>

                    }
                }
            />
        </ul>

    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(Uuid);

impl FileId {
    pub fn new() -> Self {
        FileId(Uuid::new_v4())
    }
}

impl Display for FileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "file_id_{}", self.0)
    }
}

impl Serialize for FileId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("file_id_{}", self.0))
    }
}

impl<'de> Deserialize<'de> for FileId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut value = String::deserialize(deserializer)?;
        if !value.starts_with("file_id_") {
            return Err(D::Error::custom("prefix mismatch"));
        }
        match Uuid::from_str(&value.split_off(8)) {
            Ok(uuid) => Ok(FileId(uuid)),
            Err(_) => Err(D::Error::custom("invalid uuid")),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FileInfo {
    file_id: FileId,
    file_name: String,
    content_type: Option<String>,
}

impl FileInfo {
    pub fn new(file_id: FileId, file_name: String, content_type: Option<String>) -> Self {
        FileInfo { file_id, file_name, content_type }
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn file_id(&self) -> FileId {
        self.file_id
    }
}

#[server(input = MultipartFormData)]
async fn upload_file(data: MultipartData) -> Result<Vec<FileInfo>, ServerFnError> {
    let mut data = data.into_inner().unwrap();
    let mut file_info = Vec::new();

    while let Ok(Some(mut field)) = data.next_field().await {
        let content_type = field.content_type().map(|mime| mime.to_string());
        let file_name = field.file_name().expect("no filename on field").to_string();
        while let Ok(Some(chunk)) = field.chunk().await {
            let len = chunk.len();
            println!("[{file_name}]\t{len}");
            //progress::add_chunk(&name, len).await;
            // in a real server function, you'd do something like saving the file here
        }

        file_info.push(FileInfo::new(FileId::new(), file_name, content_type));
    }

    Ok(file_info)
}
