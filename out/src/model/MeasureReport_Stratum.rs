#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::MeasureReport_Component::MeasureReport_Component;
use crate::model::MeasureReport_Population1::MeasureReport_Population1;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Quantity::Quantity;
use crate::model::Extension::Extension;


/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReport_Stratum {
  /// A stratifier component value.
  component: Vec<MeasureReport_Component>,

  /// The populations that make up the stratum, one for each type of population
  /// appropriate to the measure.
  population: Vec<MeasureReport_Population1>,

  /// The measure score for this stratum, calculated as appropriate for the measure
  /// type and scoring method, and based on only the members of this stratum.
  #[serde(rename = "measureScore")]
  measure_score: Quantity,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The value for this stratum, expressed as a CodeableConcept. When defining
  /// stratifiers on complex values, the value must be rendered such that the value
  /// for each stratum within the stratifier is unique.
  value: CodeableConcept,

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

}