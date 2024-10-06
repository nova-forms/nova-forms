use leptos::*;

pub fn use_zip_service() -> (impl Fn(leptos::ev::Event), Signal<Option<String>>) {
    let (zip, set_zip) = create_signal(String::new());
    let zip_service = create_server_action::<ZipService>();
    create_effect(move |_| {
        zip_service.dispatch(ZipService { zip: zip.get() });
    });
    let zip_service_value = zip_service.value();
    let city = create_memo(move |_| {
        if let Some(Ok(Some(city))) = zip_service_value.get() {
            Some(city)
        } else {
            None
        }
    });

    (move |ev| set_zip.set(event_target_value(&ev)), city.into())
}

#[server]
async fn zip_service(zip: String) -> Result<Option<String>, ServerFnError> {
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Response {
        zips: Vec<ResponseZip>,
    }

    #[derive(Deserialize)]
    struct ResponseZip {
        city18: String,
    }

    let zips = reqwest::get(format!(
        "https://service.post.ch/zopa/app/api/addresschecker/v1/zips?limit=2&zip={zip}"
    ))
    .await?
    .json::<Response>()
    .await?
    .zips;

    if zips.len() != 1 {
        return Ok(None);
    }

    Ok(Some(zips.first().unwrap().city18.to_owned()))
}
