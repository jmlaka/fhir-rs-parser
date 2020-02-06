#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Meta::Meta;
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ResourceList::ResourceList;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;


/// A reply to an appointment request for a patient and/or practitioner(s), such as
/// a confirmation or rejection.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentResponse {
  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Appointment that this response is replying to.
  appointment: Box<Reference>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Extensions for comment
  #[serde(rename = "_comment")]
  _comment: Option<Element>,

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

  /// Role of participant in the appointment.
  #[serde(rename = "participantType")]
  participant_type: Option<Vec<CodeableConcept>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Extensions for start
  #[serde(rename = "_start")]
  _start: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// This may be either the same as the appointment request to confirm the details of
  /// the appointment, or alternately a new time to request a re-negotiation of the
  /// end time.
  end: Option<String>,

  /// Extensions for end
  #[serde(rename = "_end")]
  _end: Option<Element>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// A Person, Location, HealthcareService, or Device that is participating in the
  /// appointment.
  actor: Option<Box<Reference>>,

  /// Participation status of the participant. When the status is declined or
  /// tentative if the start/end times are different to the appointment, then these
  /// times should be interpreted as a requested time change. When the status is
  /// accepted, the times can either be the time of the appointment (as a confirmation
  /// of the time) or can be empty.
  #[serde(rename = "participantStatus")]
  participant_status: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Additional comments about the appointment.
  comment: Option<String>,

  /// Date/Time that the appointment is to take place, or requested new start time.
  start: Option<String>,

  /// This records identifiers associated with this appointment response concern that
  /// are defined by business processes and/ or used to refer to it when a direct URL
  /// reference to the resource itself is not appropriate.
  identifier: Option<Vec<Identifier>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Extensions for participantStatus
  #[serde(rename = "_participantStatus")]
  _participant_status: Option<Element>,

}
