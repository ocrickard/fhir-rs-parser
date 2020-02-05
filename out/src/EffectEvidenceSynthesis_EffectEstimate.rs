use serde::{Deserialize, Serialize};

/// The EffectEvidenceSynthesis resource describes the difference in an outcome
/// between exposures states in a population where the effect estimate is derived
/// from a combination of research studies.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EffectEvidenceSynthesis_EffectEstimate {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Used to define variant exposure states such as low-risk state.
  #[serde(rename = "variantState")]
  variant_state: CodeableConcept,

  /// Extensions for description
  _description: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The point estimate of the effect estimate.
  value: decimal,

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

  /// Specifies the UCUM unit for the outcome.
  #[serde(rename = "unitOfMeasure")]
  unit_of_measure: CodeableConcept,

  /// Human-readable summary of effect estimate.
  description: String,

  /// A description of the precision of the estimate for the effect.
  #[serde(rename = "precisionEstimate")]
  precision_estimate: Vec<EffectEvidenceSynthesis_PrecisionEstimate>,

  /// Examples include relative risk and mean difference.
  type: CodeableConcept,

  /// Extensions for value
  _value: Element,

}