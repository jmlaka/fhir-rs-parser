#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A duration of time during which an organism (or a process) has existed.

#[derive(Debug)]
pub struct Age<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Age<'_> {
    pub fn new(value: &Value) -> Age {
        Age {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for comparator
    pub fn _comparator(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comparator") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for system
    pub fn _system(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_system") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for unit
    pub fn _unit(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_unit") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A computer processable form of the unit in some unit representation system.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
        }
        return None;
    }

    /// How the value should be understood and represented - whether the actual value is
    /// greater or less than the stated value due to measurement issues; e.g. if the
    /// comparator is "<" , then the real value is < stated value.
    pub fn comparator(&self) -> Option<AgeComparator> {
        match self.value.get("comparator") {
            Some(Value::String(val)) => AgeComparator::from_string(&val),
            _ => None,
        }
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The identification of the system that provides the coded form of the unit.
    pub fn system(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("system") {
            return Some(string);
        }
        return None;
    }

    /// A human-readable form of the unit.
    pub fn unit(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("unit") {
            return Some(string);
        }
        return None;
    }

    /// The value of the measured amount. The value includes an implicit precision in
    /// the presentation of the value.
    pub fn value(&self) -> Option<f64> {
        match self.value.get("value") {
            Some(val) => val.as_f64(),
            _ => None,
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._comparator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._system() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._unit() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.comparator() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.system() {}
        if let Some(_val) = self.unit() {}
        if let Some(_val) = self.value() {}
        return true;
    }
}

#[derive(Debug)]
pub struct AgeBuilder {
    pub(crate) value: Value,
}

impl AgeBuilder {
    pub fn build(&self) -> Age {
        Age {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Age) -> AgeBuilder {
        AgeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> AgeBuilder {
        let mut __value: Value = json!({});
        return AgeBuilder { value: __value };
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut AgeBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn _comparator<'a>(&'a mut self, val: Element) -> &'a mut AgeBuilder {
        self.value["_comparator"] = json!(val.value);
        return self;
    }

    pub fn _system<'a>(&'a mut self, val: Element) -> &'a mut AgeBuilder {
        self.value["_system"] = json!(val.value);
        return self;
    }

    pub fn _unit<'a>(&'a mut self, val: Element) -> &'a mut AgeBuilder {
        self.value["_unit"] = json!(val.value);
        return self;
    }

    pub fn _value<'a>(&'a mut self, val: Element) -> &'a mut AgeBuilder {
        self.value["_value"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: &str) -> &'a mut AgeBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn comparator<'a>(&'a mut self, val: AgeComparator) -> &'a mut AgeBuilder {
        self.value["comparator"] = json!(val.to_string());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AgeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AgeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn system<'a>(&'a mut self, val: &str) -> &'a mut AgeBuilder {
        self.value["system"] = json!(val);
        return self;
    }

    pub fn unit<'a>(&'a mut self, val: &str) -> &'a mut AgeBuilder {
        self.value["unit"] = json!(val);
        return self;
    }

    pub fn value<'a>(&'a mut self, val: f64) -> &'a mut AgeBuilder {
        self.value["value"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum AgeComparator {
    LessThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    GreaterThan,
}

impl AgeComparator {
    pub fn from_string(string: &str) -> Option<AgeComparator> {
        match string {
            "<" => Some(AgeComparator::LessThan),
            "<=" => Some(AgeComparator::LessThanOrEqual),
            ">=" => Some(AgeComparator::GreaterThanOrEqual),
            ">" => Some(AgeComparator::GreaterThan),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            AgeComparator::LessThan => "<".to_string(),
            AgeComparator::LessThanOrEqual => "<=".to_string(),
            AgeComparator::GreaterThanOrEqual => ">=".to_string(),
            AgeComparator::GreaterThan => ">".to_string(),
        }
    }
}
