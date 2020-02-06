#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Address::Address;
use crate::model::RelatedPerson_Communication::RelatedPerson_Communication;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Period::Period;
use crate::model::ResourceList::ResourceList;
use crate::model::Identifier::Identifier;
use crate::model::HumanName::HumanName;
use crate::model::Reference::Reference;
use crate::model::Meta::Meta;
use crate::model::Attachment::Attachment;


/// Information about a person that is involved in the care for a patient, but who
/// is not the target of healthcare, nor has a formal responsibility in the care
/// process.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedPerson {
  /// A language which may be used to communicate with about the patient's health.
  communication: Vec<RelatedPerson_Communication>,

  /// Address where the related person can be contacted or visited.
  address: Vec<Address>,

  /// Extensions for active
  _active: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// A name associated with the person.
  name: Vec<HumanName>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

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

  /// The period of time during which this relationship is or was active. If there are
  /// no dates defined, then the interval is unknown.
  period: Period,

  /// Extensions for gender
  _gender: Element,

  /// Extensions for language
  _language: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Identifier for a person within a particular scope.
  identifier: Vec<Identifier>,

  /// The patient this person is related to.
  patient: Box<Reference>,

  /// The nature of the relationship between a patient and the related person.
  relationship: Vec<CodeableConcept>,

  /// The date on which the related person was born.
  #[serde(rename = "birthDate")]
  birth_date: i32,

  /// Whether this related person record is in active use.
  active: bool,

  /// A contact detail for the person, e.g. a telephone number or an email address.
  telecom: Vec<ContactPoint>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Extensions for birthDate
  #[serde(rename = "_birthDate")]
  _birth_date: Element,

  /// Image of the person.
  photo: Vec<Attachment>,

  /// Administrative Gender - the gender that the person is considered to have for
  /// administration and record keeping purposes.
  gender: RelatedPersonGender,

  /// The base language in which the resource is written.
  language: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum RelatedPersonGender {
  #[serde(rename = "male")]
  Male,

  #[serde(rename = "female")]
  Female,

  #[serde(rename = "other")]
  Other,

  #[serde(rename = "unknown")]
  Unknown,

}