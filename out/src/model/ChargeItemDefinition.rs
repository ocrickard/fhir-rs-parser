#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ChargeItemDefinition_Applicability::ChargeItemDefinition_Applicability;
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::UsageContext::UsageContext;
use crate::model::ChargeItemDefinition_PropertyGroup::ChargeItemDefinition_PropertyGroup;
use crate::model::Identifier::Identifier;
use crate::model::Period::Period;
use crate::model::Extension::Extension;
use crate::model::ResourceList::ResourceList;
use crate::model::Reference::Reference;
use crate::model::ContactDetail::ContactDetail;


/// The ChargeItemDefinition resource provides the properties that apply to the
/// (billing) codes necessary to calculate costs and prices. The properties may
/// differ largely depending on type and realm, therefore this resource gives only a
/// rough structure and requires profiling for each type of billing code system.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargeItemDefinition {
  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// A Boolean value to indicate that this charge item definition is authored for
  /// testing purposes (or education/evaluation/marketing) and is not intended to be
  /// used for genuine usage.
  experimental: Option<bool>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// The defined billing details in this resource pertain to the given billing code.
  code: Option<CodeableConcept>,

  /// The defined billing details in this resource pertain to the given product
  /// instance(s).
  instance: Option<Vec<Box<Reference>>>,

  /// Extensions for approvalDate
  #[serde(rename = "_approvalDate")]
  _approval_date: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// A larger definition of which this particular definition is a component or step.
  #[serde(rename = "partOf")]
  part_of: Option<Vec<String>>,

  /// A free text natural language description of the charge item definition from a
  /// consumer's perspective.
  description: Option<String>,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate charge item
  /// definition instances.
  #[serde(rename = "useContext")]
  use_context: Option<Vec<UsageContext>>,

  /// Extensions for derivedFromUri
  #[serde(rename = "_derivedFromUri")]
  _derived_from_uri: Option<Vec<Element>>,

  /// As new versions of a protocol or guideline are defined, allows identification of
  /// what versions are replaced by a new instance.
  replaces: Option<Vec<String>>,

  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for publisher
  #[serde(rename = "_publisher")]
  _publisher: Option<Element>,

  /// The date on which the resource content was last reviewed. Review happens
  /// periodically after approval but does not change the original approval date.
  #[serde(rename = "lastReviewDate")]
  last_review_date: Option<i32>,

  /// Expressions that describe applicability criteria for the billing code.
  applicability: Option<Vec<ChargeItemDefinition_Applicability>>,

  /// An absolute URI that is used to identify this charge item definition when it is
  /// referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this charge item
  /// definition is (or will be) published. This URL can be the target of a canonical
  /// reference. It SHALL remain the same when the charge item definition is stored on
  /// different servers.
  url: Option<String>,

  /// A short, descriptive, user-friendly title for the charge item definition.
  title: Option<String>,

  /// A legal or geographic region in which the charge item definition is intended to
  /// be used.
  jurisdiction: Option<Vec<CodeableConcept>>,

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
  modifier_extension: Option<Vec<Extension>>,

  /// Extensions for copyright
  #[serde(rename = "_copyright")]
  _copyright: Option<Element>,

  /// The period during which the charge item definition content was or is planned to
  /// be in active use.
  #[serde(rename = "effectivePeriod")]
  effective_period: Option<Period>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

  /// The current state of the ChargeItemDefinition.
  status: Option<ChargeItemDefinitionStatus>,

  /// The URL pointing to an externally-defined charge item definition that is adhered
  /// to in whole or in part by this definition.
  #[serde(rename = "derivedFromUri")]
  derived_from_uri: Option<Vec<String>>,

  /// The date  (and optionally time) when the charge item definition was published.
  /// The date must change when the business version changes and it must change if the
  /// status code changes. In addition, it should change when the substantive content
  /// of the charge item definition changes.
  date: Option<String>,

  /// Group of properties which are applicable under the same conditions. If no
  /// applicability rules are established for the group, then all properties always
  /// apply.
  #[serde(rename = "propertyGroup")]
  property_group: Option<Vec<ChargeItemDefinition_PropertyGroup>>,

  /// The date on which the resource content was approved by the publisher. Approval
  /// happens once when the content is officially approved for usage.
  #[serde(rename = "approvalDate")]
  approval_date: Option<i32>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// A formal identifier that is used to identify this charge item definition when it
  /// is represented in other formats, or referenced in a specification, model, design
  /// or an instance.
  identifier: Option<Vec<Identifier>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Extensions for experimental
  #[serde(rename = "_experimental")]
  _experimental: Option<Element>,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Option<Vec<ContactDetail>>,

  /// A copyright statement relating to the charge item definition and/or its
  /// contents. Copyright statements are generally legal restrictions on the use and
  /// publishing of the charge item definition.
  copyright: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// Extensions for lastReviewDate
  #[serde(rename = "_lastReviewDate")]
  _last_review_date: Option<Element>,

  /// The identifier that is used to identify this version of the charge item
  /// definition when it is referenced in a specification, model, design or instance.
  /// This is an arbitrary value managed by the charge item definition author and is
  /// not expected to be globally unique. For example, it might be a timestamp (e.g.
  /// yyyymmdd) if a managed version is not available. There is also no expectation
  /// that versions can be placed in a lexicographical sequence. To provide a version
  /// consistent with the Decision Support Service specification, use the format
  /// Major.Minor.Revision (e.g. 1.0.0). For more information on versioning knowledge
  /// assets, refer to the Decision Support Service specification. Note that a version
  /// is required for non-experimental active assets.
  version: Option<String>,

  /// The name of the organization or individual that published the charge item
  /// definition.
  publisher: Option<String>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChargeItemDefinitionStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
