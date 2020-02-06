#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMap_Parameter {
  /// Extensions for valueId
  #[serde(rename = "_valueId")]
  _value_id: Option<Element>,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Option<Element>,

  /// Parameter value - variable or literal.
  #[serde(rename = "valueId")]
  value_id: Option<String>,

  /// Parameter value - variable or literal.
  #[serde(rename = "valueBoolean")]
  value_boolean: Option<bool>,

  /// Parameter value - variable or literal.
  #[serde(rename = "valueString")]
  value_string: Option<String>,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Option<Element>,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Option<Element>,

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
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Parameter value - variable or literal.
  #[serde(rename = "valueInteger")]
  value_integer: Option<i32>,

  /// Parameter value - variable or literal.
  #[serde(rename = "valueDecimal")]
  value_decimal: Option<i32>,

}
