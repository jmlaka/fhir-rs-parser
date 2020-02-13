#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use serde_json::value::Value;



/// A summary of information based on the results of executing a TestScript.

#[derive(Debug)]
pub struct TestReport_Operation<'a> {
  pub value: &'a Value,
}

impl TestReport_Operation<'_> {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for result
  pub fn _result(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_result") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// An explanatory message associated with the result.
  pub fn message(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("message") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for message
  pub fn _message(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_message") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The result of this operation.
  pub fn result(&self) -> Option<TestReport_OperationResult> {
    if let Some(Value::String(val)) = self.value.get("result") {
      return Some(TestReport_OperationResult::from_string(&val).unwrap());
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
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
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
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A link to further details on the result.
  pub fn detail(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("detail") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for detail
  pub fn _detail(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_detail") {
      return Some(Element { value: val });
    }
    return None;
  }

}

#[derive(Debug)]
pub enum TestReport_OperationResult {
  Pass,
  Skip,
  Fail,
  Warning,
  Error,
}

impl TestReport_OperationResult {
    pub fn from_string(string: &str) -> Option<TestReport_OperationResult> {
      match string {
        "pass" => Some(TestReport_OperationResult::Pass),
        "skip" => Some(TestReport_OperationResult::Skip),
        "fail" => Some(TestReport_OperationResult::Fail),
        "warning" => Some(TestReport_OperationResult::Warning),
        "error" => Some(TestReport_OperationResult::Error),
        _ => None,
    }
  }
}

