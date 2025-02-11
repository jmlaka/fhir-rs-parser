#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::ExplanationOfBenefit_Adjudication::ExplanationOfBenefit_Adjudication;
use crate::model::Extension::Extension;
use crate::model::Money::Money;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.

#[derive(Debug)]
pub struct ExplanationOfBenefit_SubDetail<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ExplanationOfBenefit_SubDetail<'_> {
    pub fn new(value: &Value) -> ExplanationOfBenefit_SubDetail {
        ExplanationOfBenefit_SubDetail {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for factor
    pub fn _factor(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_factor") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for noteNumber
    pub fn _note_number(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_noteNumber") {
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

    /// Extensions for sequence
    pub fn _sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sequence") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The adjudication results.
    pub fn adjudication(&self) -> Option<Vec<ExplanationOfBenefit_Adjudication>> {
        if let Some(Value::Array(val)) = self.value.get("adjudication") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Adjudication {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Code to identify the general type of benefits under which products and services
    /// are provided.
    pub fn category(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("category") {
            return Some(CodeableConcept {
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

    /// A real number that represents a multiplier used in determining the overall value
    /// of services delivered and/or goods received. The concept of a Factor allows for
    /// a discount or surcharge multiplier to be applied to a monetary amount.
    pub fn factor(&self) -> Option<f64> {
        match self.value.get("factor") {
            Some(val) => val.as_f64(),
            _ => None,
        }
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Item typification or modifiers codes to convey additional context for the
    /// product or service.
    pub fn modifier(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("modifier") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// The quantity times the unit price for an additional service or product or
    /// charge.
    pub fn net(&self) -> Option<Money> {
        if let Some(val) = self.value.get("net") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The numbers associated with notes below which apply to the adjudication of this
    /// item.
    pub fn note_number(&self) -> Option<Vec<i64>> {
        match self.value.get("noteNumber") {
            Some(Value::Array(val)) => Some(
                val.into_iter()
                    .filter_map(|e| e.as_i64())
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        }
    }

    /// When the value is a group code then this item collects a set of related claim
    /// details, otherwise this contains the product, service, drug or other billing
    /// code for the item.
    pub fn product_or_service(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["productOrService"]),
        }
    }

    /// Identifies the program under which this may be recovered.
    pub fn program_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("programCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The number of repetitions of a service or product.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of revenue or cost center providing the product and/or service.
    pub fn revenue(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("revenue") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A claim detail line. Either a simple (a product or service) or a 'group' of sub-
    /// details which are simple items.
    pub fn sequence(&self) -> Option<i64> {
        match self.value.get("sequence") {
            Some(val) => val.as_i64(),
            _ => None,
        }
    }

    /// Unique Device Identifiers associated with this line item.
    pub fn udi(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("udi") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// If the item is not a group then this is the fee for the product or service,
    /// otherwise this is the total of the fees for the details of the group.
    pub fn unit_price(&self) -> Option<Money> {
        if let Some(val) = self.value.get("unitPrice") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._factor() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._note_number() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._sequence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.adjudication() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.category() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.factor() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.net() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.note_number() {
            _val.into_iter().for_each(|_e| {});
        }
        if !self.product_or_service().validate() {
            return false;
        }
        if let Some(_val) = self.program_code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.revenue() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.sequence() {}
        if let Some(_val) = self.udi() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.unit_price() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ExplanationOfBenefit_SubDetailBuilder {
    pub(crate) value: Value,
}

impl ExplanationOfBenefit_SubDetailBuilder {
    pub fn build(&self) -> ExplanationOfBenefit_SubDetail {
        ExplanationOfBenefit_SubDetail {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ExplanationOfBenefit_SubDetail) -> ExplanationOfBenefit_SubDetailBuilder {
        ExplanationOfBenefit_SubDetailBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(product_or_service: CodeableConcept) -> ExplanationOfBenefit_SubDetailBuilder {
        let mut __value: Value = json!({});
        __value["productOrService"] = json!(product_or_service.value);
        return ExplanationOfBenefit_SubDetailBuilder { value: __value };
    }

    pub fn _factor<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["_factor"] = json!(val.value);
        return self;
    }

    pub fn _note_number<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["_noteNumber"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _sequence<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["_sequence"] = json!(val.value);
        return self;
    }

    pub fn adjudication<'a>(
        &'a mut self,
        val: Vec<ExplanationOfBenefit_Adjudication>,
    ) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["adjudication"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn category<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["category"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn factor<'a>(&'a mut self, val: f64) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["factor"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["modifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn net<'a>(&'a mut self, val: Money) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["net"] = json!(val.value);
        return self;
    }

    pub fn note_number<'a>(
        &'a mut self,
        val: Vec<i64>,
    ) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["noteNumber"] = json!(val);
        return self;
    }

    pub fn program_code<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["programCode"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn revenue<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["revenue"] = json!(val.value);
        return self;
    }

    pub fn sequence<'a>(&'a mut self, val: i64) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["sequence"] = json!(val);
        return self;
    }

    pub fn udi<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["udi"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn unit_price<'a>(
        &'a mut self,
        val: Money,
    ) -> &'a mut ExplanationOfBenefit_SubDetailBuilder {
        self.value["unitPrice"] = json!(val.value);
        return self;
    }
}
