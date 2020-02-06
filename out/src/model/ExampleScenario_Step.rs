#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ExampleScenario_Operation::ExampleScenario_Operation;
use crate::model::ExampleScenario_Process::ExampleScenario_Process;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::ExampleScenario_Alternative::ExampleScenario_Alternative;


/// Example of workflow instance.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenario_Step {
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

  /// Nested process.
  process: Option<Vec<ExampleScenario_Process>>,

  /// Extensions for pause
  #[serde(rename = "_pause")]
  _pause: Option<Element>,

  /// If there is a pause in the flow.
  pause: Option<bool>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Indicates an alternative step that can be taken instead of the operations on the
  /// base step in exceptional/atypical circumstances.
  alternative: Option<Vec<ExampleScenario_Alternative>>,

  /// Each interaction or action.
  operation: Option<ExampleScenario_Operation>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

}
