#![allow(unused_imports, non_camel_case_types)]

use crate::model::ClaimResponse_Adjudication::ClaimResponse_Adjudication;
use crate::model::ClaimResponse_SubDetail::ClaimResponse_SubDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides the adjudication details from the processing of a Claim
/// resource.

#[derive(Debug)]
pub struct ClaimResponse_Detail<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ClaimResponse_Detail<'_> {
    pub fn new(value: &Value) -> ClaimResponse_Detail {
        ClaimResponse_Detail {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for detailSequence
    pub fn _detail_sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_detailSequence") {
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

    /// The adjudication results.
    pub fn adjudication(&self) -> Vec<ClaimResponse_Adjudication> {
        if let Some(val) = self.value.get("adjudication") {
            if let Some(arr) = val.as_array() {
                return arr
                    .into_iter()
                    .map(|e| ClaimResponse_Adjudication {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>();
            }
        }
        return Vec::new();
    }

    /// A number to uniquely reference the claim detail entry.
    pub fn detail_sequence(&self) -> Option<i64> {
        match self.value.get("detailSequence") {
            Some(val) => val.as_i64(),
            _ => None,
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

    /// A sub-detail adjudication of a simple product or service.
    pub fn sub_detail(&self) -> Option<Vec<ClaimResponse_SubDetail>> {
        if let Some(Value::Array(val)) = self.value.get("subDetail") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_SubDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._detail_sequence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._note_number() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self
            .adjudication()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.detail_sequence() {}
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
        if let Some(_val) = self.note_number() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.sub_detail() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ClaimResponse_DetailBuilder {
    pub(crate) value: Value,
}

impl ClaimResponse_DetailBuilder {
    pub fn build(&self) -> ClaimResponse_Detail {
        ClaimResponse_Detail {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ClaimResponse_Detail) -> ClaimResponse_DetailBuilder {
        ClaimResponse_DetailBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(adjudication: Vec<ClaimResponse_Adjudication>) -> ClaimResponse_DetailBuilder {
        let mut __value: Value = json!({});
        __value["adjudication"] = json!(adjudication
            .into_iter()
            .map(|e| e.value)
            .collect::<Vec<_>>());
        return ClaimResponse_DetailBuilder { value: __value };
    }

    pub fn _detail_sequence<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponse_DetailBuilder {
        self.value["_detailSequence"] = json!(val.value);
        return self;
    }

    pub fn _note_number<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ClaimResponse_DetailBuilder {
        self.value["_noteNumber"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn detail_sequence<'a>(&'a mut self, val: i64) -> &'a mut ClaimResponse_DetailBuilder {
        self.value["detailSequence"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ClaimResponse_DetailBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponse_DetailBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClaimResponse_DetailBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note_number<'a>(&'a mut self, val: Vec<i64>) -> &'a mut ClaimResponse_DetailBuilder {
        self.value["noteNumber"] = json!(val);
        return self;
    }

    pub fn sub_detail<'a>(
        &'a mut self,
        val: Vec<ClaimResponse_SubDetail>,
    ) -> &'a mut ClaimResponse_DetailBuilder {
        self.value["subDetail"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
