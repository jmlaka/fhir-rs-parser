#![allow(unused_imports, non_camel_case_types)]

use crate::model::ElementDefinition::ElementDefinition;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A definition of a FHIR structure. This resource is used to describe the
/// underlying resources, data types defined in FHIR, and also for describing
/// extensions and constraints on resources and data types.

#[derive(Debug)]
pub struct StructureDefinition_Snapshot<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl StructureDefinition_Snapshot<'_> {
    pub fn new(value: &Value) -> StructureDefinition_Snapshot {
        StructureDefinition_Snapshot {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Captures constraints on each element within the resource.
    pub fn element(&self) -> Vec<ElementDefinition> {
        if let Some(val) = self.value.get("element") {
            if let Some(arr) = val.as_array() {
                return arr
                    .into_iter()
                    .map(|e| ElementDefinition {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>();
            }
        }
        return Vec::new();
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

    pub fn validate(&self) -> bool {
        if !self
            .element()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
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
        return true;
    }
}

#[derive(Debug)]
pub struct StructureDefinition_SnapshotBuilder {
    pub(crate) value: Value,
}

impl StructureDefinition_SnapshotBuilder {
    pub fn build(&self) -> StructureDefinition_Snapshot {
        StructureDefinition_Snapshot {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: StructureDefinition_Snapshot) -> StructureDefinition_SnapshotBuilder {
        StructureDefinition_SnapshotBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(element: Vec<ElementDefinition>) -> StructureDefinition_SnapshotBuilder {
        let mut __value: Value = json!({});
        __value["element"] = json!(element.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return StructureDefinition_SnapshotBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut StructureDefinition_SnapshotBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinition_SnapshotBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut StructureDefinition_SnapshotBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
