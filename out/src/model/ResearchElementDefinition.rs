#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::ResearchElementDefinition_Characteristic::ResearchElementDefinition_Characteristic;
use crate::model::ResourceList::ResourceList;
use crate::model::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The ResearchElementDefinition resource describes a "PICO" element that knowledge
/// (evidence, assertion, recommendation) is about.

#[derive(Debug)]
pub struct ResearchElementDefinition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ResearchElementDefinition<'_> {
    pub fn new(value: &Value) -> ResearchElementDefinition {
        ResearchElementDefinition {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for approvalDate
    pub fn _approval_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_approvalDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for comment
    pub fn _comment(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_comment") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for copyright
    pub fn _copyright(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_copyright") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for experimental
    pub fn _experimental(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_experimental") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for lastReviewDate
    pub fn _last_review_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastReviewDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for publisher
    pub fn _publisher(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publisher") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for purpose
    pub fn _purpose(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_purpose") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for shortTitle
    pub fn _short_title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_shortTitle") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for subtitle
    pub fn _subtitle(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subtitle") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for usage
    pub fn _usage(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_usage") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for variableType
    pub fn _variable_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_variableType") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub fn approval_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("approvalDate") {
            return Some(string);
        }
        return None;
    }

    /// An individiual or organization primarily involved in the creation and
    /// maintenance of the content.
    pub fn author(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("author") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A characteristic that defines the members of the research element. Multiple
    /// characteristics are applied with "and" semantics.
    pub fn characteristic(&self) -> Vec<ResearchElementDefinition_Characteristic> {
        if let Some(val) = self.value.get("characteristic") {
            if let Some(arr) = val.as_array() {
                return arr
                    .into_iter()
                    .map(|e| ResearchElementDefinition_Characteristic {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>();
            }
        }
        return Vec::new();
    }

    /// A human-readable string to clarify or explain concepts about the resource.
    pub fn comment(&self) -> Option<Vec<&str>> {
        match self.value.get("comment") {
            Some(Value::Array(val)) => Some(
                val.into_iter()
                    .filter_map(|e| e.as_str())
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        }
    }

    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub fn contact(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, and nor can they
    /// have their own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A copyright statement relating to the research element definition and/or its
    /// contents. Copyright statements are generally legal restrictions on the use and
    /// publishing of the research element definition.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// The date  (and optionally time) when the research element definition was
    /// published. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the research element definition changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// A free text natural language description of the research element definition from
    /// a consumer's perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// An individual or organization primarily responsible for internal coherence of
    /// the content.
    pub fn editor(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("editor") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The period during which the research element definition content was or is
    /// planned to be in active use.
    pub fn effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectivePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An individual or organization responsible for officially endorsing the content
    /// for use in some setting.
    pub fn endorser(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("endorser") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A Boolean value to indicate that this research element definition is authored
    /// for testing purposes (or education/evaluation/marketing) and is not intended to
    /// be used for genuine usage.
    pub fn experimental(&self) -> Option<bool> {
        match self.value.get("experimental") {
            Some(val) => val.as_bool(),
            _ => None,
        }
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A formal identifier that is used to identify this research element definition
    /// when it is represented in other formats, or referenced in a specification,
    /// model, design or an instance.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// A legal or geographic region in which the research element definition is
    /// intended to be used.
    pub fn jurisdiction(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("jurisdiction") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The date on which the resource content was last reviewed. Review happens
    /// periodically after approval but does not change the original approval date.
    pub fn last_review_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lastReviewDate") {
            return Some(string);
        }
        return None;
    }

    /// A reference to a Library resource containing the formal logic used by the
    /// ResearchElementDefinition.
    pub fn library(&self) -> Option<Vec<&str>> {
        match self.value.get("library") {
            Some(Value::Array(val)) => Some(
                val.into_iter()
                    .filter_map(|e| e.as_str())
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        }
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
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
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A natural language name identifying the research element definition. This name
    /// should be usable as an identifier for the module by machine processing
    /// applications such as code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The name of the organization or individual that published the research element
    /// definition.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// Explanation of why this research element definition is needed and why it has
    /// been designed as it has.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
        }
        return None;
    }

    /// Related artifacts such as additional documentation, justification, or
    /// bibliographic references.
    pub fn related_artifact(&self) -> Option<Vec<RelatedArtifact>> {
        if let Some(Value::Array(val)) = self.value.get("relatedArtifact") {
            return Some(
                val.into_iter()
                    .map(|e| RelatedArtifact {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An individual or organization primarily responsible for review of some aspect of
    /// the content.
    pub fn reviewer(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("reviewer") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The short title provides an alternate title for use in informal descriptive
    /// contexts where the full, formal title is not necessary.
    pub fn short_title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("shortTitle") {
            return Some(string);
        }
        return None;
    }

    /// The status of this research element definition. Enables tracking the life-cycle
    /// of the content.
    pub fn status(&self) -> Option<ResearchElementDefinitionStatus> {
        match self.value.get("status") {
            Some(Value::String(val)) => ResearchElementDefinitionStatus::from_string(&val),
            _ => None,
        }
    }

    /// The intended subjects for the ResearchElementDefinition. If this element is not
    /// provided, a Patient subject is assumed, but the subject of the
    /// ResearchElementDefinition can be anything.
    pub fn subject_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subjectCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The intended subjects for the ResearchElementDefinition. If this element is not
    /// provided, a Patient subject is assumed, but the subject of the
    /// ResearchElementDefinition can be anything.
    pub fn subject_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subjectReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An explanatory or alternate title for the ResearchElementDefinition giving
    /// additional information about its content.
    pub fn subtitle(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("subtitle") {
            return Some(string);
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
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A short, descriptive, user-friendly title for the research element definition.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// Descriptive topics related to the content of the ResearchElementDefinition.
    /// Topics provide a high-level categorization grouping types of
    /// ResearchElementDefinitions that can be useful for filtering and searching.
    pub fn topic(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("topic") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The type of research element, a population, an exposure, or an outcome.
    pub fn fhir_type(&self) -> Option<ResearchElementDefinitionType> {
        match self.value.get("type") {
            Some(Value::String(val)) => ResearchElementDefinitionType::from_string(&val),
            _ => None,
        }
    }

    /// An absolute URI that is used to identify this research element definition when
    /// it is referenced in a specification, model, design or an instance; also called
    /// its canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which at which an authoritative instance of this research element
    /// definition is (or will be) published. This URL can be the target of a canonical
    /// reference. It SHALL remain the same when the research element definition is
    /// stored on different servers.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// A detailed description, from a clinical perspective, of how the
    /// ResearchElementDefinition is used.
    pub fn usage(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("usage") {
            return Some(string);
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate research element
    /// definition instances.
    pub fn use_context(&self) -> Option<Vec<UsageContext>> {
        if let Some(Value::Array(val)) = self.value.get("useContext") {
            return Some(
                val.into_iter()
                    .map(|e| UsageContext {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The type of the outcome (e.g. Dichotomous, Continuous, or Descriptive).
    pub fn variable_type(&self) -> Option<ResearchElementDefinitionVariableType> {
        match self.value.get("variableType") {
            Some(Value::String(val)) => ResearchElementDefinitionVariableType::from_string(&val),
            _ => None,
        }
    }

    /// The identifier that is used to identify this version of the research element
    /// definition when it is referenced in a specification, model, design or instance.
    /// This is an arbitrary value managed by the research element definition author and
    /// is not expected to be globally unique. For example, it might be a timestamp
    /// (e.g. yyyymmdd) if a managed version is not available. There is also no
    /// expectation that versions can be placed in a lexicographical sequence. To
    /// provide a version consistent with the Decision Support Service specification,
    /// use the format Major.Minor.Revision (e.g. 1.0.0). For more information on
    /// versioning knowledge assets, refer to the Decision Support Service
    /// specification. Note that a version is required for non-experimental active
    /// artifacts.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._approval_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._comment() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._copyright() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._experimental() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._last_review_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._publisher() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._purpose() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._short_title() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._subtitle() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._title() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._url() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._usage() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._variable_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._version() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.approval_date() {}
        if let Some(_val) = self.author() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self
            .characteristic()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.comment() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.contact() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.editor() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.effective_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.endorser() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.jurisdiction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.last_review_date() {}
        if let Some(_val) = self.library() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self.related_artifact() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reviewer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.short_title() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.subject_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.subject_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.subtitle() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.topic() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.usage() {}
        if let Some(_val) = self.use_context() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.variable_type() {}
        if let Some(_val) = self.version() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ResearchElementDefinitionBuilder {
    pub(crate) value: Value,
}

impl ResearchElementDefinitionBuilder {
    pub fn build<'a>(&'a mut self) -> ResearchElementDefinition {
        self.value["resourceType"] = json!("ResearchElementDefinition");
        ResearchElementDefinition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ResearchElementDefinition) -> ResearchElementDefinitionBuilder {
        ResearchElementDefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        characteristic: Vec<ResearchElementDefinition_Characteristic>,
    ) -> ResearchElementDefinitionBuilder {
        let mut __value: Value = json!({});
        __value["characteristic"] = json!(characteristic
            .into_iter()
            .map(|e| e.value)
            .collect::<Vec<_>>());
        return ResearchElementDefinitionBuilder { value: __value };
    }

    pub fn _approval_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_approvalDate"] = json!(val.value);
        return self;
    }

    pub fn _comment<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_comment"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _copyright<'a>(&'a mut self, val: Element) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_copyright"] = json!(val.value);
        return self;
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _experimental<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_experimental"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _last_review_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_lastReviewDate"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _publisher<'a>(&'a mut self, val: Element) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_publisher"] = json!(val.value);
        return self;
    }

    pub fn _purpose<'a>(&'a mut self, val: Element) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_purpose"] = json!(val.value);
        return self;
    }

    pub fn _short_title<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_shortTitle"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _subtitle<'a>(&'a mut self, val: Element) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_subtitle"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn _usage<'a>(&'a mut self, val: Element) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_usage"] = json!(val.value);
        return self;
    }

    pub fn _variable_type<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_variableType"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn approval_date<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["approvalDate"] = json!(val);
        return self;
    }

    pub fn author<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["author"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn comment<'a>(&'a mut self, val: Vec<&str>) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["comment"] = json!(val);
        return self;
    }

    pub fn contact<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn copyright<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["copyright"] = json!(val);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn editor<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["editor"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn effective_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["effectivePeriod"] = json!(val.value);
        return self;
    }

    pub fn endorser<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["endorser"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn experimental<'a>(&'a mut self, val: bool) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["experimental"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["jurisdiction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn last_review_date<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["lastReviewDate"] = json!(val);
        return self;
    }

    pub fn library<'a>(&'a mut self, val: Vec<&str>) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["library"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn publisher<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["publisher"] = json!(val);
        return self;
    }

    pub fn purpose<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["purpose"] = json!(val);
        return self;
    }

    pub fn related_artifact<'a>(
        &'a mut self,
        val: Vec<RelatedArtifact>,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["relatedArtifact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reviewer<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["reviewer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn short_title<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["shortTitle"] = json!(val);
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: ResearchElementDefinitionStatus,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn subject_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["subjectCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn subject_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["subjectReference"] = json!(val.value);
        return self;
    }

    pub fn subtitle<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["subtitle"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn topic<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["topic"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: ResearchElementDefinitionType,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn usage<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["usage"] = json!(val);
        return self;
    }

    pub fn use_context<'a>(
        &'a mut self,
        val: Vec<UsageContext>,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["useContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn variable_type<'a>(
        &'a mut self,
        val: ResearchElementDefinitionVariableType,
    ) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["variableType"] = json!(val.to_string());
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut ResearchElementDefinitionBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum ResearchElementDefinitionStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl ResearchElementDefinitionStatus {
    pub fn from_string(string: &str) -> Option<ResearchElementDefinitionStatus> {
        match string {
            "draft" => Some(ResearchElementDefinitionStatus::Draft),
            "active" => Some(ResearchElementDefinitionStatus::Active),
            "retired" => Some(ResearchElementDefinitionStatus::Retired),
            "unknown" => Some(ResearchElementDefinitionStatus::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ResearchElementDefinitionStatus::Draft => "draft".to_string(),
            ResearchElementDefinitionStatus::Active => "active".to_string(),
            ResearchElementDefinitionStatus::Retired => "retired".to_string(),
            ResearchElementDefinitionStatus::Unknown => "unknown".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ResearchElementDefinitionType {
    Population,
    Exposure,
    Outcome,
}

impl ResearchElementDefinitionType {
    pub fn from_string(string: &str) -> Option<ResearchElementDefinitionType> {
        match string {
            "population" => Some(ResearchElementDefinitionType::Population),
            "exposure" => Some(ResearchElementDefinitionType::Exposure),
            "outcome" => Some(ResearchElementDefinitionType::Outcome),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ResearchElementDefinitionType::Population => "population".to_string(),
            ResearchElementDefinitionType::Exposure => "exposure".to_string(),
            ResearchElementDefinitionType::Outcome => "outcome".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ResearchElementDefinitionVariableType {
    Dichotomous,
    Continuous,
    Descriptive,
}

impl ResearchElementDefinitionVariableType {
    pub fn from_string(string: &str) -> Option<ResearchElementDefinitionVariableType> {
        match string {
            "dichotomous" => Some(ResearchElementDefinitionVariableType::Dichotomous),
            "continuous" => Some(ResearchElementDefinitionVariableType::Continuous),
            "descriptive" => Some(ResearchElementDefinitionVariableType::Descriptive),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ResearchElementDefinitionVariableType::Dichotomous => "dichotomous".to_string(),
            ResearchElementDefinitionVariableType::Continuous => "continuous".to_string(),
            ResearchElementDefinitionVariableType::Descriptive => "descriptive".to_string(),
        }
    }
}
