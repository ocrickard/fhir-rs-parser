#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Timing::Timing;
use crate::model::Identifier::Identifier;
use crate::model::SupplyDelivery_SuppliedItem::SupplyDelivery_SuppliedItem;
use crate::model::Period::Period;
use crate::model::Element::Element;
use crate::model::Meta::Meta;
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;


/// Record of delivery of what is supplied.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplyDelivery {
  /// The date or time(s) the activity occurred.
  #[serde(rename = "occurrencePeriod")]
  occurrence_period: Period,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for occurrenceDateTime
  #[serde(rename = "_occurrenceDateTime")]
  _occurrence_date_time: Element,

  /// Identification of the facility/location where the Supply was shipped to, as part
  /// of the dispense event.
  destination: Box<Reference>,

  /// Indicates the type of dispensing event that is performed. Examples include:
  /// Trial Fill, Completion of Trial, Partial Fill, Emergency Fill, Samples, etc.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Identifies the person who picked up the Supply.
  receiver: Vec<Box<Reference>>,

  /// Extensions for language
  _language: Element,

  /// The item that is being delivered or has been supplied.
  #[serde(rename = "suppliedItem")]
  supplied_item: SupplyDelivery_SuppliedItem,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for status
  _status: Element,

  /// The date or time(s) the activity occurred.
  #[serde(rename = "occurrenceTiming")]
  occurrence_timing: Timing,

  /// The date or time(s) the activity occurred.
  #[serde(rename = "occurrenceDateTime")]
  occurrence_date_time: String,

  /// A larger event of which this particular event is a component or step.
  #[serde(rename = "partOf")]
  part_of: Vec<Box<Reference>>,

  /// The individual responsible for dispensing the medication, supplier or device.
  supplier: Box<Reference>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// A plan, proposal or order that is fulfilled in whole or in part by this event.
  #[serde(rename = "basedOn")]
  based_on: Vec<Box<Reference>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Identifier for the supply delivery event that is used to identify it across
  /// multiple disparate systems.
  identifier: Vec<Identifier>,

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

  /// A code specifying the state of the dispense event.
  status: SupplyDeliveryStatus,

  /// A link to a resource representing the person whom the delivered item is for.
  patient: Box<Reference>,

  /// The base language in which the resource is written.
  language: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum SupplyDeliveryStatus {
  #[serde(rename = "in-progress")]
  InProgress,

  #[serde(rename = "completed")]
  Completed,

  #[serde(rename = "abandoned")]
  Abandoned,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}