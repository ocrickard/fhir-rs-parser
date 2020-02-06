#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Annotation::Annotation;
use crate::model::Reference::Reference;
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Identifier::Identifier;
use crate::model::Timing::Timing;
use crate::model::ResourceList::ResourceList;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Period::Period;


/// A record of a device being used by a patient where the record is the result of a
/// report from the patient or another clinician.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceUseStatement {
  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Extensions for recordedOn
  #[serde(rename = "_recordedOn")]
  _recorded_on: Element,

  /// Extensions for status
  _status: Element,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Allows linking the DeviceUseStatement to the underlying Request, or to other
  /// information that supports or is used to derive the DeviceUseStatement.
  #[serde(rename = "derivedFrom")]
  derived_from: Vec<Box<Reference>>,

  /// How often the device was used.
  #[serde(rename = "timingTiming")]
  timing_timing: Timing,

  /// How often the device was used.
  #[serde(rename = "timingPeriod")]
  timing_period: Period,

  /// An external identifier for this statement such as an IRI.
  identifier: Vec<Identifier>,

  /// Extensions for timingDateTime
  #[serde(rename = "_timingDateTime")]
  _timing_date_time: Element,

  /// Indicates the anotomic location on the subject's body where the device was used
  /// ( i.e. the target).
  #[serde(rename = "bodySite")]
  body_site: CodeableConcept,

  /// Details about the device statement that were not represented at all or
  /// sufficiently in one of the attributes provided in a class. These may include for
  /// example a comment, an instruction, or a note associated with the statement.
  note: Vec<Annotation>,

  /// The time at which the statement was made/recorded.
  #[serde(rename = "recordedOn")]
  recorded_on: String,

  /// A plan, proposal or order that is fulfilled in whole or in part by this
  /// DeviceUseStatement.
  #[serde(rename = "basedOn")]
  based_on: Vec<Box<Reference>>,

  /// Who reported the device was being used by the patient.
  source: Box<Reference>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The base language in which the resource is written.
  language: String,

  /// The details of the device used.
  device: Box<Reference>,

  /// Extensions for language
  _language: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// Indicates another resource whose existence justifies this DeviceUseStatement.
  #[serde(rename = "reasonReference")]
  reason_reference: Vec<Box<Reference>>,

  /// Reason or justification for the use of the device.
  #[serde(rename = "reasonCode")]
  reason_code: Vec<CodeableConcept>,

  /// A code representing the patient or other source's judgment about the state of
  /// the device used that this statement is about.  Generally this will be active or
  /// completed.
  status: DeviceUseStatementStatus,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The patient who used the device.
  subject: Box<Reference>,

  /// How often the device was used.
  #[serde(rename = "timingDateTime")]
  timing_date_time: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceUseStatementStatus {
  #[serde(rename = "active")]
  Active,

  #[serde(rename = "completed")]
  Completed,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

  #[serde(rename = "intended")]
  Intended,

  #[serde(rename = "stopped")]
  Stopped,

  #[serde(rename = "on-hold")]
  OnHold,

}