#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::Meta::Meta;


/// A resource that represents the data of a single raw artifact as digital content
/// accessible in its native format.  A Binary resource can contain any content,
/// whether text, image, pdf, zip archive, etc.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binary {
  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// This element identifies another resource that can be used as a proxy of the
  /// security sensitivity to use when deciding and enforcing access control rules for
  /// the Binary resource. Given that the Binary resource contains very few elements
  /// that can be used to determine the sensitivity of the data and relationships to
  /// individuals, the referenced resource stands in as a proxy equivalent for this
  /// purpose. This referenced resource may be related to the Binary (e.g. Media,
  /// DocumentReference), or may be some non-related Resource purely as a security
  /// proxy. E.g. to identify that the binary resource relates to a patient, and
  /// access should only be granted to applications that have access to the patient.
  #[serde(rename = "securityContext")]
  security_context: Option<Box<Reference>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The actual content, base64 encoded.
  data: Option<String>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// MimeType of the binary content represented as a standard MimeType (BCP 13).
  #[serde(rename = "contentType")]
  content_type: Option<String>,

  /// Extensions for contentType
  #[serde(rename = "_contentType")]
  _content_type: Option<Element>,

  /// Extensions for data
  #[serde(rename = "_data")]
  _data: Option<Element>,

}
