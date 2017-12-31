/* This file is auto generated! Do not edit. */


/// {{ description }}
#[allow(non_camel_case_types)]
pub struct {{ fourcc }}_raw {
    {% for field in member %}
    /// {{ field.description }}
    pub {{ field.key | lower }}: {{ field.type }},

    {% endfor %}
}


/// {{ description }}
#[allow(non_camel_case_types)]
pub struct {{ fourcc }} {
    {% for field in member %}
    /// {{ field.description }} [{{ field.units.mks }}]
    pub {{ field.key | lower }}: f64,
    {% endfor %}
}


