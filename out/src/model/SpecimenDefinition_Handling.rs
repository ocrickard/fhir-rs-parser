#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Range::Range;
use crate::model::Extension::Extension;
use crate::model::Duration::Duration;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;


/// A kind of specimen with associated set of requirements.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinition_Handling {
  /// The temperature interval for this set of handling instructions.
  #[serde(rename = "temperatureRange")]
  temperature_range: Option<Range>,

  /// Additional textual instructions for the preservation or transport of the
  /// specimen. For instance, 'Protect from light exposure'.
  instruction: Option<String>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for instruction
  #[serde(rename = "_instruction")]
  _instruction: Option<Element>,

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

  /// It qualifies the interval of temperature, which characterizes an occurrence of
  /// handling. Conditions that are not related to temperature may be handled in the
  /// instruction element.
  #[serde(rename = "temperatureQualifier")]
  temperature_qualifier: Option<CodeableConcept>,

  /// The maximum time interval of preservation of the specimen with these conditions.
  #[serde(rename = "maxDuration")]
  max_duration: Option<Duration>,

}
