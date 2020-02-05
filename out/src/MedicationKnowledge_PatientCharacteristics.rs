use serde::{Deserialize, Serialize};

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicationKnowledge_PatientCharacteristics {
  /// Extensions for value
  _value: Vec<Element>,

  /// The specific characteristic (e.g. height, weight, gender, etc.).
  value: Vec<String>,

  /// Specific characteristic that is relevant to the administration guideline (e.g.
  /// height, weight, gender).
  #[serde(rename = "characteristicCodeableConcept")]
  characteristic_codeable_concept: CodeableConcept,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

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

  /// Specific characteristic that is relevant to the administration guideline (e.g.
  /// height, weight, gender).
  #[serde(rename = "characteristicQuantity")]
  characteristic_quantity: Quantity,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

}