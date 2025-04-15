use crate::common::coords::Coordinates;

pub(crate) fn decode_string_coordinates(coords: String) -> Option<Coordinates> {
    // format: N12345W123456 or N12345 W123456

    let coords_as_slice = coords.as_bytes();

    if coords_as_slice.len() < 13 {
        return None;
    }

    let first_char = coords_as_slice[0];
    let mut middle_char = coords_as_slice[6];
    let mut longitude_chars = coords_as_slice[7..13].to_vec();

    if middle_char == b' ' {
        if coords_as_slice.len() < 14 {
            return None;
        }

        middle_char = coords_as_slice[7];
        longitude_chars = coords_as_slice[8..14].to_vec();
    }

    if (first_char == b'N' || first_char == b'S') && (middle_char == b'E' || middle_char == b'W') {
        let lat = String::from_utf8_lossy(&coords_as_slice[1..6]);
        let lon = String::from_utf8_lossy(&longitude_chars);

        let lat = lat.parse::<f32>().ok()? / 1000.0;
        let lon = lon.parse::<f32>().ok()? / 1000.0;

        let lat = lat * get_direction(first_char as char);
        let lon = lon * get_direction(middle_char as char);

        Some(Coordinates {
            latitude: lat,
            longitude: lon,
        })
    } else {
        None
    }
}

pub(crate) fn decode_string_coordinates_decimal_minutes(coords: String) -> Option<Coordinates> {
    // format: N12345W123456 or N12345 W123456

    let coords_as_slice = coords.as_bytes();
    if coords_as_slice.len() < 13 {
        return None;
    }

    let first_char = coords_as_slice[0];
    let mut middle_char = coords_as_slice[6];
    let mut longitude_chars = coords_as_slice[7..13].to_vec();

    if middle_char == b' ' {
        if coords_as_slice.len() < 14 {
            return None;
        }

        middle_char = coords_as_slice[7];
        longitude_chars = coords_as_slice[8..14].to_vec();
    }

    let lat_deg = (String::from_utf8_lossy(&coords_as_slice[1..6])
        .parse::<f32>()
        .ok()?
        / 1000.0)
        .trunc();
    let lat_min = (String::from_utf8_lossy(&coords_as_slice[1..6])
        .parse::<f32>()
        .ok()?
        % 1000.0)
        / 10.0;

    let lon_deg = (String::from_utf8_lossy(&longitude_chars)
        .parse::<f32>()
        .ok()?
        / 1000.0)
        .trunc();
    let lon_min = (String::from_utf8_lossy(&longitude_chars)
        .parse::<f32>()
        .ok()?
        % 1000.0)
        / 10.0;

    if (first_char == b'N' || first_char == b'S') && (middle_char == b'E' || middle_char == b'W') {
        let latitude = (lat_deg + (lat_min / 60.0)) * get_direction(first_char as char);
        let longitude = (lon_deg + (lon_min / 60.0)) * get_direction(middle_char as char);
        Some(Coordinates {
            latitude,
            longitude,
        })
    } else {
        None
    }
}

pub(crate) fn coordinate_string(coords: Coordinates) -> String {
    let lat_dir = if coords.latitude >= 0.0 { 'N' } else { 'S' };

    let lon_dir = if coords.longitude >= 0.0 { 'E' } else { 'W' };

    let lat = coords.latitude.abs();
    let lon = coords.longitude.abs();

    format!("{lat:.3} {lat_dir}, {lon:.3} {lon_dir}")
}

pub(crate) fn get_direction(coord: char) -> f32 {
    match coord {
        'E' | 'N' => 1.0,
        'W' | 'S' => -1.0,
        _ => 0.0,
    }
}

pub(crate) fn dms_to_decimal_degrees(degrees: f32, minutes: f32, seconds: f32) -> f32 {
    degrees + (minutes / 60.0) + (seconds / 3600.0)
}

#[cfg(test)]
mod tests {
    use crate::utils::float_comp::assert_almost_eq;

    use super::*;

    #[test]
    fn test_decode_string_coordinates() {
        let coords = "N12345W123456".to_string();
        let result = decode_string_coordinates(coords);
        assert!(result.is_some());
        let coords = result.unwrap();
        assert_almost_eq!(coords.latitude, 12.345, f32::EPSILON);
        assert_almost_eq!(coords.longitude, -123.456, f32::EPSILON);

        let coords = "N12345 W123456".to_string();
        let result = decode_string_coordinates(coords);
        assert!(result.is_some());
        let coords = result.unwrap();
        assert_almost_eq!(coords.latitude, 12.345, f32::EPSILON);
        assert_almost_eq!(coords.longitude, -123.456, f32::EPSILON);
    }

    #[test]
    fn test_coordinate_string() {
        let coords = Coordinates {
            latitude: 12.345,
            longitude: -123.456,
        };
        let result = coordinate_string(coords);
        assert_eq!(result, "12.345 N, 123.456 W");
    }

    #[test]
    fn test_decode_string_coordinates_dm() {
        let coords = "N12345W123456".to_string();
        let result = decode_string_coordinates_decimal_minutes(coords);
        assert!(result.is_some());
        let coords = result.unwrap();

        assert_almost_eq!(coords.latitude, 12.575, f32::EPSILON);
        assert_almost_eq!(coords.longitude, -123.76, f32::EPSILON);

        let coords = "N12345 W123456".to_string();
        let result = decode_string_coordinates_decimal_minutes(coords);
        assert!(result.is_some());
        let coords = result.unwrap();
        assert_almost_eq!(coords.latitude, 12.575, f32::EPSILON);
        assert_almost_eq!(coords.longitude, -123.76, f32::EPSILON);
    }
}
