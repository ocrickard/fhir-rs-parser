#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Range::Range;
use crate::model::Extension::Extension;
use crate::model::Duration::Duration;
use crate::model::Element::Element;


/// A group of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestGroup_RelatedAction {
  /// The relationship of this action to the related action.
  relationship: String,

  /// The element id of the action this is related to.
  #[serde(rename = "actionId")]
  action_id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for actionId
  #[serde(rename = "_actionId")]
  _action_id: Element,

  /// A duration or range of durations to apply to the relationship. For example, 30-
  /// 60 minutes before.
  #[serde(rename = "offsetDuration")]
  offset_duration: Duration,

  /// A duration or range of durations to apply to the relationship. For example, 30-
  /// 60 minutes before.
  #[serde(rename = "offsetRange")]
  offset_range: Range,

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

  /// Extensions for relationship
  _relationship: Element,

}