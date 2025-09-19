use std::error::Error;

#[derive(Debug, Clone, Default)]
pub struct VehicleInfo {
    pub brand: Option<String>,
    pub model: Option<String>,
    pub variant: Option<String>,
}

fn extract_content_by_id(html: &str, id: &str) -> Option<String> {
    let id_pattern = format!("id=\"{id}\"");

    if let Some(id_pos) = html.find(&id_pattern) {
        if let Some(tag_end) = html[id_pos..].find('>') {
            let content_start = id_pos + tag_end + 1;

            if let Some(content_end) = html[content_start..].find('<') {
                let content_end = content_start + content_end;
                let content = html[content_start..content_end].trim();

                if !content.is_empty() {
                    return Some(content.to_string());
                }
            }
        }
    }

    None
}

pub async fn fetch_vehicle_info(license_plate: &str) -> Result<VehicleInfo, Box<dyn Error>> {
    let url = format!("https://www.nummerplade.net/nummerplade/{license_plate}.html");

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Ok(VehicleInfo::default());
    }

    let html = response.text().await?;

    let vehicle_info = VehicleInfo {
        brand: extract_content_by_id(&html, "maerke"),
        model: extract_content_by_id(&html, "model"),
        variant: extract_content_by_id(&html, "variant"),
    };

    Ok(vehicle_info)
}
