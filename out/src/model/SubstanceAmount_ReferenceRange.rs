#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Quantity::Quantity;
use crate::model::Extension::Extension;


/// Chemical substances are a single substance type whose primary defining element
/// is the molecular structure. Chemical substances shall be defined on the basis of
/// their complete covalent molecular structure; the presence of a salt (counter-
/// ion) and/or solvates (water, alcohols) is also captured. Purity, grade,
/// physical form or particle size are not taken into account in the definition of a
/// chemical substance or in the assignment of a Substance ID.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceAmount_ReferenceRange {
  /// Upper limit possible or expected.
  #[serde(rename = "highLimit")]
  high_limit: Option<Quantity>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

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

  /// Lower limit possible or expected.
  #[serde(rename = "lowLimit")]
  low_limit: Option<Quantity>,

}
