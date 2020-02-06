#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::MedicinalProductPackaged_PackageItem::MedicinalProductPackaged_PackageItem;
use crate::model::Identifier::Identifier;
use crate::model::ResourceList::ResourceList;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::MarketingStatus::MarketingStatus;
use crate::model::Element::Element;
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::MedicinalProductPackaged_BatchIdentifier::MedicinalProductPackaged_BatchIdentifier;
use crate::model::Reference::Reference;


/// A medicinal product in a container or package.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductPackaged {
  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The legal status of supply of the medicinal product as classified by the
  /// regulator.
  #[serde(rename = "legalStatusOfSupply")]
  legal_status_of_supply: Option<CodeableConcept>,

  /// Manufacturer of this Package Item.
  manufacturer: Option<Vec<Box<Reference>>>,

  /// Batch numbering.
  #[serde(rename = "batchIdentifier")]
  batch_identifier: Option<Vec<MedicinalProductPackaged_BatchIdentifier>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Marketing information.
  #[serde(rename = "marketingStatus")]
  marketing_status: Option<Vec<MarketingStatus>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// A packaging item, as a contained for medicine, possibly with other packaging
  /// items within.
  #[serde(rename = "packageItem")]
  package_item: Vec<MedicinalProductPackaged_PackageItem>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Textual description.
  description: Option<String>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Manufacturer of this Package Item.
  #[serde(rename = "marketingAuthorization")]
  marketing_authorization: Option<Box<Reference>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// The product with this is a pack for.
  subject: Option<Vec<Box<Reference>>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// Unique identifier.
  identifier: Option<Vec<Identifier>>,

}
