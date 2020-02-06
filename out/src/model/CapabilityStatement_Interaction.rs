#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatement_Interaction {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Coded identifier of the operation, supported by the system resource.
  code: Option<CapabilityStatement_InteractionCode>,

  /// Extensions for code
  #[serde(rename = "_code")]
  _code: Option<Element>,

  /// Guidance specific to the implementation of this operation, such as 'delete is a
  /// logical delete' or 'updates are only allowed with version id' or 'creates
  /// permitted from pre-authorized certificates only'.
  documentation: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

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

  /// Extensions for documentation
  #[serde(rename = "_documentation")]
  _documentation: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CapabilityStatement_InteractionCode {
  #[serde(rename = "read")]
  Read,

  #[serde(rename = "vread")]
  Vread,

  #[serde(rename = "update")]
  Update,

  #[serde(rename = "patch")]
  Patch,

  #[serde(rename = "delete")]
  Delete,

  #[serde(rename = "history-instance")]
  HistoryInstance,

  #[serde(rename = "history-type")]
  HistoryType,

  #[serde(rename = "create")]
  Create,

  #[serde(rename = "search-type")]
  SearchType,

}
