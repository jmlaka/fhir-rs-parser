#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::Ratio::Ratio;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.

#[derive(Debug)]
pub struct SubstanceSpecification_Relationship<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceSpecification_Relationship<'_> {
    pub fn new(value: &Value) -> SubstanceSpecification_Relationship {
        SubstanceSpecification_Relationship {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for amountString
    pub fn _amount_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_amountString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for isDefining
    pub fn _is_defining(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_isDefining") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A numeric factor for the relationship, for instance to express that the salt of
    /// a substance has some percentage of the active substance in relation to some
    /// other.
    pub fn amount_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("amountQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A numeric factor for the relationship, for instance to express that the salt of
    /// a substance has some percentage of the active substance in relation to some
    /// other.
    pub fn amount_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("amountRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A numeric factor for the relationship, for instance to express that the salt of
    /// a substance has some percentage of the active substance in relation to some
    /// other.
    pub fn amount_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("amountRatio") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// For use when the numeric.
    pub fn amount_ratio_low_limit(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("amountRatioLowLimit") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A numeric factor for the relationship, for instance to express that the salt of
    /// a substance has some percentage of the active substance in relation to some
    /// other.
    pub fn amount_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("amountString") {
            return Some(string);
        }
        return None;
    }

    /// An operator for the amount, for example "average", "approximately", "less than".
    pub fn amount_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("amountType") {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// For example where an enzyme strongly bonds with a particular substance, this is
    /// a defining relationship for that enzyme, out of several possible substance
    /// relationships.
    pub fn is_defining(&self) -> Option<bool> {
        match self.value.get("isDefining") {
            Some(val) => val.as_bool(),
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

    /// For example "salt to parent", "active moiety", "starting material".
    pub fn relationship(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("relationship") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Supporting literature.
    pub fn source(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("source") {
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

    /// A pointer to another substance, as a resource or just a representational code.
    pub fn substance_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("substanceCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A pointer to another substance, as a resource or just a representational code.
    pub fn substance_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("substanceReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._amount_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._is_defining() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount_ratio() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount_ratio_low_limit() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount_string() {}
        if let Some(_val) = self.amount_type() {
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
        if let Some(_val) = self.is_defining() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.relationship() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.source() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.substance_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.substance_reference() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceSpecification_RelationshipBuilder {
    pub(crate) value: Value,
}

impl SubstanceSpecification_RelationshipBuilder {
    pub fn build(&self) -> SubstanceSpecification_Relationship {
        SubstanceSpecification_Relationship {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: SubstanceSpecification_Relationship,
    ) -> SubstanceSpecification_RelationshipBuilder {
        SubstanceSpecification_RelationshipBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceSpecification_RelationshipBuilder {
        let mut __value: Value = json!({});
        return SubstanceSpecification_RelationshipBuilder { value: __value };
    }

    pub fn _amount_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["_amountString"] = json!(val.value);
        return self;
    }

    pub fn _is_defining<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["_isDefining"] = json!(val.value);
        return self;
    }

    pub fn amount_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["amountQuantity"] = json!(val.value);
        return self;
    }

    pub fn amount_range<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["amountRange"] = json!(val.value);
        return self;
    }

    pub fn amount_ratio<'a>(
        &'a mut self,
        val: Ratio,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["amountRatio"] = json!(val.value);
        return self;
    }

    pub fn amount_ratio_low_limit<'a>(
        &'a mut self,
        val: Ratio,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["amountRatioLowLimit"] = json!(val.value);
        return self;
    }

    pub fn amount_string<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["amountString"] = json!(val);
        return self;
    }

    pub fn amount_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["amountType"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn is_defining<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["isDefining"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn relationship<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["relationship"] = json!(val.value);
        return self;
    }

    pub fn source<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["source"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn substance_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["substanceCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn substance_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut SubstanceSpecification_RelationshipBuilder {
        self.value["substanceReference"] = json!(val.value);
        return self;
    }
}
