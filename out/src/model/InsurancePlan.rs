#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::InsurancePlan_Contact::InsurancePlan_Contact;
use crate::model::InsurancePlan_Coverage::InsurancePlan_Coverage;
use crate::model::InsurancePlan_Plan::InsurancePlan_Plan;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Details of a Health Insurance product/plan provided by an organization.

#[derive(Debug)]
pub struct InsurancePlan<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl InsurancePlan<'_> {
    pub fn new(value: &Value) -> InsurancePlan {
        InsurancePlan {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for alias
    pub fn _alias(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_alias") {
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

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
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

    /// An organization which administer other services such as underwriting, customer
    /// service and/or claims processing on behalf of the health insurance product
    /// owner.
    pub fn administered_by(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("administeredBy") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A list of alternate names that the product is known as, or was known as in the
    /// past.
    pub fn alias(&self) -> Option<Vec<&str>> {
        match self.value.get("alias") {
            Some(Value::Array(val)) => Some(
                val.into_iter()
                    .filter_map(|e| e.as_str())
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        }
    }

    /// The contact for the health insurance product for a certain purpose.
    pub fn contact(&self) -> Option<Vec<InsurancePlan_Contact>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| InsurancePlan_Contact {
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

    /// Details about the coverage offered by the insurance product.
    pub fn coverage(&self) -> Option<Vec<InsurancePlan_Coverage>> {
        if let Some(Value::Array(val)) = self.value.get("coverage") {
            return Some(
                val.into_iter()
                    .map(|e| InsurancePlan_Coverage {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The geographic region in which a health insurance product's benefits apply.
    pub fn coverage_area(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("coverageArea") {
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

    /// The technical endpoints providing access to services operated for the health
    /// insurance product.
    pub fn endpoint(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("endpoint") {
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

    /// Business identifiers assigned to this health insurance product which remain
    /// constant as the resource is updated and propagates from server to server.
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
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

    /// Official name of the health insurance product as designated by the owner.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Reference to the network included in the health insurance product.
    pub fn network(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("network") {
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

    /// The entity that is providing  the health insurance product and underwriting the
    /// risk.  This is typically an insurance carriers, other third-party payers, or
    /// health plan sponsors comonly referred to as 'payers'.
    pub fn owned_by(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("ownedBy") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The period of time that the health insurance product is available.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Details about an insurance plan.
    pub fn plan(&self) -> Option<Vec<InsurancePlan_Plan>> {
        if let Some(Value::Array(val)) = self.value.get("plan") {
            return Some(
                val.into_iter()
                    .map(|e| InsurancePlan_Plan {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The current state of the health insurance product.
    pub fn status(&self) -> Option<InsurancePlanStatus> {
        match self.value.get("status") {
            Some(Value::String(val)) => InsurancePlanStatus::from_string(&val),
            _ => None,
        }
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

    /// The kind of health insurance product.
    pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("type") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._alias() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.administered_by() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.alias() {
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
        if let Some(_val) = self.coverage() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.coverage_area() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.endpoint() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
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
        if let Some(_val) = self.language() {}
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
        if let Some(_val) = self.network() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.owned_by() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.plan() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct InsurancePlanBuilder {
    pub(crate) value: Value,
}

impl InsurancePlanBuilder {
    pub fn build<'a>(&'a mut self) -> InsurancePlan {
        self.value["resourceType"] = json!("InsurancePlan");
        InsurancePlan {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: InsurancePlan) -> InsurancePlanBuilder {
        InsurancePlanBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> InsurancePlanBuilder {
        let mut __value: Value = json!({});
        return InsurancePlanBuilder { value: __value };
    }

    pub fn _alias<'a>(&'a mut self, val: Vec<Element>) -> &'a mut InsurancePlanBuilder {
        self.value["_alias"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut InsurancePlanBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut InsurancePlanBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut InsurancePlanBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut InsurancePlanBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn administered_by<'a>(&'a mut self, val: Reference) -> &'a mut InsurancePlanBuilder {
        self.value["administeredBy"] = json!(val.value);
        return self;
    }

    pub fn alias<'a>(&'a mut self, val: Vec<&str>) -> &'a mut InsurancePlanBuilder {
        self.value["alias"] = json!(val);
        return self;
    }

    pub fn contact<'a>(
        &'a mut self,
        val: Vec<InsurancePlan_Contact>,
    ) -> &'a mut InsurancePlanBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut InsurancePlanBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn coverage<'a>(
        &'a mut self,
        val: Vec<InsurancePlan_Coverage>,
    ) -> &'a mut InsurancePlanBuilder {
        self.value["coverage"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn coverage_area<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut InsurancePlanBuilder {
        self.value["coverageArea"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn endpoint<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut InsurancePlanBuilder {
        self.value["endpoint"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut InsurancePlanBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut InsurancePlanBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut InsurancePlanBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut InsurancePlanBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut InsurancePlanBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut InsurancePlanBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InsurancePlanBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut InsurancePlanBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn network<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut InsurancePlanBuilder {
        self.value["network"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn owned_by<'a>(&'a mut self, val: Reference) -> &'a mut InsurancePlanBuilder {
        self.value["ownedBy"] = json!(val.value);
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut InsurancePlanBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }

    pub fn plan<'a>(&'a mut self, val: Vec<InsurancePlan_Plan>) -> &'a mut InsurancePlanBuilder {
        self.value["plan"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: InsurancePlanStatus) -> &'a mut InsurancePlanBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut InsurancePlanBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut InsurancePlanBuilder {
        self.value["type"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum InsurancePlanStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl InsurancePlanStatus {
    pub fn from_string(string: &str) -> Option<InsurancePlanStatus> {
        match string {
            "draft" => Some(InsurancePlanStatus::Draft),
            "active" => Some(InsurancePlanStatus::Active),
            "retired" => Some(InsurancePlanStatus::Retired),
            "unknown" => Some(InsurancePlanStatus::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            InsurancePlanStatus::Draft => "draft".to_string(),
            InsurancePlanStatus::Active => "active".to_string(),
            InsurancePlanStatus::Retired => "retired".to_string(),
            InsurancePlanStatus::Unknown => "unknown".to_string(),
        }
    }
}
