#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Quantity::Quantity;
use crate::model::Money::Money;


/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim_SubDetail {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A number to uniquely identify item entries.
  sequence: i32,

  /// Item typification or modifiers codes to convey additional context for the
  /// product or service.
  modifier: Vec<CodeableConcept>,

  /// Unique Device Identifiers associated with this line item.
  udi: Vec<Box<Reference>>,

  /// The number of repetitions of a service or product.
  quantity: Quantity,

  /// Extensions for factor
  _factor: Element,

  /// Code to identify the general type of benefits under which products and services
  /// are provided.
  category: CodeableConcept,

  /// Extensions for sequence
  _sequence: Element,

  /// The type of revenue or cost center providing the product and/or service.
  revenue: CodeableConcept,

  /// If the item is not a group then this is the fee for the product or service,
  /// otherwise this is the total of the fees for the details of the group.
  #[serde(rename = "unitPrice")]
  unit_price: Money,

  /// Identifies the program under which this may be recovered.
  #[serde(rename = "programCode")]
  program_code: Vec<CodeableConcept>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// When the value is a group code then this item collects a set of related claim
  /// details, otherwise this contains the product, service, drug or other billing
  /// code for the item.
  #[serde(rename = "productOrService")]
  product_or_service: CodeableConcept,

  /// A real number that represents a multiplier used in determining the overall value
  /// of services delivered and/or goods received. The concept of a Factor allows for
  /// a discount or surcharge multiplier to be applied to a monetary amount.
  factor: f32,

  /// The quantity times the unit price for an additional service or product or
  /// charge.
  net: Money,

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