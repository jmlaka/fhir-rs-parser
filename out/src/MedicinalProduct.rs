use serde::{Deserialize, Serialize};

/// Detailed definition of a medicinal product, typically for uses other than direct
/// patient care (e.g. regulatory use).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicinalProduct {
  /// The product's name, including full name and possibly coded parts.
  name: Vec<MedicinalProduct_Name>,

  /// Indicates if the medicinal product has an orphan designation for the treatment
  /// of a rare disease.
  #[serde(rename = "specialDesignation")]
  special_designation: Vec<MedicinalProduct_SpecialDesignation>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Allows the product to be classified by various systems.
  #[serde(rename = "productClassification")]
  product_classification: Vec<CodeableConcept>,

  /// A product specific contact, person (in a role), or an organization.
  contact: Vec<Reference>,

  /// Regulatory type, e.g. Investigational or Authorized.
  type: CodeableConcept,

  /// Whether the Medicinal Product is subject to additional monitoring for regulatory
  /// reasons.
  #[serde(rename = "additionalMonitoringIndicator")]
  additional_monitoring_indicator: CodeableConcept,

  /// Extensions for language
  _language: Element,

  /// Marketing status of the medicinal product, in contrast to marketing
  /// authorizaton.
  #[serde(rename = "marketingStatus")]
  marketing_status: Vec<MarketingStatus>,

  /// Reference to another product, e.g. for linking authorised to investigational
  /// product.
  #[serde(rename = "crossReference")]
  cross_reference: Vec<Identifier>,

  /// Package representation for the product.
  #[serde(rename = "packagedMedicinalProduct")]
  packaged_medicinal_product: Vec<Reference>,

  /// A master file for to the medicinal product (e.g. Pharmacovigilance System Master
  /// File).
  #[serde(rename = "masterFile")]
  master_file: Vec<Reference>,

  /// If this medicine applies to human or veterinary uses.
  domain: Coding,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The base language in which the resource is written.
  language: String,

  /// Business identifier for this product. Could be an MPID.
  identifier: Vec<Identifier>,

  /// Whether the Medicinal Product is subject to special measures for regulatory
  /// reasons.
  #[serde(rename = "specialMeasures")]
  special_measures: Vec<String>,

  /// If authorised for use in children.
  #[serde(rename = "paediatricUseIndicator")]
  paediatric_use_indicator: CodeableConcept,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Pharmaceutical aspects of product.
  #[serde(rename = "pharmaceuticalProduct")]
  pharmaceutical_product: Vec<Reference>,

  /// The legal status of supply of the medicinal product as classified by the
  /// regulator.
  #[serde(rename = "legalStatusOfSupply")]
  legal_status_of_supply: CodeableConcept,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// An operation applied to the product, for manufacturing or adminsitrative
  /// purpose.
  #[serde(rename = "manufacturingBusinessOperation")]
  manufacturing_business_operation: Vec<MedicinalProduct_ManufacturingBusinessOperation>,

  /// Supporting documentation, typically for regulatory submission.
  #[serde(rename = "attachedDocument")]
  attached_document: Vec<Reference>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Clinical trials or studies that this product is involved in.
  #[serde(rename = "clinicalTrial")]
  clinical_trial: Vec<Reference>,

  /// Extensions for specialMeasures
  #[serde(rename = "_specialMeasures")]
  _special_measures: Vec<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The dose form for a single part product, or combined form of a multiple part
  /// product.
  #[serde(rename = "combinedPharmaceuticalDoseForm")]
  combined_pharmaceutical_dose_form: CodeableConcept,

}
