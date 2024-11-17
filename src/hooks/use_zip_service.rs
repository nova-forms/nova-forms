use leptos::*;

/// Provides a service to get the city for a given swiss zip code.
/// This uses the swiss postal service address checker API (`https://service.post.ch/zopa/app/api/addresschecker/v1/zips`).
/// Returns a tuple that contains a function which takes an input event, and a signal that contains the city name.
/// The function taking an input event can be added to a input field and triggers the service call when the input field changes.
/// The response of the service call is stored in the signal.
pub fn use_zip_service() -> (Signal<Option<String>>, WriteSignal<String>) {
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

    (city.into(), set_zip)
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
 