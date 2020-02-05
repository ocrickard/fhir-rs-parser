use serde::{Deserialize, Serialize};

/// An assessment of the likely outcome(s) for a patient or other subject as well as
/// the likelihood of each outcome.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RiskAssessment {
  /// The type of the risk assessment performed.
  code: CodeableConcept,

  /// The patient or group the risk assessment applies to.
  subject: Reference,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The status of the RiskAssessment, using the same statuses as an Observation.
  status: String,

  /// Business identifier assigned to the risk assessment.
  identifier: Vec<Identifier>,

  /// For assessments or prognosis specific to a particular condition, indicates the
  /// condition being assessed.
  condition: Reference,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// The encounter where the assessment was performed.
  encounter: Reference,

  /// Describes the expected outcome for the subject.
  prediction: Vec<RiskAssessment_Prediction>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// A reference to the request that is fulfilled by this risk assessment.
  #[serde(rename = "basedOn")]
  based_on: Reference,

  /// Extensions for language
  _language: Element,

  /// The date (and possibly time) the risk assessment was performed.
  #[serde(rename = "occurrencePeriod")]
  occurrence_period: Period,

  /// The algorithm, process or mechanism used to evaluate the risk.
  method: CodeableConcept,

  /// Indicates the source data considered as part of the assessment (for example,
  /// FamilyHistory, Observations, Procedures, Conditions, etc.).
  basis: Vec<Reference>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Resources supporting the reason the risk assessment was performed.
  #[serde(rename = "reasonReference")]
  reason_reference: Vec<Reference>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// A description of the steps that might be taken to reduce the identified risk(s).
  mitigation: String,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for status
  _status: Element,

  /// Extensions for occurrenceDateTime
  #[serde(rename = "_occurrenceDateTime")]
  _occurrence_date_time: Element,

  /// The base language in which the resource is written.
  language: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The date (and possibly time) the risk assessment was performed.
  #[serde(rename = "occurrenceDateTime")]
  occurrence_date_time: String,

  /// The provider or software application that performed the assessment.
  performer: Reference,

  /// Additional comments about the risk assessment.
  note: Vec<Annotation>,

  /// The reason the risk assessment was performed.
  #[serde(rename = "reasonCode")]
  reason_code: Vec<CodeableConcept>,

  /// A reference to a resource that this risk assessment is part of, such as a
  /// Procedure.
  parent: Reference,

  /// Extensions for mitigation
  _mitigation: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

}