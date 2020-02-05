use serde::{Deserialize, Serialize};

/// A material substance originating from a biological entity intended to be
/// transplanted or infused
/// into another (possibly the same) biological entity.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BiologicallyDerivedProduct {
  /// Whether the product is currently available.
  status: BiologicallyDerivedProductStatus,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

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

  /// Procedure request to obtain this biologically derived product.
  request: Vec<Reference>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The base language in which the resource is written.
  language: String,

  /// Any manipulation of product post-collection that is intended to alter the
  /// product.  For example a buffy-coat enrichment or CD8 reduction of Peripheral
  /// Blood Stem Cells to make it more suitable for infusion.
  manipulation: BiologicallyDerivedProduct_Manipulation,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Parent product (if any).
  parent: Vec<Reference>,

  /// Any processing of the product during collection that does not change the
  /// fundamental nature of the product. For example adding anti-coagulants during the
  /// collection of Peripheral Blood Stem Cells.
  processing: Vec<BiologicallyDerivedProduct_Processing>,

  /// Product storage.
  storage: Vec<BiologicallyDerivedProduct_Storage>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Number of discrete units within this product.
  quantity: integer,

  /// How this product was collected.
  collection: BiologicallyDerivedProduct_Collection,

  /// Extensions for quantity
  _quantity: Element,

  /// A code that identifies the kind of this biologically derived product (SNOMED
  /// Ctcode).
  #[serde(rename = "productCode")]
  product_code: CodeableConcept,

  /// Extensions for status
  _status: Element,

  /// This records identifiers associated with this biologically derived product
  /// instance that are defined by business processes and/or used to refer to it when
  /// a direct URL reference to the resource itself is not appropriate (e.g. in CDA
  /// documents, or in written / printed documentation).
  identifier: Vec<Identifier>,

  /// Broad category of this product.
  #[serde(rename = "productCategory")]
  product_category: BiologicallyDerivedProductProductCategory,

  /// Extensions for productCategory
  #[serde(rename = "_productCategory")]
  _product_category: Element,

  /// Extensions for language
  _language: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

}

#[derive(Debug, Serialize, Deserialize)]
enum BiologicallyDerivedProductStatus {
  #[serde(rename = "available")]
  Available,

  #[serde(rename = "unavailable")]
  Unavailable,

}

#[derive(Debug, Serialize, Deserialize)]
enum BiologicallyDerivedProductProductCategory {
  #[serde(rename = "organ")]
  Organ,

  #[serde(rename = "tissue")]
  Tissue,

  #[serde(rename = "fluid")]
  Fluid,

  #[serde(rename = "cells")]
  Cells,

  #[serde(rename = "biologicalAgent")]
  BiologicalAgent,

}
