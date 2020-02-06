#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::TestScript_Link::TestScript_Link;
use crate::model::TestScript_Capability::TestScript_Capability;


/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScript_Metadata {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

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

  /// A link to the FHIR specification that this test is covering.
  link: Option<Vec<TestScript_Link>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Capabilities that must exist and are assumed to function correctly on the FHIR
  /// server being tested.
  capability: Vec<TestScript_Capability>,

}
