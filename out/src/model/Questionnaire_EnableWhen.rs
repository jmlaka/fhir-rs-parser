#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::Coding::Coding;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.

#[derive(Debug)]
pub struct Questionnaire_EnableWhen<'a> {
  pub value: &'a Value,
}

impl Questionnaire_EnableWhen<'_> {
  /// Extensions for answerString
  pub fn _answer_string(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_answerString") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  pub fn answer_coding(&self) -> Option<Coding> {
    if let Some(val) = self.value.get("answerCoding") {
      return Some(Coding { value: val });
    }
    return None;
  }

  /// Extensions for question
  pub fn _question(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_question") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for answerBoolean
  pub fn _answer_boolean(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_answerBoolean") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The linkId for the question whose answer (or lack of answer) governs whether
  /// this item is enabled.
  pub fn question(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("question") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  pub fn answer_decimal(&self) -> Option<i64> {
    if let Some(val) = self.value.get("answerDecimal") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  pub fn answer_integer(&self) -> Option<i64> {
    if let Some(val) = self.value.get("answerInteger") {
      return Some(val.as_i64().unwrap());
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
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for answerDecimal
  pub fn _answer_decimal(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_answerDecimal") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for answerInteger
  pub fn _answer_integer(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_answerInteger") {
      return Some(Element { value: val });
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
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  pub fn answer_date(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("answerDate") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for answerDate
  pub fn _answer_date(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_answerDate") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  pub fn answer_date_time(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("answerDateTime") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for operator
  pub fn _operator(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_operator") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  pub fn answer_time(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("answerTime") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for answerTime
  pub fn _answer_time(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_answerTime") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  pub fn answer_quantity(&self) -> Option<Quantity> {
    if let Some(val) = self.value.get("answerQuantity") {
      return Some(Quantity { value: val });
    }
    return None;
  }

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  pub fn answer_reference(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("answerReference") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Extensions for answerDateTime
  pub fn _answer_date_time(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_answerDateTime") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  pub fn answer_boolean(&self) -> Option<bool> {
    if let Some(val) = self.value.get("answerBoolean") {
      return Some(val.as_bool().unwrap());
    }
    return None;
  }

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  pub fn answer_string(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("answerString") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Specifies the criteria by which the question is enabled.
  pub fn operator(&self) -> Option<Questionnaire_EnableWhenOperator> {
    if let Some(Value::String(val)) = self.value.get("operator") {
      return Some(Questionnaire_EnableWhenOperator::from_string(&val).unwrap());
    }
    return None;
  }

}

#[derive(Debug)]
pub enum Questionnaire_EnableWhenOperator {
  Exists,
  Equal,
  NotEqual,
  GreaterThan,
  LessThan,
  GreaterThanOrEqual,
  LessThanOrEqual,
}

impl Questionnaire_EnableWhenOperator {
    pub fn from_string(string: &str) -> Option<Questionnaire_EnableWhenOperator> {
      match string {
        "exists" => Some(Questionnaire_EnableWhenOperator::Exists),
        "=" => Some(Questionnaire_EnableWhenOperator::Equal),
        "!=" => Some(Questionnaire_EnableWhenOperator::NotEqual),
        ">" => Some(Questionnaire_EnableWhenOperator::GreaterThan),
        "<" => Some(Questionnaire_EnableWhenOperator::LessThan),
        ">=" => Some(Questionnaire_EnableWhenOperator::GreaterThanOrEqual),
        "<=" => Some(Questionnaire_EnableWhenOperator::LessThanOrEqual),
        _ => None,
    }
  }
}

