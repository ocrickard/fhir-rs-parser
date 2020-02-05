use serde::{Deserialize, Serialize};

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical artifacts such as clinical decision
/// support rules, order sets and protocols.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlanDefinition_RelatedAction {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// A duration or range of durations to apply to the relationship. For example, 30-
  /// 60 minutes before.
  #[serde(rename = "offsetRange")]
  offset_range: Range,

  /// Extensions for actionId
  #[serde(rename = "_actionId")]
  _action_id: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

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

  /// The element id of the related action.
  #[serde(rename = "actionId")]
  action_id: id,

  /// The relationship of this action to the related action.
  relationship: PlanDefinition_RelatedActionRelationship,

  /// A duration or range of durations to apply to the relationship. For example, 30-
  /// 60 minutes before.
  #[serde(rename = "offsetDuration")]
  offset_duration: Duration,

  /// Extensions for relationship
  _relationship: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum PlanDefinition_RelatedActionRelationship {
  #[serde(rename = "before-start")]
  BeforeStart,

  #[serde(rename = "before")]
  Before,

  #[serde(rename = "before-end")]
  BeforeEnd,

  #[serde(rename = "concurrent-with-start")]
  ConcurrentWithStart,

  #[serde(rename = "concurrent")]
  Concurrent,

  #[serde(rename = "concurrent-with-end")]
  ConcurrentWithEnd,

  #[serde(rename = "after-start")]
  AfterStart,

  #[serde(rename = "after")]
  After,

  #[serde(rename = "after-end")]
  AfterEnd,

}