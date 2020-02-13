#![allow(unused_imports, non_camel_case_types)]

use crate::model::MedicationAdministration_Performer::MedicationAdministration_Performer;
use crate::model::Narrative::Narrative;
use crate::model::Identifier::Identifier;
use crate::model::Period::Period;
use crate::model::ResourceList::ResourceList;
use crate::model::Annotation::Annotation;
use crate::model::MedicationAdministration_Dosage::MedicationAdministration_Dosage;
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use serde_json::value::Value;



/// Describes the event of a patient consuming or otherwise being administered a
/// medication.  This may be as simple as swallowing a tablet or it may be a long
/// running infusion.  Related resources tie this event to the authorizing
/// prescription, and the specific encounter between patient and health care
/// practitioner.

#[derive(Debug)]
pub struct MedicationAdministration<'a> {
  pub value: &'a Value,
}

impl MedicationAdministration<'_> {
  /// The original request, instruction or authority to perform the administration.
  pub fn request(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("request") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Identifiers associated with this Medication Administration that are defined by
  /// business processes and/or used to refer to it when a direct URL reference to the
  /// resource itself is not appropriate. They are business identifiers assigned to
  /// this resource by the performer or other systems and remain constant as the
  /// resource is updated and propagates from server to server.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A summary of the events of interest that have occurred, such as when the
  /// administration was verified.
  pub fn event_history(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("eventHistory") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Describes the medication dosage information details e.g. dose, rate, site,
  /// route, etc.
  pub fn dosage(&self) -> Option<MedicationAdministration_Dosage> {
    if let Some(val) = self.value.get("dosage") {
      return Some(MedicationAdministration_Dosage { value: val });
    }
    return None;
  }

  /// Additional information (for example, patient height and weight) that supports
  /// the administration of the medication.
  pub fn supporting_information(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("supportingInformation") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A code indicating why the medication was given.
  pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("reasonCode") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  pub fn meta(&self) -> Option<Meta> {
    if let Some(val) = self.value.get("meta") {
      return Some(Meta { value: val });
    }
    return None;
  }

  /// The device used in administering the medication to the patient.  For example, a
  /// particular infusion pump.
  pub fn device(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("device") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

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
  pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The base language in which the resource is written.
  pub fn language(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("language") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Identifies the medication that was administered. This is either a link to a
  /// resource representing the details of the medication or a simple attribute
  /// carrying a code that identifies the medication from a known list of medications.
  pub fn medication_reference(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("medicationReference") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Extensions for language
  pub fn _language(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_language") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Indicates who or what performed the medication administration and how they were
  /// involved.
  pub fn performer(&self) -> Option<Vec<MedicationAdministration_Performer>> {
    if let Some(Value::Array(val)) = self.value.get("performer") {
      return Some(val.into_iter().map(|e| MedicationAdministration_Performer { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  pub fn text(&self) -> Option<Narrative> {
    if let Some(val) = self.value.get("text") {
      return Some(Narrative { value: val });
    }
    return None;
  }

  /// Condition or observation that supports why the medication was administered.
  pub fn reason_reference(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("reasonReference") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  pub fn implicit_rules(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("implicitRules") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A specific date/time or interval of time during which the administration took
  /// place (or did not take place, when the 'notGiven' attribute is true). For many
  /// administrations, such as swallowing a tablet the use of dateTime is more
  /// appropriate.
  pub fn effective_period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("effectivePeriod") {
      return Some(Period { value: val });
    }
    return None;
  }

  /// Identifies the medication that was administered. This is either a link to a
  /// resource representing the details of the medication or a simple attribute
  /// carrying a code that identifies the medication from a known list of medications.
  pub fn medication_codeable_concept(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("medicationCodeableConcept") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for status
  pub fn _status(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_status") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  pub fn contained(&self) -> Option<Vec<ResourceList>> {
    if let Some(Value::Array(val)) = self.value.get("contained") {
      return Some(val.into_iter().map(|e| ResourceList { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A code indicating why the administration was not performed.
  pub fn status_reason(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("statusReason") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for implicitRules
  pub fn _implicit_rules(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_implicitRules") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The person or animal or group receiving the medication.
  pub fn subject(&self) -> Reference {
    Reference {
      value: &self.value["subject"],
    }
  }

  /// A specific date/time or interval of time during which the administration took
  /// place (or did not take place, when the 'notGiven' attribute is true). For many
  /// administrations, such as swallowing a tablet the use of dateTime is more
  /// appropriate.
  pub fn effective_date_time(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("effectiveDateTime") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A protocol, guideline, orderset, or other definition that was adhered to in
  /// whole or in part by this event.
  pub fn instantiates(&self) -> Option<Vec<String>> {
    if let Some(Value::Array(val)) = self.value.get("instantiates") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap().to_string()).collect::<Vec<_>>());
    }
    return None;
  }

  /// Indicates where the medication is expected to be consumed or administered.
  pub fn category(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("category") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Extra information about the medication administration that is not conveyed by
  /// the other attributes.
  pub fn note(&self) -> Option<Vec<Annotation>> {
    if let Some(Value::Array(val)) = self.value.get("note") {
      return Some(val.into_iter().map(|e| Annotation { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The visit, admission, or other contact between patient and health care provider
  /// during which the medication administration was performed.
  pub fn context(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("context") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Extensions for effectiveDateTime
  pub fn _effective_date_time(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_effectiveDateTime") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  pub fn extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("extension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A larger event of which this particular event is a component or step.
  pub fn part_of(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("partOf") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Will generally be set to show that the administration has been completed.  For
  /// some long running administrations such as infusions, it is possible for an
  /// administration to be started but not completed or it may be paused while some
  /// other process is under way.
  pub fn status(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("status") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for instantiates
  pub fn _instantiates(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_instantiates") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

}
