#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMap_Structure {
  /// How the referenced structure is used in this mapping.
  mode: Option<StructureMap_StructureMode>,

  /// Extensions for alias
  #[serde(rename = "_alias")]
  _alias: Option<Element>,

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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The name used for this type in the map.
  alias: Option<String>,

  /// The canonical reference to the structure.
  url: String,

  /// Documentation that describes how the structure is used in the mapping.
  documentation: Option<String>,

  /// Extensions for documentation
  #[serde(rename = "_documentation")]
  _documentation: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for mode
  #[serde(rename = "_mode")]
  _mode: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum StructureMap_StructureMode {
  #[serde(rename = "source")]
  Source,

  #[serde(rename = "queried")]
  Queried,

  #[serde(rename = "target")]
  Target,

  #[serde(rename = "produced")]
  Produced,

}
