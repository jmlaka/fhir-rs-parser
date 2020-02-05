use serde::{Deserialize, Serialize};

/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity. The device may be a
/// medical or non-medical device.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Device_UdiCarrier {
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

  /// The device identifier (DI) is a mandatory, fixed portion of a UDI that
  /// identifies the labeler and the specific version or model of a device.
  #[serde(rename = "deviceIdentifier")]
  device_identifier: String,

  /// Extensions for jurisdiction
  _jurisdiction: Element,

  /// The full UDI carrier as the human readable form (HRF) representation of the
  /// barcode string as printed on the packaging of the device.
  #[serde(rename = "carrierHRF")]
  carrier_h_r_f: String,

  /// A coded entry to indicate how the data was entered.
  #[serde(rename = "entryType")]
  entry_type: Device_UdiCarrierEntryType,

  /// Organization that is charged with issuing UDIs for devices.  For example, the US
  /// FDA issuers include :
  /// 1) GS1: 
  /// http://hl7.org/fhir/NamingSystem/gs1-di, 
  /// 2) HIBCC:
  /// http://hl7.org/fhir/NamingSystem/hibcc-dI, 
  /// 3) ICCBBA for blood containers:
  /// http://hl7.org/fhir/NamingSystem/iccbba-blood-di, 
  /// 4) ICCBA for other devices:
  /// http://hl7.org/fhir/NamingSystem/iccbba-other-di.
  issuer: String,

  /// The full UDI carrier of the Automatic Identification and Data Capture (AIDC)
  /// technology representation of the barcode string as printed on the packaging of
  /// the device - e.g., a barcode or RFID.   Because of limitations on character sets
  /// in XML and the need to round-trip JSON data through XML, AIDC Formats *SHALL* be
  /// base64 encoded.
  #[serde(rename = "carrierAIDC")]
  carrier_a_i_d_c: base64Binary,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for carrierHRF
  #[serde(rename = "_carrierHRF")]
  _carrier_h_r_f: Element,

  /// Extensions for deviceIdentifier
  #[serde(rename = "_deviceIdentifier")]
  _device_identifier: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for carrierAIDC
  #[serde(rename = "_carrierAIDC")]
  _carrier_a_i_d_c: Element,

  /// The identity of the authoritative source for UDI generation within a
  /// jurisdiction.  All UDIs are globally unique within a single namespace with the
  /// appropriate repository uri as the system.  For example,  UDIs of devices managed
  /// in the U.S. by the FDA, the value is  http://hl7.org/fhir/NamingSystem/fda-udi.
  jurisdiction: String,

  /// Extensions for issuer
  _issuer: Element,

  /// Extensions for entryType
  #[serde(rename = "_entryType")]
  _entry_type: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum Device_UdiCarrierEntryType {
  #[serde(rename = "barcode")]
  Barcode,

  #[serde(rename = "rfid")]
  Rfid,

  #[serde(rename = "manual")]
  Manual,

  #[serde(rename = "card")]
  Card,

  #[serde(rename = "self-reported")]
  SelfReported,

  #[serde(rename = "unknown")]
  Unknown,

}
