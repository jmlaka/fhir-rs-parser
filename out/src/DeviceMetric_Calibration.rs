use serde::{Deserialize, Serialize};

/// Describes a measurement, calculation or setting capability of a medical device.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceMetric_Calibration {
  /// Describes the type of the calibration method.
  type: DeviceMetric_CalibrationType,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Describes the state of the calibration.
  state: DeviceMetric_CalibrationState,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// Extensions for state
  _state: Element,

  /// Extensions for time
  _time: Element,

  /// Describes the time last calibration has been performed.
  time: instant,

  /// Extensions for type
  _type: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum DeviceMetric_CalibrationType {
  #[serde(rename = "unspecified")]
  Unspecified,

  #[serde(rename = "offset")]
  Offset,

  #[serde(rename = "gain")]
  Gain,

  #[serde(rename = "two-point")]
  TwoPoint,

}

#[derive(Debug, Serialize, Deserialize)]
enum DeviceMetric_CalibrationState {
  #[serde(rename = "not-calibrated")]
  NotCalibrated,

  #[serde(rename = "calibration-required")]
  CalibrationRequired,

  #[serde(rename = "calibrated")]
  Calibrated,

  #[serde(rename = "unspecified")]
  Unspecified,

}
