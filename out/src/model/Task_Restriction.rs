#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;


/// A task to be performed.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task_Restriction {
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

  /// Over what time-period is fulfillment sought.
  period: Option<Period>,

  /// For requests that are targeted to more than on potential recipient/target, for
  /// whom is fulfillment sought?
  recipient: Option<Vec<Box<Reference>>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for repetitions
  #[serde(rename = "_repetitions")]
  _repetitions: Option<Element>,

  /// Indicates the number of times the requested action should occur.
  repetitions: Option<i32>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

}
