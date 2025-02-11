#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Money::Money;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides the adjudication details from the processing of a Claim
/// resource.

#[derive(Debug)]
pub struct ClaimResponse_Adjudication<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ClaimResponse_Adjudication<'_> {
    pub fn new(value: &Value) -> ClaimResponse_Adjudication {
        ClaimResponse_Adjudication {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Monetary amount associated with the category.
    pub fn amount(&self) -> Option<Money> {
        if let Some(val) = self.value.get("amount") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code to indicate the information type of this adjudication record. Information
    /// types may include the value submitted, maximum values or percentages allowed or
    /// payable under the plan, amounts that: the patient is responsible for in
    /// aggregate or pertaining to this item; amounts paid by other coverages; and, the
    /// benefit payable for this item.
    pub fn category(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["category"]),
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

    /// A code supporting the understanding of the adjudication result and explaining
    /// variance from expected amount.
    pub fn reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("reason") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A non-monetary value associated with the category. Mutually exclusive to the
    /// amount element above.
    pub fn value(&self) -> Option<f64> {
        match self.value.get("value") {
            Some(val) => val.as_f64(),
            _ => None,
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._value() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.category().validate() {
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
        if let Some(_val) = self.reason() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ClaimResponse_AdjudicationBuilder {
    pub(crate) value: Value,
}

impl ClaimResponse_AdjudicationBuilder {
    pub fn build(&self) -> ClaimResponse_Adjudication {
        ClaimResponse_Adjudication {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ClaimResponse_Adjudication) -> ClaimResponse_AdjudicationBuilder {
        ClaimResponse_AdjudicationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(category: CodeableConcept) -> ClaimResponse_AdjudicationBuilder {
        let mut __value: Value = json!({});
        __value["category"] = json!(category.value);
        return ClaimResponse_AdjudicationBuilder { value: __value };
    }

    pub fn _value<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponse_AdjudicationBuilder {
        self.value["_value"] = json!(val.value);
        return self;
    }

    pub fn amount<'a>(&'a mut self, val: Money) -> &'a mut ClaimResponse_AdjudicationBuilder {
        self.value["amount"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClaimResponse_AdjudicationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponse_AdjudicationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClaimResponse_AdjudicationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ClaimResponse_AdjudicationBuilder {
        self.value["reason"] = json!(val.value);
        return self;
    }

    pub fn value<'a>(&'a mut self, val: f64) -> &'a mut ClaimResponse_AdjudicationBuilder {
        self.value["value"] = json!(val);
        return self;
    }
}
