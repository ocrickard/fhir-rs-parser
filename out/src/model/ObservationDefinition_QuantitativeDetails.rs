#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;


/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationDefinition_QuantitativeDetails {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Number of digits after decimal separator when the results of such observations
  /// are of type Quantity.
  #[serde(rename = "decimalPrecision")]
  decimal_precision: i32,

  /// Factor for converting value expressed with SI unit to value expressed with
  /// customary unit.
  #[serde(rename = "conversionFactor")]
  conversion_factor: f32,

  /// Extensions for decimalPrecision
  #[serde(rename = "_decimalPrecision")]
  _decimal_precision: Element,

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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Customary unit used to report quantitative results of observations conforming to
  /// this ObservationDefinition.
  #[serde(rename = "customaryUnit")]
  customary_unit: CodeableConcept,

  /// SI unit used to report quantitative results of observations conforming to this
  /// ObservationDefinition.
  unit: CodeableConcept,

  /// Extensions for conversionFactor
  #[serde(rename = "_conversionFactor")]
  _conversion_factor: Element,

}