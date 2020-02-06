#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;


/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract_ContentDefinition {
  /// amended | appended | cancelled | disputed | entered-in-error | executable |
  /// executed | negotiable | offered | policy | rejected | renewed | revoked |
  /// resolved | terminated.
  #[serde(rename = "publicationStatus")]
  publication_status: String,

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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for publicationStatus
  #[serde(rename = "_publicationStatus")]
  _publication_status: Element,

  /// Precusory content structure and use, i.e., a boilerplate, template, application
  /// for a contract such as an insurance policy or benefits under a program, e.g.,
  /// workers compensation.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// The date (and optionally time) when the contract was published. The date must
  /// change when the business version changes and it must change if the status code
  /// changes. In addition, it should change when the substantive content of the
  /// contract changes.
  #[serde(rename = "publicationDate")]
  publication_date: String,

  /// Detailed Precusory content type.
  #[serde(rename = "subType")]
  sub_type: CodeableConcept,

  /// A copyright statement relating to Contract precursor content. Copyright
  /// statements are generally legal restrictions on the use and publishing of the
  /// Contract precursor content.
  copyright: String,

  /// The  individual or organization that published the Contract precursor content.
  publisher: Box<Reference>,

  /// Extensions for publicationDate
  #[serde(rename = "_publicationDate")]
  _publication_date: Element,

  /// Extensions for copyright
  _copyright: Element,

}