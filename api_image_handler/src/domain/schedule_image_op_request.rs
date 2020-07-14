use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleImageOpBody {
    pub image_base64: String,
}

#[derive(Clone, Debug)]
pub struct ScheduleImageRequest {
    pub id: String,
    pub image_base64: String,
    pub operation: String,
}
