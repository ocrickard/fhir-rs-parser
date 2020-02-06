#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ClaimResponse_SubDetail1::ClaimResponse_SubDetail1;
use crate::model::Money::Money;
use crate::model::Element::Element;
use crate::model::ClaimResponse_Adjudication::ClaimResponse_Adjudication;
use crate::model::Quantity::Quantity;
use crate::model::Extension::Extension;


/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponse_Detail1 {
  /// Item typification or modifiers codes to convey additional context for the
  /// product or service.
  modifier: Option<Vec<CodeableConcept>>,

  /// The adjudication results.
  adjudication: Vec<ClaimResponse_Adjudication>,

  /// The third-tier service adjudications for payor added services.
  #[serde(rename = "subDetail")]
  sub_detail: Option<Vec<ClaimResponse_SubDetail1>>,

  /// When the value is a group code then this item collects a set of related claim
  /// details, otherwise this contains the product, service, drug or other billing
  /// code for the item.
  #[serde(rename = "productOrService")]
  product_or_service: CodeableConcept,

  /// A real number that represents a multiplier used in determining the overall value
  /// of services delivered and/or goods received. The concept of a Factor allows for
  /// a discount or surcharge multiplier to be applied to a monetary amount.
  factor: Option<f32>,

  /// The number of repetitions of a service or product.
  quantity: Option<Quantity>,

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

  /// If the item is not a group then this is the fee for the product or service,
  /// otherwise this is the total of the fees for the details of the group.
  #[serde(rename = "unitPrice")]
  unit_price: Option<Money>,

  /// The quantity times the unit price for an additional service or product or
  /// charge.
  net: Option<Money>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for factor
  #[serde(rename = "_factor")]
  _factor: Option<Element>,

  /// The numbers associated with notes below which apply to the adjudication of this
  /// item.
  #[serde(rename = "noteNumber")]
  note_number: Option<Vec<i32>>,

  /// Extensions for noteNumber
  #[serde(rename = "_noteNumber")]
  _note_number: Option<Vec<Element>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

}
