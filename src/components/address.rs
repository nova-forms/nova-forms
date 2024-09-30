use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{Group, QueryString};

use super::Input;

#[component]
pub fn Address(#[prop(into)] bind: QueryString) -> impl IntoView{
    let zip_service = create_server_action::<ZipService>();
    let zip_service_value = zip_service.value();
    let city = create_memo(move |_| if let Some(Ok(Some(city))) = zip_service_value.get() {
        Some(city)
    } else {
        None
    });

    view! {
        <Group bind=bind>
            <fieldset class="cols-2">
                <legend>Address</legend>
                <Input<String> bind="street" label="Street" />
                <Input<String> bind="house_number" label="House Number" />
                <Input<String> bind="zip" label="ZIP Code" on:input=move |ev| {
                    zip_service.dispatch(ZipService { zip: event_target_value(&ev) });
                } />
                {                    
                    move || {                        
                        if let Some(city) = city.get() {
                            view! {
                                <Input<String> bind="city" label="City" value=city  />
                            }
                        } else  {
                            view! {
                                <Input<String> bind="city" label="City" />
                            }
                        }
                    }
                }
                <Input<String> bind="country" label="Country" />
            </fieldset>
        </Group>
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Address {
    pub street: String,
    pub house_number: String,
    pub zip: String,
    pub city: String,
    pub country: String,
}

#[server]
async fn zip_service(zip: String) -> Result<Option<String>, ServerFnError> {
    #[derive(Deserialize)]
    struct Response {
        zips: Vec<ResponseZip>,
    }

    #[derive(Deserialize)]
    struct ResponseZip {
        zip: String,
        city18: String,
    }
    
    let zips = reqwest::get(format!("https://service.post.ch/zopa/app/api/addresschecker/v1/zips?limit=2&zip={zip}"))
        .await?
        .json::<Response>()
        .await?
        .zips;

    if zips.len() != 1 {
        return Ok(None);
    }

    Ok(Some(zips.first().unwrap().city18.to_owned())) 
}
