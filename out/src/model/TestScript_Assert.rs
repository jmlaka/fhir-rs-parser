#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.

#[derive(Debug)]
pub struct TestScript_Assert<'a> {
  pub value: &'a Value,
}

impl TestScript_Assert<'_> {
  /// Extensions for compareToSourceId
  pub fn _compare_to_source_id(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_compareToSourceId") {
      return Some(Element { value: val });
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

  /// Extensions for warningOnly
  pub fn _warning_only(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_warningOnly") {
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

  /// The FHIRPath expression to be evaluated against the request or response message
  /// contents - HTTP headers and payload.
  pub fn expression(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("expression") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The HTTP header field name e.g. 'Location'.
  pub fn header_field(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("headerField") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for direction
  pub fn _direction(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_direction") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The type of the resource.  See http://build.fhir.org/resourcelist.html.
  pub fn resource(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("resource") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for resource
  pub fn _resource(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_resource") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for value
  pub fn _value(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_value") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The FHIRPath expression to evaluate against the source fixture. When
  /// compareToSourceId is defined, either compareToSourceExpression or
  /// compareToSourcePath must be defined, but not both.
  pub fn compare_to_source_expression(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("compareToSourceExpression") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for compareToSourcePath
  pub fn _compare_to_source_path(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_compareToSourcePath") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The label would be used for tracking/logging purposes by test engines.
  pub fn label(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("label") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The ID of a fixture.  Asserts that the response contains at a minimum the
  /// fixture specified by minimumId.
  pub fn minimum_id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("minimumId") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for path
  pub fn _path(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_path") {
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

  /// The request method or HTTP operation code to compare against that used by the
  /// client system under test.
  pub fn request_method(&self) -> Option<TestScript_AssertRequestMethod> {
    if let Some(Value::String(val)) = self.value.get("requestMethod") {
      return Some(TestScript_AssertRequestMethod::from_string(&val).unwrap());
    }
    return None;
  }

  /// The value to use in a comparison against the request URL path string.
  pub fn request_u_r_l(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("requestURL") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for responseCode
  pub fn _response_code(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_responseCode") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for description
  pub fn _description(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_description") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for contentType
  pub fn _content_type(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_contentType") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Whether or not the test execution will produce a warning only on error for this
  /// assert.
  pub fn warning_only(&self) -> Option<bool> {
    if let Some(val) = self.value.get("warningOnly") {
      return Some(val.as_bool().unwrap());
    }
    return None;
  }

  /// The ID of the Profile to validate against.
  pub fn validate_profile_id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("validateProfileId") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Whether or not the test execution performs validation on the bundle navigation
  /// links.
  pub fn navigation_links(&self) -> Option<bool> {
    if let Some(val) = self.value.get("navigationLinks") {
      return Some(val.as_bool().unwrap());
    }
    return None;
  }

  /// The operator type defines the conditional behavior of the assert. If not
  /// defined, the default is equals.
  pub fn operator(&self) -> Option<TestScript_AssertOperator> {
    if let Some(Value::String(val)) = self.value.get("operator") {
      return Some(TestScript_AssertOperator::from_string(&val).unwrap());
    }
    return None;
  }

  /// The value of the HTTP response code to be tested.
  pub fn response_code(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("responseCode") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for validateProfileId
  pub fn _validate_profile_id(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_validateProfileId") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The mime-type contents to compare against the request or response message
  /// 'Content-Type' header.
  pub fn content_type(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("contentType") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for headerField
  pub fn _header_field(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_headerField") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for minimumId
  pub fn _minimum_id(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_minimumId") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for navigationLinks
  pub fn _navigation_links(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_navigationLinks") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// okay | created | noContent | notModified | bad | forbidden | notFound |
  /// methodNotAllowed | conflict | gone | preconditionFailed | unprocessable.
  pub fn response(&self) -> Option<TestScript_AssertResponse> {
    if let Some(Value::String(val)) = self.value.get("response") {
      return Some(TestScript_AssertResponse::from_string(&val).unwrap());
    }
    return None;
  }

  /// Extensions for compareToSourceExpression
  pub fn _compare_to_source_expression(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_compareToSourceExpression") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for requestMethod
  pub fn _request_method(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_requestMethod") {
      return Some(Element { value: val });
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

  /// The description would be used by test engines for tracking and reporting
  /// purposes.
  pub fn description(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("description") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for requestURL
  pub fn _request_u_r_l(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_requestURL") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for sourceId
  pub fn _source_id(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_sourceId") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The value to compare to.
  pub fn value(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("value") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The direction to use for the assertion.
  pub fn direction(&self) -> Option<TestScript_AssertDirection> {
    if let Some(Value::String(val)) = self.value.get("direction") {
      return Some(TestScript_AssertDirection::from_string(&val).unwrap());
    }
    return None;
  }

  /// Id of the source fixture used as the contents to be evaluated by either the
  /// "source/expression" or "sourceId/path" definition.
  pub fn compare_to_source_id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("compareToSourceId") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Fixture to evaluate the XPath/JSONPath expression or the headerField  against.
  pub fn source_id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("sourceId") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for label
  pub fn _label(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_label") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// XPath or JSONPath expression to evaluate against the source fixture. When
  /// compareToSourceId is defined, either compareToSourceExpression or
  /// compareToSourcePath must be defined, but not both.
  pub fn compare_to_source_path(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("compareToSourcePath") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for operator
  pub fn _operator(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_operator") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for response
  pub fn _response(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_response") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The XPath or JSONPath expression to be evaluated against the fixture
  /// representing the response received from server.
  pub fn path(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("path") {
      return Some(string.to_string());
    }
    return None;
  }

}

#[derive(Debug)]
pub enum TestScript_AssertRequestMethod {
  Delete,
  Get,
  Options,
  Patch,
  Post,
  Put,
  Head,
}

impl TestScript_AssertRequestMethod {
    pub fn from_string(string: &str) -> Option<TestScript_AssertRequestMethod> {
      match string {
        "delete" => Some(TestScript_AssertRequestMethod::Delete),
        "get" => Some(TestScript_AssertRequestMethod::Get),
        "options" => Some(TestScript_AssertRequestMethod::Options),
        "patch" => Some(TestScript_AssertRequestMethod::Patch),
        "post" => Some(TestScript_AssertRequestMethod::Post),
        "put" => Some(TestScript_AssertRequestMethod::Put),
        "head" => Some(TestScript_AssertRequestMethod::Head),
        _ => None,
    }
  }
}


#[derive(Debug)]
pub enum TestScript_AssertOperator {
  Equals,
  NotEquals,
  In,
  NotIn,
  GreaterThan,
  LessThan,
  Empty,
  NotEmpty,
  Contains,
  NotContains,
  Eval,
}

impl TestScript_AssertOperator {
    pub fn from_string(string: &str) -> Option<TestScript_AssertOperator> {
      match string {
        "equals" => Some(TestScript_AssertOperator::Equals),
        "notEquals" => Some(TestScript_AssertOperator::NotEquals),
        "in" => Some(TestScript_AssertOperator::In),
        "notIn" => Some(TestScript_AssertOperator::NotIn),
        "greaterThan" => Some(TestScript_AssertOperator::GreaterThan),
        "lessThan" => Some(TestScript_AssertOperator::LessThan),
        "empty" => Some(TestScript_AssertOperator::Empty),
        "notEmpty" => Some(TestScript_AssertOperator::NotEmpty),
        "contains" => Some(TestScript_AssertOperator::Contains),
        "notContains" => Some(TestScript_AssertOperator::NotContains),
        "eval" => Some(TestScript_AssertOperator::Eval),
        _ => None,
    }
  }
}


#[derive(Debug)]
pub enum TestScript_AssertResponse {
  Okay,
  Created,
  NoContent,
  NotModified,
  Bad,
  Forbidden,
  NotFound,
  MethodNotAllowed,
  Conflict,
  Gone,
  PreconditionFailed,
  Unprocessable,
}

impl TestScript_AssertResponse {
    pub fn from_string(string: &str) -> Option<TestScript_AssertResponse> {
      match string {
        "okay" => Some(TestScript_AssertResponse::Okay),
        "created" => Some(TestScript_AssertResponse::Created),
        "noContent" => Some(TestScript_AssertResponse::NoContent),
        "notModified" => Some(TestScript_AssertResponse::NotModified),
        "bad" => Some(TestScript_AssertResponse::Bad),
        "forbidden" => Some(TestScript_AssertResponse::Forbidden),
        "notFound" => Some(TestScript_AssertResponse::NotFound),
        "methodNotAllowed" => Some(TestScript_AssertResponse::MethodNotAllowed),
        "conflict" => Some(TestScript_AssertResponse::Conflict),
        "gone" => Some(TestScript_AssertResponse::Gone),
        "preconditionFailed" => Some(TestScript_AssertResponse::PreconditionFailed),
        "unprocessable" => Some(TestScript_AssertResponse::Unprocessable),
        _ => None,
    }
  }
}


#[derive(Debug)]
pub enum TestScript_AssertDirection {
  Response,
  Request,
}

impl TestScript_AssertDirection {
    pub fn from_string(string: &str) -> Option<TestScript_AssertDirection> {
      match string {
        "response" => Some(TestScript_AssertDirection::Response),
        "request" => Some(TestScript_AssertDirection::Request),
        _ => None,
    }
  }
}

