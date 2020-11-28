use quickxml_to_serde::{xml_string_to_json, Config};
use win_event_log::prelude::*;
fn main() {
    get_wind_log(
        "Application",
        "2020-11-01T11:14:50.000Z",
        "2020-11-26T11:14:50.999Z",
    )
}
pub fn get_wind_log(path: &str, start_datetime: &str, end_datetime: &str) {
    let query = format!(
        "<QueryList>
            <Query Id=\"0\" Path=\"{}\">
                <Select Path=\"Application\">
                *[System[TimeCreated[
                    @SystemTime&gt;='{}' and 
                    @SystemTime&lt;='{}']]]
                </Select>
            </Query>
      </QueryList>",
        path, start_datetime, end_datetime
    );
    match WinEvents::get(&query) {
        Ok(events) => {
            let e = events
                .into_iter()
                .map(|event| {
                    let json = xml_string_to_json(event.to_string(), &Config::new_with_defaults())
                        .unwrap();
                    json.to_string()
                })
                .collect::<Vec<String>>();
            println!("Parsed: {:?} ", e);
        }
        Err(e) => println!("Error: {}", e),
    }
    // match WinEvents::get(query) {
    //     Ok(events) => {
    //         if let Some(event) = events.into_iter().next() {
    //             let event = event.to_string();
    //             let json = xml_string_to_json(event, &Config::new_with_defaults()).unwrap();
    //             println!("{:#?}", json.to_string());
    //         }
    //     }
    //     Err(e) => println!("Error: {}", e),
    // }
}
