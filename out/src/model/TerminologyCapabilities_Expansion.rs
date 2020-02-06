#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::TerminologyCapabilities_Parameter::TerminologyCapabilities_Parameter;


/// A TerminologyCapabilities resource documents a set of capabilities (behaviors)
/// of a FHIR Terminology Server that may be used as a statement of actual server
/// functionality or a statement of required or desired server implementation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilities_Expansion {
  /// Whether the server supports paging on expansion.
  paging: Option<bool>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for paging
  #[serde(rename = "_paging")]
  _paging: Option<Element>,

  /// Documentation about text searching works.
  #[serde(rename = "textFilter")]
  text_filter: Option<String>,

  /// Extensions for textFilter
  #[serde(rename = "_textFilter")]
  _text_filter: Option<Element>,

  /// Extensions for hierarchical
  #[serde(rename = "_hierarchical")]
  _hierarchical: Option<Element>,

  /// Whether the server can return nested value sets.
  hierarchical: Option<bool>,

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

  /// Allow request for incomplete expansions?
  incomplete: Option<bool>,

  /// Extensions for incomplete
  #[serde(rename = "_incomplete")]
  _incomplete: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Supported expansion parameter.
  parameter: Option<Vec<TerminologyCapabilities_Parameter>>,

}
