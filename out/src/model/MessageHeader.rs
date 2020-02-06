#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::MessageHeader_Destination::MessageHeader_Destination;
use crate::model::MessageHeader_Source::MessageHeader_Source;
use crate::model::ResourceList::ResourceList;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::MessageHeader_Response::MessageHeader_Response;
use crate::model::Element::Element;


/// The header for a message exchange that is either requesting or responding to an
/// action.  The reference(s) that are the subject of the action as well as other
/// information related to the action are typically transmitted in a bundle in which
/// the MessageHeader resource instance is the first resource in the bundle.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeader {
  /// Code that identifies the event this message represents and connects it with its
  /// definition. Events defined as part of the FHIR specification have the system
  /// value "http://terminology.hl7.org/CodeSystem/message-events".  Alternatively uri
  /// to the EventDefinition.
  #[serde(rename = "eventUri")]
  event_uri: Option<String>,

  /// Identifies the sending system to allow the use of a trust relationship.
  sender: Option<Box<Reference>>,

  /// The logical author of the message - the person or device that decided the
  /// described event should happen. When there is more than one candidate, pick the
  /// most proximal to the MessageHeader. Can provide other authors in extensions.
  author: Option<Box<Reference>>,

  /// The source application from which this message originated.
  source: MessageHeader_Source,

  /// The person or organization that accepts overall responsibility for the contents
  /// of the message. The implication is that the message event happened under the
  /// policies of the responsible party.
  responsible: Option<Box<Reference>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Information about the message that this message is a response to.  Only present
  /// if this message is a response.
  response: Option<MessageHeader_Response>,

  /// Permanent link to the MessageDefinition for this message.
  definition: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The destination application which the message is intended for.
  destination: Option<Vec<MessageHeader_Destination>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Code that identifies the event this message represents and connects it with its
  /// definition. Events defined as part of the FHIR specification have the system
  /// value "http://terminology.hl7.org/CodeSystem/message-events".  Alternatively uri
  /// to the EventDefinition.
  #[serde(rename = "eventCoding")]
  event_coding: Option<Coding>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for eventUri
  #[serde(rename = "_eventUri")]
  _event_uri: Option<Element>,

  /// Coded indication of the cause for the event - indicates  a reason for the
  /// occurrence of the event that is a focus of this message.
  reason: Option<CodeableConcept>,

  /// The actual data of the message - a reference to the root/focus class of the
  /// event.
  focus: Option<Vec<Box<Reference>>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

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

  /// The person or device that performed the data entry leading to this message. When
  /// there is more than one candidate, pick the most proximal to the message. Can
  /// provide other enterers in extensions.
  enterer: Option<Box<Reference>>,

}
