#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Address::Address;
use crate::model::Narrative::Narrative;
use crate::model::Element::Element;
use crate::model::Person_Link::Person_Link;
use crate::model::ContactPoint::ContactPoint;
use crate::model::HumanName::HumanName;
use crate::model::Extension::Extension;
use crate::model::ResourceList::ResourceList;
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;
use crate::model::Attachment::Attachment;
use crate::model::Meta::Meta;


/// Demographics and administrative information about a person independent of a
/// specific health-related context.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
  /// Extensions for gender
  _gender: Element,

  /// Extensions for birthDate
  #[serde(rename = "_birthDate")]
  _birth_date: Element,

  /// One or more addresses for the person.
  address: Vec<Address>,

  /// The organization that is the custodian of the person record.
  #[serde(rename = "managingOrganization")]
  managing_organization: Box<Reference>,

  /// A name associated with the person.
  name: Vec<HumanName>,

  /// An image that can be displayed as a thumbnail of the person to enhance the
  /// identification of the individual.
  photo: Attachment,

  /// Identifier for a person within a particular scope.
  identifier: Vec<Identifier>,

  /// Whether this person's record is in active use.
  active: bool,

  /// Link to a resource that concerns the same actual person.
  link: Vec<Person_Link>,

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

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for language
  _language: Element,

  /// A contact detail for the person, e.g. a telephone number or an email address.
  telecom: Vec<ContactPoint>,

  /// Extensions for active
  _active: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The birth date for the person.
  #[serde(rename = "birthDate")]
  birth_date: i32,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// The base language in which the resource is written.
  language: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Administrative Gender.
  gender: PersonGender,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum PersonGender {
  #[serde(rename = "male")]
  Male,

  #[serde(rename = "female")]
  Female,

  #[serde(rename = "other")]
  Other,

  #[serde(rename = "unknown")]
  Unknown,

}