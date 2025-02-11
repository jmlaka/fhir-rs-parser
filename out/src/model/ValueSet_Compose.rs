#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ValueSet_Include::ValueSet_Include;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [[[CodeSystem]]] definitions and their use in [coded
/// elements](terminologies.html).

#[derive(Debug)]
pub struct ValueSet_Compose<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ValueSet_Compose<'_> {
    pub fn new(value: &Value) -> ValueSet_Compose {
        ValueSet_Compose {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for inactive
    pub fn _inactive(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_inactive") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for lockedDate
    pub fn _locked_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lockedDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Exclude one or more codes from the value set based on code system filters and/or
    /// other value sets.
    pub fn exclude(&self) -> Option<Vec<ValueSet_Include>> {
        if let Some(Value::Array(val)) = self.value.get("exclude") {
            return Some(
                val.into_iter()
                    .map(|e| ValueSet_Include {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
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

    /// Whether inactive codes - codes that are not approved for current use - are in
    /// the value set. If inactive = true, inactive codes are to be included in the
    /// expansion, if inactive = false, the inactive codes will not be included in the
    /// expansion. If absent, the behavior is determined by the implementation, or by
    /// the applicable $expand parameters (but generally, inactive codes would be
    /// expected to be included).
    pub fn inactive(&self) -> Option<bool> {
        match self.value.get("inactive") {
            Some(val) => val.as_bool(),
            _ => None,
        }
    }

    /// Include one or more codes from a code system or other value set(s).
    pub fn include(&self) -> Vec<ValueSet_Include> {
        if let Some(val) = self.value.get("include") {
            if let Some(arr) = val.as_array() {
                return arr
                    .into_iter()
                    .map(|e| ValueSet_Include {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>();
            }
        }
        return Vec::new();
    }

    /// The Locked Date is  the effective date that is used to determine the version of
    /// all referenced Code Systems and Value Set Definitions included in the compose
    /// that are not already tied to a specific version.
    pub fn locked_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lockedDate") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
    pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._inactive() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._locked_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.exclude() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.inactive() {}
        if !self
            .include()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.locked_date() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ValueSet_ComposeBuilder {
    pub(crate) value: Value,
}

impl ValueSet_ComposeBuilder {
    pub fn build(&self) -> ValueSet_Compose {
        ValueSet_Compose {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ValueSet_Compose) -> ValueSet_ComposeBuilder {
        ValueSet_ComposeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(include: Vec<ValueSet_Include>) -> ValueSet_ComposeBuilder {
        let mut __value: Value = json!({});
        __value["include"] = json!(include.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return ValueSet_ComposeBuilder { value: __value };
    }

    pub fn _inactive<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_ComposeBuilder {
        self.value["_inactive"] = json!(val.value);
        return self;
    }

    pub fn _locked_date<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_ComposeBuilder {
        self.value["_lockedDate"] = json!(val.value);
        return self;
    }

    pub fn exclude<'a>(
        &'a mut self,
        val: Vec<ValueSet_Include>,
    ) -> &'a mut ValueSet_ComposeBuilder {
        self.value["exclude"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ValueSet_ComposeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_ComposeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn inactive<'a>(&'a mut self, val: bool) -> &'a mut ValueSet_ComposeBuilder {
        self.value["inactive"] = json!(val);
        return self;
    }

    pub fn locked_date<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_ComposeBuilder {
        self.value["lockedDate"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ValueSet_ComposeBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
