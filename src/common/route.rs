use super::waypoint::Waypoint;

pub struct Route {
    pub name: Option<String>,
    pub runway: Option<String>,
    pub waypoints: Vec<Waypoint>,
}
