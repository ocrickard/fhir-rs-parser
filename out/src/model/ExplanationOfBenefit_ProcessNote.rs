#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;


/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefit_ProcessNote {
  /// Extensions for text
  #[serde(rename = "_text")]
  _text: Option<Element>,

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
  modifier_extension: Option<Vec<Extension>>,

  /// A number to uniquely identify a note entry.
  number: Option<i32>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// The explanation or description associated with the processing.
  text: Option<String>,

  /// Extensions for number
  #[serde(rename = "_number")]
  _number: Option<Element>,

  /// The business purpose of the note text.
  #[serde(rename = "type")]
  fhir_type: Option<ExplanationOfBenefit_ProcessNoteType>,

  /// A code to define the language used in the text of the note.
  language: Option<CodeableConcept>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExplanationOfBenefit_ProcessNoteType {
  #[serde(rename = "display")]
  Display,

  #[serde(rename = "print")]
  Print,

  #[serde(rename = "printoper")]
  Printoper,

}
