#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataRequirement_Sort {
  /// The direction of the sort, ascending or descending.
  direction: Option<DataRequirement_SortDirection>,

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

  /// Extensions for path
  #[serde(rename = "_path")]
  _path: Option<Element>,

  /// Extensions for direction
  #[serde(rename = "_direction")]
  _direction: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// The attribute of the sort. The specified path must be resolvable from the type
  /// of the required data. The path is allowed to contain qualifiers (.) to traverse
  /// sub-elements, as well as indexers ([x]) to traverse multiple-cardinality sub-
  /// elements. Note that the index must be an integer constant.
  path: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataRequirement_SortDirection {
  #[serde(rename = "ascending")]
  Ascending,

  #[serde(rename = "descending")]
  Descending,

}
