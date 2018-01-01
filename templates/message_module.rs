/* This file is auto generated! Do not edit. */
{% for definition in defs %}
pub mod {{ definition.fourcc | lower }};
{% endfor %}

use std::collections::HashMap;

/// Attempt to read Message body based on it's fourcc and convert to a CSV line of scaled, unit-converted values
pub fn to_csv(fourcc: [u8; 4], body: Vec<u8>) -> Option<String> {
    match &fourcc {
        {% for definition in defs %}
        b"{{ definition.fourcc }}" => {
            let data_raw = {{ definition.fourcc | lower }}::read_raw(body);
            let data_scaled = {{ definition.fourcc | lower }}::{{ definition.fourcc }}::from_raw(data_raw);
            Some(data_scaled.to_csv_line())
        },
        {% endfor %}
        _ => {
            println!("Skipped type {}", String::from_utf8_lossy(&fourcc));
            None
        },
    }
}


/// Attempt to read Message body based on it's fourcc and convert to a map of scaled, unit-converted values
pub fn to_values(fourcc: [u8; 4], body: Vec<u8>) -> Option<HashMap<String, f64>> {
    match &fourcc {
        {% for definition in defs %}
        b"{{ definition.fourcc }}" => {
            let data_raw = {{ definition.fourcc | lower }}::read_raw(body);
            let data_scaled = {{ definition.fourcc | lower }}::{{ definition.fourcc }}::from_raw(data_raw);
            Some(data_scaled.to_dict())
        },
        {% endfor %}
        _ => {
            println!("Skipped type {}", String::from_utf8_lossy(&fourcc));
            None
        },
    }
}
