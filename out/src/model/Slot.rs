#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Narrative::Narrative;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ResourceList::ResourceList;
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Element::Element;


/// A slot of time on a schedule that may be available for booking appointments.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
  /// The type of appointments that can be booked into this slot (ideally this would
  /// be an identifiable service - which is at a location, rather than the location
  /// itself). If provided then this overrides the value provided on the availability
  /// resource.
  #[serde(rename = "serviceType")]
  service_type: Vec<CodeableConcept>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The specialty of a practitioner that would be required to perform the service
  /// requested in this appointment.
  specialty: Vec<CodeableConcept>,

  /// The schedule resource that this slot defines an interval of status information.
  schedule: Box<Reference>,

  /// busy | free | busy-unavailable | busy-tentative | entered-in-error.
  status: SlotStatus,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// The style of appointment or patient that may be booked in the slot (not service
  /// type).
  #[serde(rename = "appointmentType")]
  appointment_type: CodeableConcept,

  /// Date/Time that the slot is to begin.
  start: i32,

  /// Extensions for end
  _end: Element,

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

  /// Comments on the slot to describe any extended information. Such as custom
  /// constraints on the slot.
  comment: String,

  /// Extensions for comment
  _comment: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for language
  _language: Element,

  /// Extensions for status
  _status: Element,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for overbooked
  _overbooked: Element,

  /// External Ids for this item.
  identifier: Vec<Identifier>,

  /// This slot has already been overbooked, appointments are unlikely to be accepted
  /// for this time.
  overbooked: bool,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for start
  _start: Element,

  /// Date/Time that the slot is to conclude.
  end: i32,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A broad categorization of the service that is to be performed during this
  /// appointment.
  #[serde(rename = "serviceCategory")]
  service_category: Vec<CodeableConcept>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum SlotStatus {
  #[serde(rename = "busy")]
  Busy,

  #[serde(rename = "free")]
  Free,

  #[serde(rename = "busy-unavailable")]
  BusyUnavailable,

  #[serde(rename = "busy-tentative")]
  BusyTentative,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}