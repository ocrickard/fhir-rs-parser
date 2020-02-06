#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Quantity::Quantity;
use crate::model::SpecimenDefinition_Additive::SpecimenDefinition_Additive;


/// A kind of specimen with associated set of requirements.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinition_Container {
  /// Extensions for preparation
  _preparation: Element,

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
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The type of container used to contain this kind of specimen.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// The capacity (volume or other measure) of this kind of container.
  capacity: Quantity,

  /// Color of container cap.
  cap: CodeableConcept,

  /// Extensions for minimumVolumeString
  #[serde(rename = "_minimumVolumeString")]
  _minimum_volume_string: Element,

  /// The type of material of the container.
  material: CodeableConcept,

  /// Substance introduced in the kind of container to preserve, maintain or enhance
  /// the specimen. Examples: Formalin, Citrate, EDTA.
  additive: Vec<SpecimenDefinition_Additive>,

  /// The textual description of the kind of container.
  description: String,

  /// Special processing that should be applied to the container for this kind of
  /// specimen.
  preparation: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for description
  _description: Element,

  /// The minimum volume to be conditioned in the container.
  #[serde(rename = "minimumVolumeString")]
  minimum_volume_string: String,

  /// The minimum volume to be conditioned in the container.
  #[serde(rename = "minimumVolumeQuantity")]
  minimum_volume_quantity: Quantity,

}