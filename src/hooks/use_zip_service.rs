use leptos::*;

use crate::{Datatype, NonEmptyString, PostalCodeCH};

/// Provides a signal to get the city for a given swiss zip code.
/// This uses the swiss postal service address checker API (`https://service.post.ch/zopa/app/api/addresschecker/v1/zips`).
pub fn postal_code_service<S: SignalGet<Value = Option<PostalCodeCH>> + 'static>(postal_code: S) -> Signal<Option<NonEmptyString>> {
    let zip_service = create_server_action::<PostalCodeServiceServerFn>();
    create_effect(move |_| {
        if let Some(postal_code) = postal_code.get() {
            zip_service.dispatch(PostalCodeServiceServerFn { zip: postal_code.into() });
        }
    });
    let zip_service_value = zip_service.value();
    let city = create_memo(move |_| {
        if let Some(Ok(Some(city))) = zip_service_value.get() {
            NonEmptyString::validate(city).ok()
        } else {
            None
        }
    });
    city.into()
}

#[server]
async fn postal_code_service_server_fn(zip: String) -> Result<Option<String>, ServerFnError> {
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
 