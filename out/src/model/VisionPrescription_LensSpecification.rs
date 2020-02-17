#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::VisionPrescription_Prism::VisionPrescription_Prism;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.

#[derive(Debug)]
pub struct VisionPrescription_LensSpecification<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl VisionPrescription_LensSpecification<'_> {
    /// Extensions for add
    pub fn _add(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_add") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for axis
    pub fn _axis(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_axis") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for backCurve
    pub fn _back_curve(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_backCurve") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for brand
    pub fn _brand(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_brand") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for color
    pub fn _color(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_color") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for cylinder
    pub fn _cylinder(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_cylinder") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for diameter
    pub fn _diameter(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_diameter") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for eye
    pub fn _eye(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_eye") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for power
    pub fn _power(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_power") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for sphere
    pub fn _sphere(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sphere") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Power adjustment for multifocal lenses measured in dioptres (0.25 units).
    pub fn add(&self) -> Option<f64> {
        if let Some(val) = self.value.get("add") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Adjustment for astigmatism measured in integer degrees.
    pub fn axis(&self) -> Option<i64> {
        if let Some(val) = self.value.get("axis") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Back curvature measured in millimetres.
    pub fn back_curve(&self) -> Option<f64> {
        if let Some(val) = self.value.get("backCurve") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Brand recommendations or restrictions.
    pub fn brand(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("brand") {
            return Some(string);
        }
        return None;
    }

    /// Special color or pattern.
    pub fn color(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("color") {
            return Some(string);
        }
        return None;
    }

    /// Power adjustment for astigmatism measured in dioptres (0.25 units).
    pub fn cylinder(&self) -> Option<f64> {
        if let Some(val) = self.value.get("cylinder") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Contact lens diameter measured in millimetres.
    pub fn diameter(&self) -> Option<f64> {
        if let Some(val) = self.value.get("diameter") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The recommended maximum wear period for the lens.
    pub fn duration(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("duration") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
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

    /// The eye for which the lens specification applies.
    pub fn eye(&self) -> Option<VisionPrescription_LensSpecificationEye> {
        if let Some(Value::String(val)) = self.value.get("eye") {
            return Some(VisionPrescription_LensSpecificationEye::from_string(&val).unwrap());
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

    /// Notes for special requirements such as coatings and lens materials.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Contact lens power measured in dioptres (0.25 units).
    pub fn power(&self) -> Option<f64> {
        if let Some(val) = self.value.get("power") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Allows for adjustment on two axis.
    pub fn prism(&self) -> Option<Vec<VisionPrescription_Prism>> {
        if let Some(Value::Array(val)) = self.value.get("prism") {
            return Some(
                val.into_iter()
                    .map(|e| VisionPrescription_Prism {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Identifies the type of vision correction product which is required for the
    /// patient.
    pub fn product(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["product"]),
        }
    }

    /// Lens power measured in dioptres (0.25 units).
    pub fn sphere(&self) -> Option<f64> {
        if let Some(val) = self.value.get("sphere") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._add() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._axis() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._back_curve() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._brand() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._color() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._cylinder() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._diameter() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._eye() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._power() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._sphere() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.add() {}
        if let Some(_val) = self.axis() {}
        if let Some(_val) = self.back_curve() {}
        if let Some(_val) = self.brand() {}
        if let Some(_val) = self.color() {}
        if let Some(_val) = self.cylinder() {}
        if let Some(_val) = self.diameter() {}
        if let Some(_val) = self.duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.eye() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.power() {}
        if let Some(_val) = self.prism() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.product().validate() {
            return false;
        }
        if let Some(_val) = self.sphere() {}
        return true;
    }
}

#[derive(Debug)]
pub struct VisionPrescription_LensSpecificationBuilder {
    pub value: Value,
}

impl VisionPrescription_LensSpecificationBuilder {
    pub fn build(&self) -> VisionPrescription_LensSpecification {
        VisionPrescription_LensSpecification {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn new(product: CodeableConcept) -> VisionPrescription_LensSpecificationBuilder {
        let mut __value: Value = json!({});
        __value["product"] = json!(product.value);
        return VisionPrescription_LensSpecificationBuilder { value: __value };
    }
}

#[derive(Debug)]
pub enum VisionPrescription_LensSpecificationEye {
    Right,
    Left,
}

impl VisionPrescription_LensSpecificationEye {
    pub fn from_string(string: &str) -> Option<VisionPrescription_LensSpecificationEye> {
        match string {
            "right" => Some(VisionPrescription_LensSpecificationEye::Right),
            "left" => Some(VisionPrescription_LensSpecificationEye::Left),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            VisionPrescription_LensSpecificationEye::Right => "right".to_string(),
            VisionPrescription_LensSpecificationEye::Left => "left".to_string(),
        }
    }
}
