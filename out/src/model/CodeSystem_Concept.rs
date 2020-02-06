#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeSystem_Designation::CodeSystem_Designation;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CodeSystem_Property1::CodeSystem_Property1;


/// The CodeSystem resource is used to declare the existence of and describe a code
/// system or code system supplement and its key properties, and optionally define a
/// part or all of its content.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystem_Concept {
  /// A property value for this concept.
  property: Option<Vec<CodeSystem_Property1>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

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

  /// Extensions for display
  #[serde(rename = "_display")]
  _display: Option<Element>,

  /// The formal definition of the concept. The code system resource does not make
  /// formal definitions required, because of the prevalence of legacy systems.
  /// However, they are highly recommended, as without them there is no formal meaning
  /// associated with the concept.
  definition: Option<String>,

  /// Additional representations for the concept - other languages, aliases,
  /// specialized purposes, used for particular purposes, etc.
  designation: Option<Vec<CodeSystem_Designation>>,

  /// Extensions for code
  #[serde(rename = "_code")]
  _code: Option<Element>,

  /// A code - a text symbol - that uniquely identifies the concept within the code
  /// system.
  code: Option<String>,

  /// A human readable string that is the recommended default way to present this
  /// concept to a user.
  display: Option<String>,

  /// Defines children of a concept to produce a hierarchy of concepts. The nature of
  /// the relationships is variable (is-a/contains/categorizes) - see
  /// hierarchyMeaning.
  concept: Option<Vec<CodeSystem_Concept>>,

  /// Extensions for definition
  #[serde(rename = "_definition")]
  _definition: Option<Element>,

}
