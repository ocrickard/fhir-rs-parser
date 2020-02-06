#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Timing::Timing;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Period::Period;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Identifier::Identifier;
use crate::model::Annotation::Annotation;
use crate::model::ChargeItem_Performer::ChargeItem_Performer;
use crate::model::Money::Money;


/// The resource ChargeItem describes the provision of healthcare provider products
/// for a certain patient, therefore referring not only to the product, but
/// containing in addition details of the provision, like date, time, amounts and
/// participating organizations and persons. Main Usage of the ChargeItem is to
/// enable the billing process and internal cost allocation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargeItem {
  /// Extensions for occurrenceDateTime
  #[serde(rename = "_occurrenceDateTime")]
  _occurrence_date_time: Element,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// If the list price or the rule-based factor associated with the code is
  /// overridden, this attribute can capture a text to indicate the  reason for this
  /// action.
  #[serde(rename = "overrideReason")]
  override_reason: String,

  /// Describes why the event occurred in coded or textual form.
  reason: Vec<CodeableConcept>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Account into which this ChargeItems belongs.
  account: Vec<Box<Reference>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for status
  _status: Element,

  /// Comments made about the event by the performer, subject or other participants.
  note: Vec<Annotation>,

  /// Extensions for language
  _language: Element,

  /// The organization requesting the service.
  #[serde(rename = "performingOrganization")]
  performing_organization: Box<Reference>,

  /// The current state of the ChargeItem.
  status: ChargeItemStatus,

  /// Identifies the device, food, drug or other product being charged either by type
  /// code or reference to an instance.
  #[serde(rename = "productReference")]
  product_reference: Box<Reference>,

  /// Extensions for definitionUri
  #[serde(rename = "_definitionUri")]
  _definition_uri: Vec<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Date/time(s) or duration when the charged service was applied.
  #[serde(rename = "occurrencePeriod")]
  occurrence_period: Period,

  /// The encounter or episode of care that establishes the context for this event.
  context: Box<Reference>,

  /// Further information supporting this charge.
  #[serde(rename = "supportingInformation")]
  supporting_information: Vec<Box<Reference>>,

  /// The anatomical location where the related service has been applied.
  bodysite: Vec<CodeableConcept>,

  /// References the (external) source of pricing information, rules of application
  /// for the code this ChargeItem uses.
  #[serde(rename = "definitionUri")]
  definition_uri: Vec<String>,

  /// Factor overriding the factor determined by the rules associated with the code.
  #[serde(rename = "factorOverride")]
  factor_override: f32,

  /// The organization performing the service.
  #[serde(rename = "requestingOrganization")]
  requesting_organization: Box<Reference>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

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
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// References the source of pricing information, rules of application for the code
  /// this ChargeItem uses.
  #[serde(rename = "definitionCanonical")]
  definition_canonical: Vec<String>,

  /// A code that identifies the charge, like a billing code.
  code: CodeableConcept,

  /// Quantity of which the charge item has been serviced.
  quantity: Quantity,

  /// Extensions for overrideReason
  #[serde(rename = "_overrideReason")]
  _override_reason: Element,

  /// Identifiers assigned to this event performer or other systems.
  identifier: Vec<Identifier>,

  /// Indicates who or what performed or participated in the charged service.
  performer: Vec<ChargeItem_Performer>,

  /// Total price of the charge overriding the list price associated with the code.
  #[serde(rename = "priceOverride")]
  price_override: Money,

  /// Extensions for factorOverride
  #[serde(rename = "_factorOverride")]
  _factor_override: Element,

  /// The device, practitioner, etc. who entered the charge item.
  enterer: Box<Reference>,

  /// Date/time(s) or duration when the charged service was applied.
  #[serde(rename = "occurrenceTiming")]
  occurrence_timing: Timing,

  /// The individual or set of individuals the action is being or was performed on.
  subject: Box<Reference>,

  /// Indicated the rendered service that caused this charge.
  service: Vec<Box<Reference>>,

  /// Date/time(s) or duration when the charged service was applied.
  #[serde(rename = "occurrenceDateTime")]
  occurrence_date_time: String,

  /// ChargeItems can be grouped to larger ChargeItems covering the whole set.
  #[serde(rename = "partOf")]
  part_of: Vec<Box<Reference>>,

  /// The financial cost center permits the tracking of charge attribution.
  #[serde(rename = "costCenter")]
  cost_center: Box<Reference>,

  /// Identifies the device, food, drug or other product being charged either by type
  /// code or reference to an instance.
  #[serde(rename = "productCodeableConcept")]
  product_codeable_concept: CodeableConcept,

  /// Date the charge item was entered.
  #[serde(rename = "enteredDate")]
  entered_date: String,

  /// Extensions for enteredDate
  #[serde(rename = "_enteredDate")]
  _entered_date: Element,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChargeItemStatus {
  #[serde(rename = "planned")]
  Planned,

  #[serde(rename = "billable")]
  Billable,

  #[serde(rename = "not-billable")]
  NotBillable,

  #[serde(rename = "aborted")]
  Aborted,

  #[serde(rename = "billed")]
  Billed,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

  #[serde(rename = "unknown")]
  Unknown,

}