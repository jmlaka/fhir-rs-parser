#![allow(unused_imports, non_camel_case_types)]

use crate::model::Identifier::Identifier;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::Reference::Reference;
use crate::model::List_Entry::List_Entry;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Annotation::Annotation;
use crate::model::Meta::Meta;
use serde_json::value::Value;



/// A list is a curated collection of resources.

#[derive(Debug)]
pub struct List<'a> {
  pub value: &'a Value,
}

impl List<'_> {
  /// What order applies to the items in the list.
  pub fn ordered_by(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("orderedBy") {
      return Some(CodeableConcept { value: val });
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

  /// Indicates the current state of this list.
  pub fn status(&self) -> Option<ListStatus> {
    if let Some(Value::String(val)) = self.value.get("status") {
      return Some(ListStatus::from_string(&val).unwrap());
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

  /// The base language in which the resource is written.
  pub fn language(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("language") {
      return Some(string.to_string());
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

  /// Extensions for implicitRules
  pub fn _implicit_rules(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_implicitRules") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// How this list was prepared - whether it is a working list that is suitable for
  /// being maintained on an ongoing basis, or if it represents a snapshot of a list
  /// of items from another source, or whether it is a prepared list where items may
  /// be marked as added, modified or deleted.
  pub fn mode(&self) -> Option<ListMode> {
    if let Some(Value::String(val)) = self.value.get("mode") {
      return Some(ListMode::from_string(&val).unwrap());
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

  /// Identifier for the List assigned for business purposes outside the context of
  /// FHIR.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
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

  /// This code defines the purpose of the list - why it was created.
  pub fn code(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("code") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// The common subject (or patient) of the resources that are in the list if there
  /// is one.
  pub fn subject(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("subject") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The date that the list was prepared.
  pub fn date(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("date") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The entity responsible for deciding what the contents of the list were. Where
  /// the list was created by a human, this is the same as the author of the list.
  pub fn source(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("source") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Entries in this list.
  pub fn entry(&self) -> Option<Vec<List_Entry>> {
    if let Some(Value::Array(val)) = self.value.get("entry") {
      return Some(val.into_iter().map(|e| List_Entry { value: e }).collect::<Vec<_>>());
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

  /// A label for the list assigned by the author.
  pub fn title(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("title") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The encounter that is the context in which this list was created.
  pub fn encounter(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("encounter") {
      return Some(Reference { value: val });
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

  /// Comments that apply to the overall list.
  pub fn note(&self) -> Option<Vec<Annotation>> {
    if let Some(Value::Array(val)) = self.value.get("note") {
      return Some(val.into_iter().map(|e| Annotation { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// If the list is empty, why the list is empty.
  pub fn empty_reason(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("emptyReason") {
      return Some(CodeableConcept { value: val });
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

  /// Extensions for title
  pub fn _title(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_title") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for date
  pub fn _date(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_date") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for mode
  pub fn _mode(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_mode") {
      return Some(Element { value: val });
    }
    return None;
  }

}

#[derive(Debug)]
pub enum ListStatus {
  Current,
  Retired,
  EnteredInError,
}

impl ListStatus {
    pub fn from_string(string: &str) -> Option<ListStatus> {
      match string {
        "current" => Some(ListStatus::Current),
        "retired" => Some(ListStatus::Retired),
        "entered-in-error" => Some(ListStatus::EnteredInError),
        _ => None,
    }
  }
}


#[derive(Debug)]
pub enum ListMode {
  Working,
  Snapshot,
  Changes,
}

impl ListMode {
    pub fn from_string(string: &str) -> Option<ListMode> {
      match string {
        "working" => Some(ListMode::Working),
        "snapshot" => Some(ListMode::Snapshot),
        "changes" => Some(ListMode::Changes),
        _ => None,
    }
  }
}

