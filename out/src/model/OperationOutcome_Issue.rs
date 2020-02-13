#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use serde_json::value::Value;



/// A collection of error, warning, or information messages that result from a
/// system action.

#[derive(Debug)]
pub struct OperationOutcome_Issue<'a> {
  pub value: &'a Value,
}

impl OperationOutcome_Issue<'_> {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Additional diagnostic information about the issue.
  pub fn diagnostics(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("diagnostics") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for expression
  pub fn _expression(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_expression") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Additional details about the error. This may be a text description of the error
  /// or a system code that identifies the error.
  pub fn details(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("details") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Indicates whether the issue indicates a variation from successful processing.
  pub fn severity(&self) -> Option<OperationOutcome_IssueSeverity> {
    if let Some(Value::String(val)) = self.value.get("severity") {
      return Some(OperationOutcome_IssueSeverity::from_string(&val).unwrap());
    }
    return None;
  }

  /// A [simple subset of FHIRPath](fhirpath.html#simple) limited to element names,
  /// repetition indicators and the default child accessor that identifies one of the
  /// elements in the resource that caused this issue to be raised.
  pub fn expression(&self) -> Option<Vec<String>> {
    if let Some(Value::Array(val)) = self.value.get("expression") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap().to_string()).collect::<Vec<_>>());
    }
    return None;
  }

  /// This element is deprecated because it is XML specific. It is replaced by
  /// issue.expression, which is format independent, and simpler to parse.     For
  /// resource issues, this will be a simple XPath limited to element names,
  /// repetition indicators and the default child accessor that identifies one of the
  /// elements in the resource that caused this issue to be raised.  For HTTP errors,
  /// will be "http." + the parameter name.
  pub fn location(&self) -> Option<Vec<String>> {
    if let Some(Value::Array(val)) = self.value.get("location") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap().to_string()).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for code
  pub fn _code(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_code") {
      return Some(Element { value: val });
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

  /// Extensions for diagnostics
  pub fn _diagnostics(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_diagnostics") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for location
  pub fn _location(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_location") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
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

  /// Extensions for severity
  pub fn _severity(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_severity") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Describes the type of the issue. The system that creates an OperationOutcome
  /// SHALL choose the most applicable code from the IssueType value set, and may
  /// additional provide its own code for the error in the details element.
  pub fn code(&self) -> Option<OperationOutcome_IssueCode> {
    if let Some(Value::String(val)) = self.value.get("code") {
      return Some(OperationOutcome_IssueCode::from_string(&val).unwrap());
    }
    return None;
  }

}

#[derive(Debug)]
pub enum OperationOutcome_IssueSeverity {
  Fatal,
  Error,
  Warning,
  Information,
}

impl OperationOutcome_IssueSeverity {
    pub fn from_string(string: &str) -> Option<OperationOutcome_IssueSeverity> {
      match string {
        "fatal" => Some(OperationOutcome_IssueSeverity::Fatal),
        "error" => Some(OperationOutcome_IssueSeverity::Error),
        "warning" => Some(OperationOutcome_IssueSeverity::Warning),
        "information" => Some(OperationOutcome_IssueSeverity::Information),
        _ => None,
    }
  }
}


#[derive(Debug)]
pub enum OperationOutcome_IssueCode {
  Invalid,
  Structure,
  Required,
  Value,
  Invariant,
  Security,
  Login,
  Unknown,
  Expired,
  Forbidden,
  Suppressed,
  Processing,
  NotSupported,
  Duplicate,
  MultipleMatches,
  NotFound,
  Deleted,
  TooLong,
  CodeInvalid,
  Extension,
  TooCostly,
  BusinessRule,
  Conflict,
  Transient,
  LockError,
  NoStore,
  Exception,
  Timeout,
  Incomplete,
  Throttled,
  Informational,
}

impl OperationOutcome_IssueCode {
    pub fn from_string(string: &str) -> Option<OperationOutcome_IssueCode> {
      match string {
        "invalid" => Some(OperationOutcome_IssueCode::Invalid),
        "structure" => Some(OperationOutcome_IssueCode::Structure),
        "required" => Some(OperationOutcome_IssueCode::Required),
        "value" => Some(OperationOutcome_IssueCode::Value),
        "invariant" => Some(OperationOutcome_IssueCode::Invariant),
        "security" => Some(OperationOutcome_IssueCode::Security),
        "login" => Some(OperationOutcome_IssueCode::Login),
        "unknown" => Some(OperationOutcome_IssueCode::Unknown),
        "expired" => Some(OperationOutcome_IssueCode::Expired),
        "forbidden" => Some(OperationOutcome_IssueCode::Forbidden),
        "suppressed" => Some(OperationOutcome_IssueCode::Suppressed),
        "processing" => Some(OperationOutcome_IssueCode::Processing),
        "not-supported" => Some(OperationOutcome_IssueCode::NotSupported),
        "duplicate" => Some(OperationOutcome_IssueCode::Duplicate),
        "multiple-matches" => Some(OperationOutcome_IssueCode::MultipleMatches),
        "not-found" => Some(OperationOutcome_IssueCode::NotFound),
        "deleted" => Some(OperationOutcome_IssueCode::Deleted),
        "too-long" => Some(OperationOutcome_IssueCode::TooLong),
        "code-invalid" => Some(OperationOutcome_IssueCode::CodeInvalid),
        "extension" => Some(OperationOutcome_IssueCode::Extension),
        "too-costly" => Some(OperationOutcome_IssueCode::TooCostly),
        "business-rule" => Some(OperationOutcome_IssueCode::BusinessRule),
        "conflict" => Some(OperationOutcome_IssueCode::Conflict),
        "transient" => Some(OperationOutcome_IssueCode::Transient),
        "lock-error" => Some(OperationOutcome_IssueCode::LockError),
        "no-store" => Some(OperationOutcome_IssueCode::NoStore),
        "exception" => Some(OperationOutcome_IssueCode::Exception),
        "timeout" => Some(OperationOutcome_IssueCode::Timeout),
        "incomplete" => Some(OperationOutcome_IssueCode::Incomplete),
        "throttled" => Some(OperationOutcome_IssueCode::Throttled),
        "informational" => Some(OperationOutcome_IssueCode::Informational),
        _ => None,
    }
  }
}

