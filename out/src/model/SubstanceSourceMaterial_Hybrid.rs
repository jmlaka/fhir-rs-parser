#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;


/// Source material shall capture information on the taxonomic and anatomical
/// origins as well as the fraction of a material that can result in or can be
/// modified to form a substance. This set of data elements shall be used to define
/// polymer substances isolated from biological matrices. Taxonomic and anatomical
/// origins shall be described using a controlled vocabulary as required. This
/// information is captured for naturally derived polymers ( . starch) and
/// structurally diverse substances. For Organisms belonging to the Kingdom Plantae
/// the Substance level defines the fresh material of a single species or
/// infraspecies, the Herbal Drug and the Herbal preparation. For Herbal
/// preparations, the fraction information will be captured at the Substance
/// information level and additional information for herbal extracts will be
/// captured at the Specified Substance Group 1 information level. See for further
/// explanation the Substance Class: Structurally Diverse and the herbal annex.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterial_Hybrid {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// The name of the maternal species constituting the hybrid organism shall be
  /// specified. For plants, the parents aren’t always known, and it is unlikely that
  /// it will be known which is maternal and which is paternal.
  #[serde(rename = "maternalOrganismName")]
  maternal_organism_name: Option<String>,

  /// The name of the paternal species constituting the hybrid organism shall be
  /// specified.
  #[serde(rename = "paternalOrganismName")]
  paternal_organism_name: Option<String>,

  /// The identifier of the maternal species constituting the hybrid organism shall be
  /// specified based on a controlled vocabulary. For plants, the parents aren’t
  /// always known, and it is unlikely that it will be known which is maternal and
  /// which is paternal.
  #[serde(rename = "maternalOrganismId")]
  maternal_organism_id: Option<String>,

  /// Extensions for maternalOrganismName
  #[serde(rename = "_maternalOrganismName")]
  _maternal_organism_name: Option<Element>,

  /// Extensions for maternalOrganismId
  #[serde(rename = "_maternalOrganismId")]
  _maternal_organism_id: Option<Element>,

  /// Extensions for paternalOrganismId
  #[serde(rename = "_paternalOrganismId")]
  _paternal_organism_id: Option<Element>,

  /// Extensions for paternalOrganismName
  #[serde(rename = "_paternalOrganismName")]
  _paternal_organism_name: Option<Element>,

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

  /// The identifier of the paternal species constituting the hybrid organism shall be
  /// specified based on a controlled vocabulary.
  #[serde(rename = "paternalOrganismId")]
  paternal_organism_id: Option<String>,

  /// The hybrid type of an organism shall be specified.
  #[serde(rename = "hybridType")]
  hybrid_type: Option<CodeableConcept>,

}
