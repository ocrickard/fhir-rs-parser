#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinition_DeviceName {
  /// The type of deviceName.
  /// UDILabelName | UserFriendlyName | PatientReportedName | ManufactureDeviceName |
  /// ModelName.
  #[serde(rename = "type")]
  fhir_type: Option<DeviceDefinition_DeviceNameType>,

  /// The name of the device.
  name: Option<String>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

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

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceDefinition_DeviceNameType {
  #[serde(rename = "udi-label-name")]
  UdiLabelName,

  #[serde(rename = "user-friendly-name")]
  UserFriendlyName,

  #[serde(rename = "patient-reported-name")]
  PatientReportedName,

  #[serde(rename = "manufacturer-name")]
  ManufacturerName,

  #[serde(rename = "model-name")]
  ModelName,

  #[serde(rename = "other")]
  Other,

}
