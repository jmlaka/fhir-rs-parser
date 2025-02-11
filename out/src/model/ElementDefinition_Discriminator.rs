#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Captures constraints on each element within the resource, profile, or extension.

#[derive(Debug)]
pub struct ElementDefinition_Discriminator<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ElementDefinition_Discriminator<'_> {
    pub fn new(value: &Value) -> ElementDefinition_Discriminator {
        ElementDefinition_Discriminator {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for path
    pub fn _path(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_path") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
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

    /// A FHIRPath expression, using [the simple subset of
    /// FHIRPath](fhirpath.html#simple), that is used to identify the element on which
    /// discrimination is based.
    pub fn path(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("path") {
            return Some(string);
        }
        return None;
    }

    /// How the element value is interpreted when discrimination is evaluated.
    pub fn fhir_type(&self) -> Option<ElementDefinition_DiscriminatorType> {
        match self.value.get("type") {
            Some(Value::String(val)) => ElementDefinition_DiscriminatorType::from_string(&val),
            _ => None,
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._path() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
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
        if let Some(_val) = self.path() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ElementDefinition_DiscriminatorBuilder {
    pub(crate) value: Value,
}

impl ElementDefinition_DiscriminatorBuilder {
    pub fn build(&self) -> ElementDefinition_Discriminator {
        ElementDefinition_Discriminator {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ElementDefinition_Discriminator,
    ) -> ElementDefinition_DiscriminatorBuilder {
        ElementDefinition_DiscriminatorBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ElementDefinition_DiscriminatorBuilder {
        let mut __value: Value = json!({});
        return ElementDefinition_DiscriminatorBuilder { value: __value };
    }

    pub fn _path<'a>(&'a mut self, val: Element) -> &'a mut ElementDefinition_DiscriminatorBuilder {
        self.value["_path"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut ElementDefinition_DiscriminatorBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ElementDefinition_DiscriminatorBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_DiscriminatorBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ElementDefinition_DiscriminatorBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn path<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_DiscriminatorBuilder {
        self.value["path"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: ElementDefinition_DiscriminatorType,
    ) -> &'a mut ElementDefinition_DiscriminatorBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum ElementDefinition_DiscriminatorType {
    Value,
    Exists,
    Pattern,
    FhirType,
    Profile,
}

impl ElementDefinition_DiscriminatorType {
    pub fn from_string(string: &str) -> Option<ElementDefinition_DiscriminatorType> {
        match string {
            "value" => Some(ElementDefinition_DiscriminatorType::Value),
            "exists" => Some(ElementDefinition_DiscriminatorType::Exists),
            "pattern" => Some(ElementDefinition_DiscriminatorType::Pattern),
            "type" => Some(ElementDefinition_DiscriminatorType::FhirType),
            "profile" => Some(ElementDefinition_DiscriminatorType::Profile),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ElementDefinition_DiscriminatorType::Value => "value".to_string(),
            ElementDefinition_DiscriminatorType::Exists => "exists".to_string(),
            ElementDefinition_DiscriminatorType::Pattern => "pattern".to_string(),
            ElementDefinition_DiscriminatorType::FhirType => "type".to_string(),
            ElementDefinition_DiscriminatorType::Profile => "profile".to_string(),
        }
    }
}
