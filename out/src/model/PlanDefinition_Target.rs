#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Duration::Duration;


/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical artifacts such as clinical decision
/// support rules, order sets and protocols.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinition_Target {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The target value of the measure to be achieved to signify fulfillment of the
  /// goal, e.g. 150 pounds or 7.0%. Either the high or low or both values of the
  /// range can be specified. When a low value is missing, it indicates that the goal
  /// is achieved at any value at or below the high value. Similarly, if the high
  /// value is missing, it indicates that the goal is achieved at any value at or
  /// above the low value.
  #[serde(rename = "detailQuantity")]
  detail_quantity: Option<Quantity>,

  /// The target value of the measure to be achieved to signify fulfillment of the
  /// goal, e.g. 150 pounds or 7.0%. Either the high or low or both values of the
  /// range can be specified. When a low value is missing, it indicates that the goal
  /// is achieved at any value at or below the high value. Similarly, if the high
  /// value is missing, it indicates that the goal is achieved at any value at or
  /// above the low value.
  #[serde(rename = "detailRange")]
  detail_range: Option<Range>,

  /// Indicates the timeframe after the start of the goal in which the goal should be
  /// met.
  due: Option<Duration>,

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

  /// The target value of the measure to be achieved to signify fulfillment of the
  /// goal, e.g. 150 pounds or 7.0%. Either the high or low or both values of the
  /// range can be specified. When a low value is missing, it indicates that the goal
  /// is achieved at any value at or below the high value. Similarly, if the high
  /// value is missing, it indicates that the goal is achieved at any value at or
  /// above the low value.
  #[serde(rename = "detailCodeableConcept")]
  detail_codeable_concept: Option<CodeableConcept>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The parameter whose value is to be tracked, e.g. body weight, blood pressure, or
  /// hemoglobin A1c level.
  measure: Option<CodeableConcept>,

}
