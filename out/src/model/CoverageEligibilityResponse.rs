#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Period::Period;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Identifier::Identifier;
use crate::model::ResourceList::ResourceList;
use crate::model::Meta::Meta;
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::CoverageEligibilityResponse_Error::CoverageEligibilityResponse_Error;
use crate::model::CoverageEligibilityResponse_Insurance::CoverageEligibilityResponse_Insurance;
use crate::model::Element::Element;


/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponse {
  /// The party who is the beneficiary of the supplied coverage and for whom
  /// eligibility is sought.
  patient: Box<Reference>,

  /// The date or dates when the enclosed suite of services were performed or
  /// completed.
  #[serde(rename = "servicedDate")]
  serviced_date: String,

  /// Extensions for disposition
  _disposition: Element,

  /// Extensions for status
  _status: Element,

  /// The Insurer who issued the coverage in question and is the author of the
  /// response.
  insurer: Box<Reference>,

  /// Financial instruments for reimbursement for the health care products and
  /// services.
  insurance: Vec<CoverageEligibilityResponse_Insurance>,

  /// Errors encountered during the processing of the request.
  error: Vec<CoverageEligibilityResponse_Error>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Extensions for servicedDate
  #[serde(rename = "_servicedDate")]
  _serviced_date: Element,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for language
  _language: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for purpose
  _purpose: Vec<Element>,

  /// A human readable description of the status of the adjudication.
  disposition: String,

  /// Extensions for created
  _created: Element,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The provider which is responsible for the request.
  requestor: Box<Reference>,

  /// A reference from the Insurer to which these services pertain to be used on
  /// further communication and as proof that the request occurred.
  #[serde(rename = "preAuthRef")]
  pre_auth_ref: String,

  /// The date or dates when the enclosed suite of services were performed or
  /// completed.
  #[serde(rename = "servicedPeriod")]
  serviced_period: Period,

  /// The outcome of the request processing.
  outcome: CoverageEligibilityResponseOutcome,

  /// Extensions for outcome
  _outcome: Element,

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

  /// Reference to the original request resource.
  request: Box<Reference>,

  /// A unique identifier assigned to this coverage eligiblity request.
  identifier: Vec<Identifier>,

  /// A code for the form to be used for printing the content.
  form: CodeableConcept,

  /// Extensions for preAuthRef
  #[serde(rename = "_preAuthRef")]
  _pre_auth_ref: Element,

  /// The date this resource was created.
  created: String,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The status of the resource instance.
  status: String,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CoverageEligibilityResponseOutcome {
  #[serde(rename = "queued")]
  Queued,

  #[serde(rename = "complete")]
  Complete,

  #[serde(rename = "error")]
  Error,

  #[serde(rename = "partial")]
  Partial,

}