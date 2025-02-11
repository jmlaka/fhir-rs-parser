#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.

#[derive(Debug)]
pub struct CapabilityStatement_Document<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CapabilityStatement_Document<'_> {
    pub fn new(value: &Value) -> CapabilityStatement_Document {
        CapabilityStatement_Document {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for documentation
    pub fn _documentation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_documentation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for mode
    pub fn _mode(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_mode") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A description of how the application supports or uses the specified document
    /// profile.  For example, when documents are created, what action is taken with
    /// consumed documents, etc.
    pub fn documentation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("documentation") {
            return Some(string);
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

    /// Mode of this document declaration - whether an application is a producer or
    /// consumer.
    pub fn mode(&self) -> Option<CapabilityStatement_DocumentMode> {
        match self.value.get("mode") {
            Some(Value::String(val)) => CapabilityStatement_DocumentMode::from_string(&val),
            _ => None,
        }
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

    /// A profile on the document Bundle that constrains which resources are present,
    /// and their contents.
    pub fn profile(&self) -> &str {
        self.value.get("profile").unwrap().as_str().unwrap()
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._documentation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._mode() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.documentation() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.mode() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct CapabilityStatement_DocumentBuilder {
    pub(crate) value: Value,
}

impl CapabilityStatement_DocumentBuilder {
    pub fn build(&self) -> CapabilityStatement_Document {
        CapabilityStatement_Document {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CapabilityStatement_Document) -> CapabilityStatement_DocumentBuilder {
        CapabilityStatement_DocumentBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(profile: &str) -> CapabilityStatement_DocumentBuilder {
        let mut __value: Value = json!({});
        __value["profile"] = json!(profile);
        return CapabilityStatement_DocumentBuilder { value: __value };
    }

    pub fn _documentation<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CapabilityStatement_DocumentBuilder {
        self.value["_documentation"] = json!(val.value);
        return self;
    }

    pub fn _mode<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatement_DocumentBuilder {
        self.value["_mode"] = json!(val.value);
        return self;
    }

    pub fn documentation<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CapabilityStatement_DocumentBuilder {
        self.value["documentation"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CapabilityStatement_DocumentBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatement_DocumentBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn mode<'a>(
        &'a mut self,
        val: CapabilityStatement_DocumentMode,
    ) -> &'a mut CapabilityStatement_DocumentBuilder {
        self.value["mode"] = json!(val.to_string());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CapabilityStatement_DocumentBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum CapabilityStatement_DocumentMode {
    Producer,
    Consumer,
}

impl CapabilityStatement_DocumentMode {
    pub fn from_string(string: &str) -> Option<CapabilityStatement_DocumentMode> {
        match string {
            "producer" => Some(CapabilityStatement_DocumentMode::Producer),
            "consumer" => Some(CapabilityStatement_DocumentMode::Consumer),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            CapabilityStatement_DocumentMode::Producer => "producer".to_string(),
            CapabilityStatement_DocumentMode::Consumer => "consumer".to_string(),
        }
    }
}
