/* This file is auto generated! Do not edit. */
/*!
Module for the {{ name }} telemetry messages.
*/
use std::io::Cursor;
#[allow(unused_imports)]
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use std::collections::HashMap;


/// {{ description }}
#[allow(non_camel_case_types)]
pub struct {{ fourcc }}_raw {
    {% for field in member %}
    /// {{ field.description }}
    pub {{ field.key | lower }}: {{ field.type }},

    {% endfor %}
}


/// {{ description }}
pub struct {{ fourcc }} {
    {% for field in member %}
    /// {{ field.description }} [{{ field.units.mks }}]
    pub {{ field.key | lower }}: f64,
    {% endfor %}
}


impl {{ fourcc }}_raw {
    /// Convert a {{ fourcc }} struct with converted units to a raw struct by applying scaleing backwards
    pub fn from_processed(processed: &{{ fourcc }}) -> {{ fourcc }}_raw {
        {{ fourcc }}_raw {
            {% for field in member %}
                {{ field.key | lower }}: (processed.{{ field.key | lower }} {% if field.units.scaleby %} / {{ field.units.scaleby }}{% endif %}) as {{ field.type }},
            {% endfor %}
        }
    }
}

impl {{ fourcc }} {

    /// Convert a {{ fourcc }}_raw struct to a converted units struct by applying scaleing
    pub fn from_raw(raw: {{ fourcc }}_raw) -> {{ fourcc }} {
        {{ fourcc }} {
            {% for field in member %}
            {{ field.key | lower }}: raw.{{ field.key | lower }} as f64 {% if field.units.scaleby %} * {{ field.units.scaleby }}{% endif %},
            {% endfor %}
        }
    }

    /// Convert values to a single line suitable for a CSV file
    /// (ex: "12.0,-15.2,1352.8123,")
    pub fn to_csv_line(&self) -> String {
        let mut line = String::new();

        {% for field in member %}
        line += &format!("{},", self.{{ field.key | lower }});
        {% endfor %}

        line
    }

    /// convert values to a dictionary where the key is a string
    /// (ex: {'value_1': 12.0, 'value_2': -15.2})
    pub fn to_dict(&self) -> HashMap<String, f64> {
        let mut map = HashMap::new();

        {% for field in member %}
        map.insert("{{ field.key }}".to_owned(), self.{{ field.key | lower }});
        {% endfor %}

        map
    }

    pub fn write_raw_bytes(&self, buffer: &mut Vec<u8>) {
        let raw = {{ fourcc }}_raw::from_processed(self);
        {% for field in member %}
        {% if field.type == "u8" %}
            buffer.write_{{ field.type }}(raw.{{ field.key | lower }}).unwrap();
        {% else %}
            buffer.write_{{ field.type }}::<BigEndian>(raw.{{ field.key | lower }}).unwrap();
        {% endif %}
        {% endfor %}
    }
}


/// Read the body and return unpacked struct of raw values
pub fn read_raw(body: Vec<u8>) -> {{ fourcc }}_raw {
    let mut cursor = Cursor::new(body);

    {{ fourcc }}_raw {
        {% for field in member %}
        {% if field.type == "u8" %}
            {{ field.key | lower }}: cursor.read_{{ field.type }}().unwrap(),
        {% else %}
            {{ field.key | lower}}: cursor.read_{{ field.type }}::<BigEndian>().unwrap(),
        {% endif %}
        {% endfor %}
    }
}
