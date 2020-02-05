use serde::{Deserialize, Serialize};

/// A sample to be used for analysis.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Specimen_Processing {
  /// A coded value specifying the procedure used to process the specimen.
  procedure: CodeableConcept,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Material used in the processing step.
  additive: Vec<Reference>,

  /// Textual description of procedure.
  description: String,

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

  /// Extensions for description
  _description: Element,

  /// A record of the time or period when the specimen processing occurred.  For
  /// example the time of sample fixation or the period of time the sample was in
  /// formalin.
  #[serde(rename = "timeDateTime")]
  time_date_time: String,

  /// Extensions for timeDateTime
  #[serde(rename = "_timeDateTime")]
  _time_date_time: Element,

  /// A record of the time or period when the specimen processing occurred.  For
  /// example the time of sample fixation or the period of time the sample was in
  /// formalin.
  #[serde(rename = "timePeriod")]
  time_period: Period,

}