use serde::{Deserialize, Serialize};

/// Raw data describing a biological sequence.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MolecularSequence_Repository {
  /// Id of the variantset in this external repository. The server will understand how
  /// to use this id to call for more info about variantsets in external repository.
  #[serde(rename = "variantsetId")]
  variantset_id: String,

  /// Extensions for variantsetId
  #[serde(rename = "_variantsetId")]
  _variantset_id: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Id of the read in this external repository.
  #[serde(rename = "readsetId")]
  readset_id: String,

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

  /// Click and see / RESTful API / Need login to see / RESTful API with
  /// authentication / Other ways to see resource.
  type: MolecularSequence_RepositoryType,

  /// Extensions for url
  _url: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for name
  _name: Element,

  /// Extensions for datasetId
  #[serde(rename = "_datasetId")]
  _dataset_id: Element,

  /// URI of an external repository which contains further details about the genetics
  /// data.
  url: String,

  /// Extensions for readsetId
  #[serde(rename = "_readsetId")]
  _readset_id: Element,

  /// Extensions for type
  _type: Element,

  /// Id of the variant in this external repository. The server will understand how to
  /// use this id to call for more info about datasets in external repository.
  #[serde(rename = "datasetId")]
  dataset_id: String,

  /// URI of an external repository which contains further details about the genetics
  /// data.
  name: String,

}

#[derive(Debug, Serialize, Deserialize)]
enum MolecularSequence_RepositoryType {
  #[serde(rename = "directlink")]
  Directlink,

  #[serde(rename = "openapi")]
  Openapi,

  #[serde(rename = "login")]
  Login,

  #[serde(rename = "oauth")]
  Oauth,

  #[serde(rename = "other")]
  Other,

}
