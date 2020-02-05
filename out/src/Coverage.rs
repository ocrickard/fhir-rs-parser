use serde::{Deserialize, Serialize};

/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. Includes both insurance and self-payment.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Coverage {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A suite of codes indicating the cost category and associated amount which have
  /// been detailed in the policy and may have been  included on the health card.
  #[serde(rename = "costToBeneficiary")]
  cost_to_beneficiary: Vec<Coverage_CostToBeneficiary>,

  /// The party who benefits from the insurance coverage; the patient when products
  /// and/or services are provided.
  beneficiary: Reference,

  /// The insurer assigned ID for the Subscriber.
  #[serde(rename = "subscriberId")]
  subscriber_id: String,

  /// The insurer-specific identifier for the insurer-defined network of providers to
  /// which the beneficiary may seek treatment which will be covered at the 'in-
  /// network' rate, otherwise 'out of network' terms and conditions apply.
  network: String,

  /// The type of coverage: social program, medical plan, accident coverage (workers
  /// compensation, auto), group health or payment by an individual or organization.
  type: CodeableConcept,

  /// Time period during which the coverage is in force. A missing start date
  /// indicates the start date isn't known, a missing end date means the coverage is
  /// continuing to be in force.
  period: Period,

  /// The status of the resource instance.
  status: String,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for language
  _language: Element,

  /// Extensions for status
  _status: Element,

  /// When 'subrogation=true' this insurance instance has been included not for
  /// adjudication but to provide insurers with the details to recover costs.
  subrogation: bool,

  /// Extensions for network
  _network: Element,

  /// The policy(s) which constitute this insurance coverage.
  contract: Vec<Reference>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// A unique identifier assigned to this coverage.
  identifier: Vec<Identifier>,

  /// The order of applicability of this coverage relative to other coverages which
  /// are currently in force. Note, there may be gaps in the numbering and this does
  /// not imply primary, secondary etc. as the specific positioning of coverages
  /// depends upon the episode of care.
  order: positiveInt,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for order
  _order: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for subrogation
  _subrogation: Element,

  /// The party who has signed-up for or 'owns' the contractual relationship to the
  /// policy or to whom the benefit of the policy for services rendered to them or
  /// their family is due.
  subscriber: Reference,

  /// The relationship of beneficiary (patient) to the subscriber.
  relationship: CodeableConcept,

  /// The program or plan underwriter or payor including both insurance and non-
  /// insurance agreements, such as patient-pay agreements.
  payor: Vec<Reference>,

  /// Extensions for dependent
  _dependent: Element,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

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

  /// A suite of underwriter specific classifiers.
  class: Vec<Coverage_Class>,

  /// Extensions for subscriberId
  #[serde(rename = "_subscriberId")]
  _subscriber_id: Element,

  /// A unique identifier for a dependent under the coverage.
  dependent: String,

  /// The party who 'owns' the insurance policy.
  #[serde(rename = "policyHolder")]
  policy_holder: Reference,

}