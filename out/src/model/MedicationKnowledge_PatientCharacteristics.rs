#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_PatientCharacteristics<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationKnowledge_PatientCharacteristics<'_> {
    pub fn new(value: &Value) -> MedicationKnowledge_PatientCharacteristics {
        MedicationKnowledge_PatientCharacteristics {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for value
    pub fn _value(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_value") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Specific characteristic that is relevant to the administration guideline (e.g.
    /// height, weight, gender).
    pub fn characteristic_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("characteristicCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specific characteristic that is relevant to the administration guideline (e.g.
    /// height, weight, gender).
    pub fn characteristic_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("characteristicQuantity") {
            return Some(Quantity {
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

    /// The specific characteristic (e.g. height, weight, gender, etc.).
    pub fn value(&self) -> Option<Vec<&str>> {
        match self.value.get("value") {
            Some(Value::Array(val)) => Some(
                val.into_iter()
                    .filter_map(|e| e.as_str())
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._value() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.characteristic_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.characteristic_quantity() {
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
        if let Some(_val) = self.value() {
            _val.into_iter().for_each(|_e| {});
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationKnowledge_PatientCharacteristicsBuilder {
    pub(crate) value: Value,
}

impl MedicationKnowledge_PatientCharacteristicsBuilder {
    pub fn build(&self) -> MedicationKnowledge_PatientCharacteristics {
        MedicationKnowledge_PatientCharacteristics {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicationKnowledge_PatientCharacteristics,
    ) -> MedicationKnowledge_PatientCharacteristicsBuilder {
        MedicationKnowledge_PatientCharacteristicsBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicationKnowledge_PatientCharacteristicsBuilder {
        let mut __value: Value = json!({});
        return MedicationKnowledge_PatientCharacteristicsBuilder { value: __value };
    }

    pub fn _value<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut MedicationKnowledge_PatientCharacteristicsBuilder {
        self.value["_value"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn characteristic_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationKnowledge_PatientCharacteristicsBuilder {
        self.value["characteristicCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn characteristic_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut MedicationKnowledge_PatientCharacteristicsBuilder {
        self.value["characteristicQuantity"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_PatientCharacteristicsBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicationKnowledge_PatientCharacteristicsBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_PatientCharacteristicsBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn value<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut MedicationKnowledge_PatientCharacteristicsBuilder {
        self.value["value"] = json!(val);
        return self;
    }
}
