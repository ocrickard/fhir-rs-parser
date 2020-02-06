#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// An amount of economic utility in some recognized currency.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Money {
  /// ISO 4217 Currency Code.
  currency: Option<String>,

  /// Extensions for currency
  #[serde(rename = "_currency")]
  _currency: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for value
  #[serde(rename = "_value")]
  _value: Option<Element>,

  /// Numerical value (with implicit precision).
  value: Option<f32>,

}
