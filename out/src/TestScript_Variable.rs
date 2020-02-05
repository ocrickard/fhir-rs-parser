use serde::{Deserialize, Serialize};

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestScript_Variable {
  /// A free text natural language description of the variable and its purpose.
  description: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for name
  _name: Element,

  /// Extensions for headerField
  #[serde(rename = "_headerField")]
  _header_field: Element,

  /// Extensions for defaultValue
  #[serde(rename = "_defaultValue")]
  _default_value: Element,

  /// The FHIRPath expression to evaluate against the fixture body. When variables are
  /// defined, only one of either expression, headerField or path must be specified.
  expression: String,

  /// XPath or JSONPath to evaluate against the fixture body.  When variables are
  /// defined, only one of either expression, headerField or path must be specified.
  path: String,

  /// Extensions for sourceId
  #[serde(rename = "_sourceId")]
  _source_id: Element,

  /// Extensions for hint
  _hint: Element,

  /// A default, hard-coded, or user-defined value for this variable.
  #[serde(rename = "defaultValue")]
  default_value: String,

  /// Will be used to grab the HTTP header field value from the headers that sourceId
  /// is pointing to.
  #[serde(rename = "headerField")]
  header_field: String,

  /// Extensions for path
  _path: Element,

  /// Extensions for description
  _description: Element,

  /// Displayable text string with hint help information to the user when entering a
  /// default value.
  hint: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// Descriptive name for this variable.
  name: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for expression
  _expression: Element,

  /// Fixture to evaluate the XPath/JSONPath expression or the headerField  against
  /// within this variable.
  #[serde(rename = "sourceId")]
  source_id: id,

}