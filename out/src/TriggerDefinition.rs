use serde::{Deserialize, Serialize};

/// A description of a triggering event. Triggering events can be named events, data
/// events, or periodic, as determined by the type element.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TriggerDefinition {
  /// The timing of the event (if this is a periodic trigger).
  #[serde(rename = "timingReference")]
  timing_reference: Reference,

  /// The timing of the event (if this is a periodic trigger).
  #[serde(rename = "timingDate")]
  timing_date: String,

  /// The timing of the event (if this is a periodic trigger).
  #[serde(rename = "timingDateTime")]
  timing_date_time: String,

  /// Extensions for timingDate
  #[serde(rename = "_timingDate")]
  _timing_date: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The type of triggering event.
  type: TriggerDefinitionType,

  /// Extensions for type
  _type: Element,

  /// A formal name for the event. This may be an absolute URI that identifies the
  /// event formally (e.g. from a trigger registry), or a simple relative URI that
  /// identifies the event in a local context.
  name: String,

  /// The timing of the event (if this is a periodic trigger).
  #[serde(rename = "timingTiming")]
  timing_timing: Timing,

  /// A boolean-valued expression that is evaluated in the context of the container of
  /// the trigger definition and returns whether or not the trigger fires.
  condition: Expression,

  /// The triggering data of the event (if this is a data trigger). If more than one
  /// data is requirement is specified, then all the data requirements must be true.
  data: Vec<DataRequirement>,

  /// Extensions for timingDateTime
  #[serde(rename = "_timingDateTime")]
  _timing_date_time: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for name
  _name: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum TriggerDefinitionType {
  #[serde(rename = "named-event")]
  NamedEvent,

  #[serde(rename = "periodic")]
  Periodic,

  #[serde(rename = "data-changed")]
  DataChanged,

  #[serde(rename = "data-added")]
  DataAdded,

  #[serde(rename = "data-modified")]
  DataModified,

  #[serde(rename = "data-removed")]
  DataRemoved,

  #[serde(rename = "data-accessed")]
  DataAccessed,

  #[serde(rename = "data-access-ended")]
  DataAccessEnded,

}
