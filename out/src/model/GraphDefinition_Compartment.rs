#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A formal computable definition of a graph of resources - that is, a coherent set
/// of resources that form a graph by following references. The Graph Definition
/// resource defines a set and makes rules about the set.

#[derive(Debug)]
pub struct GraphDefinition_Compartment<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl GraphDefinition_Compartment<'_> {
    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
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

    /// Extensions for expression
    pub fn _expression(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_expression") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for rule
    pub fn _rule(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_rule") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for use
    pub fn _use(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_use") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the compartment.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
        }
        return None;
    }

    /// Documentation for FHIRPath expression.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Custom rule, as a FHIRPath expression.
    pub fn expression(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("expression") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
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

    /// identical | matching | different | no-rule | custom.
    pub fn rule(&self) -> Option<GraphDefinition_CompartmentRule> {
        if let Some(Value::String(val)) = self.value.get("rule") {
            return Some(GraphDefinition_CompartmentRule::from_string(&val).unwrap());
        }
        return None;
    }

    /// Defines how the compartment rule is used - whether it it is used to test whether
    /// resources are subject to the rule, or whether it is a rule that must be
    /// followed.
    pub fn fhir_use(&self) -> Option<GraphDefinition_CompartmentUse> {
        if let Some(Value::String(val)) = self.value.get("use") {
            return Some(GraphDefinition_CompartmentUse::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._expression() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._rule() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._use() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.expression() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.rule() {}
        if let Some(_val) = self.fhir_use() {}
        return true;
    }
}

#[derive(Debug)]
pub struct GraphDefinition_CompartmentBuilder {
    pub value: Value,
}

impl GraphDefinition_CompartmentBuilder {
    pub fn build(&self) -> GraphDefinition_Compartment {
        GraphDefinition_Compartment {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn new() -> GraphDefinition_CompartmentBuilder {
        let mut __value: Value = json!({});
        return GraphDefinition_CompartmentBuilder { value: __value };
    }
}

#[derive(Debug)]
pub enum GraphDefinition_CompartmentRule {
    Identical,
    Matching,
    Different,
    Custom,
}

impl GraphDefinition_CompartmentRule {
    pub fn from_string(string: &str) -> Option<GraphDefinition_CompartmentRule> {
        match string {
            "identical" => Some(GraphDefinition_CompartmentRule::Identical),
            "matching" => Some(GraphDefinition_CompartmentRule::Matching),
            "different" => Some(GraphDefinition_CompartmentRule::Different),
            "custom" => Some(GraphDefinition_CompartmentRule::Custom),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            GraphDefinition_CompartmentRule::Identical => "identical".to_string(),
            GraphDefinition_CompartmentRule::Matching => "matching".to_string(),
            GraphDefinition_CompartmentRule::Different => "different".to_string(),
            GraphDefinition_CompartmentRule::Custom => "custom".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum GraphDefinition_CompartmentUse {
    Condition,
    Requirement,
}

impl GraphDefinition_CompartmentUse {
    pub fn from_string(string: &str) -> Option<GraphDefinition_CompartmentUse> {
        match string {
            "condition" => Some(GraphDefinition_CompartmentUse::Condition),
            "requirement" => Some(GraphDefinition_CompartmentUse::Requirement),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            GraphDefinition_CompartmentUse::Condition => "condition".to_string(),
            GraphDefinition_CompartmentUse::Requirement => "requirement".to_string(),
        }
    }
}
