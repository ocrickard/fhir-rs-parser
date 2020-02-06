#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Timing_Repeat::Timing_Repeat;
use crate::model::Extension::Extension;


/// Specifies an event that may occur multiple times. Timing schedules are used to
/// record when things are planned, expected or requested to occur. The most common
/// usage is in dosage instructions for medications. They are also used when
/// planning care of various kinds, and may be used for reporting the schedule to
/// which past regular activities were carried out.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
  /// Identifies specific times when the event occurs.
  event: Option<Vec<String>>,

  /// Extensions for event
  #[serde(rename = "_event")]
  _event: Option<Vec<Element>>,

  /// A set of rules that describe when the event is scheduled.
  repeat: Option<Timing_Repeat>,

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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// A code for the timing schedule (or just text in code.text). Some codes such as
  /// BID are ubiquitous, but many institutions define their own additional codes. If
  /// a code is provided, the code is understood to be a complete statement of
  /// whatever is specified in the structured timing data, and either the code or the
  /// data may be used to interpret the Timing, with the exception that .repeat.bounds
  /// still applies over the code (and is not contained in the code).
  code: Option<CodeableConcept>,

}
