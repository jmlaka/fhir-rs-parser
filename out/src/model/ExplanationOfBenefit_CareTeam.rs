#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.

#[derive(Debug)]
pub struct ExplanationOfBenefit_CareTeam<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ExplanationOfBenefit_CareTeam<'_> {
    /// Extensions for responsible
    pub fn _responsible(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_responsible") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for sequence
    pub fn _sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sequence") {
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

    /// Member of the team who provided the product or service.
    pub fn provider(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["provider"]),
        }
    }

    /// The qualification of the practitioner which is applicable for this service.
    pub fn qualification(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("qualification") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The party who is billing and/or responsible for the claimed products or
    /// services.
    pub fn responsible(&self) -> Option<bool> {
        if let Some(val) = self.value.get("responsible") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The lead, assisting or supervising practitioner and their discipline if a
    /// multidisciplinary team.
    pub fn role(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("role") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A number to uniquely identify care team entries.
    pub fn sequence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("sequence") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._responsible() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._sequence() {
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
        if !self.provider().validate() {
            return false;
        }
        if let Some(_val) = self.qualification() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.responsible() {}
        if let Some(_val) = self.role() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.sequence() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ExplanationOfBenefit_CareTeamBuilder {
    pub value: Value,
}

impl ExplanationOfBenefit_CareTeamBuilder {
    pub fn build(&self) -> ExplanationOfBenefit_CareTeam {
        ExplanationOfBenefit_CareTeam {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn new(provider: Reference) -> ExplanationOfBenefit_CareTeamBuilder {
        let mut __value: Value = json!({});
        __value["provider"] = json!(provider.value);
        return ExplanationOfBenefit_CareTeamBuilder { value: __value };
    }
}
