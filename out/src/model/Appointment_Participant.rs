#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Period::Period;
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one or
/// more Encounter(s).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Appointment_Participant {
  /// A Person, Location/HealthcareService or Device that is participating in the
  /// appointment.
  actor: Box<Reference>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Participation status of the actor.
  status: Appointment_ParticipantStatus,

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
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// Extensions for required
  _required: Element,

  /// Extensions for status
  _status: Element,

  /// Participation period of the actor.
  period: Period,

  /// Role of participant in the appointment.
  #[serde(rename = "type")]
  fhir_type: Vec<CodeableConcept>,

  /// Whether this participant is required to be present at the meeting. This covers a
  /// use-case where two doctors need to meet to discuss the results for a specific
  /// patient, and the patient is not required to be present.
  required: Appointment_ParticipantRequired,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Appointment_ParticipantStatus {
  #[serde(rename = "accepted")]
  Accepted,

  #[serde(rename = "declined")]
  Declined,

  #[serde(rename = "tentative")]
  Tentative,

  #[serde(rename = "needs-action")]
  NeedsAction,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Appointment_ParticipantRequired {
  #[serde(rename = "required")]
  Required,

  #[serde(rename = "optional")]
  Optional,

  #[serde(rename = "information-only")]
  InformationOnly,

}