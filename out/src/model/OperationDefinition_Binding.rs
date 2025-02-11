#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).

#[derive(Debug)]
pub struct OperationDefinition_Binding<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl OperationDefinition_Binding<'_> {
    pub fn new(value: &Value) -> OperationDefinition_Binding {
        OperationDefinition_Binding {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for strength
    pub fn _strength(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_strength") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Indicates the degree of conformance expectations associated with this binding -
    /// that is, the degree to which the provided value set must be adhered to in the
    /// instances.
    pub fn strength(&self) -> Option<OperationDefinition_BindingStrength> {
        match self.value.get("strength") {
            Some(Value::String(val)) => OperationDefinition_BindingStrength::from_string(&val),
            _ => None,
        }
    }

    /// Points to the value set or external definition (e.g. implicit value set) that
    /// identifies the set of codes to be used.
    pub fn value_set(&self) -> &str {
        self.value.get("valueSet").unwrap().as_str().unwrap()
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._strength() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.strength() {}
        return true;
    }
}

#[derive(Debug)]
pub struct OperationDefinition_BindingBuilder {
    pub(crate) value: Value,
}

impl OperationDefinition_BindingBuilder {
    pub fn build(&self) -> OperationDefinition_Binding {
        OperationDefinition_Binding {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: OperationDefinition_Binding) -> OperationDefinition_BindingBuilder {
        OperationDefinition_BindingBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(value_set: &str) -> OperationDefinition_BindingBuilder {
        let mut __value: Value = json!({});
        __value["valueSet"] = json!(value_set);
        return OperationDefinition_BindingBuilder { value: __value };
    }

    pub fn _strength<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinition_BindingBuilder {
        self.value["_strength"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut OperationDefinition_BindingBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinition_BindingBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut OperationDefinition_BindingBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn strength<'a>(
        &'a mut self,
        val: OperationDefinition_BindingStrength,
    ) -> &'a mut OperationDefinition_BindingBuilder {
        self.value["strength"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum OperationDefinition_BindingStrength {
    Required,
    Extensible,
    Preferred,
    Example,
}

impl OperationDefinition_BindingStrength {
    pub fn from_string(string: &str) -> Option<OperationDefinition_BindingStrength> {
        match string {
            "required" => Some(OperationDefinition_BindingStrength::Required),
            "extensible" => Some(OperationDefinition_BindingStrength::Extensible),
            "preferred" => Some(OperationDefinition_BindingStrength::Preferred),
            "example" => Some(OperationDefinition_BindingStrength::Example),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            OperationDefinition_BindingStrength::Required => "required".to_string(),
            OperationDefinition_BindingStrength::Extensible => "extensible".to_string(),
            OperationDefinition_BindingStrength::Preferred => "preferred".to_string(),
            OperationDefinition_BindingStrength::Example => "example".to_string(),
        }
    }
}
