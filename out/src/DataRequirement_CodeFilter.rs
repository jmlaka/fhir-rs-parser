use serde::{Deserialize, Serialize};

/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DataRequirement_CodeFilter {
  /// Extensions for searchParam
  #[serde(rename = "_searchParam")]
  _search_param: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The valueset for the code filter. The valueSet and code elements are additive.
  /// If valueSet is specified, the filter will return only those data items for which
  /// the value of the code-valued element specified in the path is a member of the
  /// specified valueset.
  #[serde(rename = "valueSet")]
  value_set: canonical,

  /// The codes for the code filter. If values are given, the filter will return only
  /// those data items for which the code-valued attribute specified by the path has a
  /// value that is one of the specified codes. If codes are specified in addition to
  /// a value set, the filter returns items matching a code in the value set or one of
  /// the specified codes.
  code: Vec<Coding>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for path
  _path: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The code-valued attribute of the filter. The specified path SHALL be a FHIRPath
  /// resolveable on the specified type of the DataRequirement, and SHALL consist only
  /// of identifiers, constant indexers, and .resolve(). The path is allowed to
  /// contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to
  /// traverse multiple-cardinality sub-elements (see the [Simple FHIRPath
  /// Profile](fhirpath.html#simple) for full details). Note that the index must be an
  /// integer constant. The path must resolve to an element of type code, Coding, or
  /// CodeableConcept.
  path: String,

  /// A token parameter that refers to a search parameter defined on the specified
  /// type of the DataRequirement, and which searches on elements of type code,
  /// Coding, or CodeableConcept.
  #[serde(rename = "searchParam")]
  search_param: String,

}
