#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::MedicinalProductInteraction_Interactant::MedicinalProductInteraction_Interactant;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The interactions of the medicinal product with other medicinal products, or
/// other forms of interactions.

#[derive(Debug)]
pub struct MedicinalProductInteraction<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductInteraction<'_> {
    pub fn new(value: &Value) -> MedicinalProductInteraction {
        MedicinalProductInteraction {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// The interaction described.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// The effect of the interaction, for example "reduced gastric absorption of
    /// primary medication".
    pub fn effect(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("effect") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// The incidence of the interaction, e.g. theoretical, observed.
    pub fn incidence(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("incidence") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The specific medication, food or laboratory test that interacts.
    pub fn interactant(&self) -> Option<Vec<MedicinalProductInteraction_Interactant>> {
        if let Some(Value::Array(val)) = self.value.get("interactant") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProductInteraction_Interactant {
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

    /// Actions for managing the interaction.
    pub fn management(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("management") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
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

    /// The medication for which this is a described interaction.
    pub fn subject(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("subject") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// The type of the interaction e.g. drug-drug interaction, drug-food interaction,
    /// drug-lab test interaction.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
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
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.effect() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.incidence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.interactant() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.management() {
            if !_val.validate() {
                return false;
            }
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
        if let Some(_val) = self.subject() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductInteractionBuilder {
    pub(crate) value: Value,
}

impl MedicinalProductInteractionBuilder {
    pub fn build<'a>(&'a mut self) -> MedicinalProductInteraction {
        self.value["resourceType"] = json!("MedicinalProductInteraction");
        MedicinalProductInteraction {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MedicinalProductInteraction) -> MedicinalProductInteractionBuilder {
        MedicinalProductInteractionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicinalProductInteractionBuilder {
        let mut __value: Value = json!({});
        return MedicinalProductInteractionBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn effect<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["effect"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn incidence<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["incidence"] = json!(val.value);
        return self;
    }

    pub fn interactant<'a>(
        &'a mut self,
        val: Vec<MedicinalProductInteraction_Interactant>,
    ) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["interactant"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn management<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["management"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn subject<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["subject"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductInteractionBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
