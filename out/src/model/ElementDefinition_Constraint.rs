#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use serde_json::value::Value;



/// Captures constraints on each element within the resource, profile, or extension.

#[derive(Debug)]
pub struct ElementDefinition_Constraint<'a> {
  pub value: &'a Value,
}

impl ElementDefinition_Constraint<'_> {
  /// Extensions for key
  pub fn _key(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_key") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Identifies the impact constraint violation has on the conformance of the
  /// instance.
  pub fn severity(&self) -> Option<ElementDefinition_ConstraintSeverity> {
    if let Some(Value::String(val)) = self.value.get("severity") {
      return Some(ElementDefinition_ConstraintSeverity::from_string(&val).unwrap());
    }
    return None;
  }

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A [FHIRPath](fhirpath.html) expression of constraint that can be executed to see
  /// if this constraint is met.
  pub fn expression(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("expression") {
      return Some(string.to_string());
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

  /// Allows identification of which elements have their cardinalities impacted by the
  /// constraint.  Will not be referenced for constraints that do not affect
  /// cardinality.
  pub fn key(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("key") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for human
  pub fn _human(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_human") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for expression
  pub fn _expression(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_expression") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// An XPath expression of constraint that can be executed to see if this constraint
  /// is met.
  pub fn xpath(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("xpath") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for severity
  pub fn _severity(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_severity") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for xpath
  pub fn _xpath(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_xpath") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Text that can be used to describe the constraint in messages identifying that
  /// the constraint has been violated.
  pub fn human(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("human") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for requirements
  pub fn _requirements(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_requirements") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A reference to the original source of the constraint, for traceability purposes.
  pub fn source(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("source") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Description of why this constraint is necessary or appropriate.
  pub fn requirements(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("requirements") {
      return Some(string.to_string());
    }
    return None;
  }

}

#[derive(Debug)]
pub enum ElementDefinition_ConstraintSeverity {
  Error,
  Warning,
}

impl ElementDefinition_ConstraintSeverity {
    pub fn from_string(string: &str) -> Option<ElementDefinition_ConstraintSeverity> {
      match string {
        "error" => Some(ElementDefinition_ConstraintSeverity::Error),
        "warning" => Some(ElementDefinition_ConstraintSeverity::Warning),
        _ => None,
    }
  }
}

