#[derive(Debug, Clone, Default)]
pub struct Waypoint {
    pub name: String,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub time: Option<TimeFormat>,
    pub time_format: Option<TimeFormat>,
    pub offset: Option<Offset>,
}

#[derive(Debug, Clone, Default)]
pub enum TimeFormat {
    TOD,
    EPOCH,
    #[default]
    None,
}

#[derive(Debug, Clone, Default)]
pub struct Offset {
    pub bearing: f32,
    pub distance: f32,
}
