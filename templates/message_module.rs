/* This file is auto generated! Do not edit. */
{% for definition in defs %}
pub mod {{ definition.fourcc | lower }};
{% endfor %}
