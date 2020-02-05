use serde::{Deserialize, Serialize};

/// Identifies two or more records (resource instances) that refer to the same real-
/// world "occurrence".
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Linkage_Item {
  /// Extensions for type
  _type: Element,

  /// The resource instance being linked as part of the group.
  resource: Reference,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Distinguishes which item is "source of truth" (if any) and which items are no
  /// longer considered to be current representations.
  type: Linkage_ItemType,

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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

}

#[derive(Debug, Serialize, Deserialize)]
enum Linkage_ItemType {
  #[serde(rename = "source")]
  Source,

  #[serde(rename = "alternate")]
  Alternate,

  #[serde(rename = "historical")]
  Historical,

}