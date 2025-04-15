use super::coordinate_utils::{coordinate_string, decode_string_coordinates_decimal_minutes};
use crate::common::coords::Coordinates;
use crate::common::route::Route;
use crate::common::waypoint::{Offset, Waypoint};
pub struct RouteUtils;

impl RouteUtils {
    pub fn format_flight_state(state: &str) -> String {
        match state {
            "TO" => "Takeoff".to_string(),
            "IC" => "Initial Climb".to_string(),
            "CL" => "Climb".to_string(),
            "ER" => "En Route".to_string(),
            "DC" => "Descent".to_string(),
            "AP" => "Approach".to_string(),
            _ => format!("Unknown {}", state),
        }
    }

    pub fn route_to_string(route: &Route) -> String {
        let mut str = String::new();
        if let Some(name) = &route.name {
            str.push_str(name);
        }
        if let Some(runway) = &route.runway {
            str.push_str(&format!("({})", runway));
        }
        if !str.is_empty() && route.waypoints.len() == 1 {
            str.push_str(" starting at ");
        } else if !str.is_empty() {
            str.push_str(": ");
        }

        str.push_str(&RouteUtils::waypoints_to_string(&route.waypoints));
        str
    }

    pub fn waypoint_to_string(waypoint: &Waypoint) -> String {
        let mut s = waypoint.name.clone();
        if let (Some(lat), Some(lon)) = (waypoint.latitude, waypoint.longitude) {
            s.push_str(&format!(
                "({})",
                coordinate_string(Coordinates {
                    latitude: lat,
                    longitude: lon
                })
            ));
        }
        if let Some(offset) = &waypoint.offset {
            s.push_str(&format!("[{}° {}nm]", offset.bearing, offset.distance));
        }
        if let (Some(time), Some(time_format)) = (waypoint.time, &waypoint.time_format) {
            s.push_str(&format!(
                "@{}",
                DateTimeUtils::timestamp_to_string(time, time_format)
            ));
        }
        s
    }

    pub fn get_waypoint(leg: &str) -> Waypoint {
        let regex = regex::Regex::new(r"^([A-Z]+)(\d{3})-(\d{4})$").unwrap();
        if let Some(caps) = regex.captures(leg) {
            return Waypoint {
                name: caps[1].to_string(),
                offset: Some(Offset {
                    bearing: caps[2].parse().unwrap(),
                    distance: caps[3].parse::<f32>().unwrap() / 10.0,
                }),
                ..Default::default()
            };
        }

        let waypoint: Vec<&str> = leg.split(',').collect();
        if waypoint.len() == 2 {
            if let Some(position) =
                decode_string_coordinates_decimal_minutes(waypoint[1].to_owned())
            {
                return Waypoint {
                    name: waypoint[0].to_string(),
                    latitude: Some(position.latitude),
                    longitude: Some(position.longitude),
                    ..Default::default()
                };
            }
        }
        if leg.len() == 13 || leg.len() == 14 {
            if let Some(position) = decode_string_coordinates_decimal_minutes(leg.to_owned()) {
                let name = if waypoint.len() == 2 {
                    waypoint[0].to_string()
                } else {
                    String::new()
                };
                return Waypoint {
                    name,
                    latitude: Some(position.latitude),
                    longitude: Some(position.longitude),
                    ..Default::default()
                };
            }
        }
        Waypoint {
            name: leg.to_string(),
            ..Default::default()
        }
    }

    fn waypoints_to_string(waypoints: &[Waypoint]) -> String {
        let str: Vec<String> = waypoints
            .iter()
            .map(|x| RouteUtils::waypoint_to_string(x))
            .collect();
        let result = str.join(" > ").replace(">  >", ">>");
        if result.starts_with(" > ") {
            format!(">>{}", &result[2..])
        } else {
            result
        }
    }
}
