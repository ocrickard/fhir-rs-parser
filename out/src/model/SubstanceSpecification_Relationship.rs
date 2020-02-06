#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Ratio::Ratio;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Range::Range;
use crate::model::Element::Element;


/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecification_Relationship {
  /// A pointer to another substance, as a resource or just a representational code.
  #[serde(rename = "substanceCodeableConcept")]
  substance_codeable_concept: Option<CodeableConcept>,

  /// For example where an enzyme strongly bonds with a particular substance, this is
  /// a defining relationship for that enzyme, out of several possible substance
  /// relationships.
  #[serde(rename = "isDefining")]
  is_defining: Option<bool>,

  /// An operator for the amount, for example "average", "approximately", "less than".
  #[serde(rename = "amountType")]
  amount_type: Option<CodeableConcept>,

  /// Extensions for amountString
  #[serde(rename = "_amountString")]
  _amount_string: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

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

  /// A numeric factor for the relationship, for instance to express that the salt of
  /// a substance has some percentage of the active substance in relation to some
  /// other.
  #[serde(rename = "amountQuantity")]
  amount_quantity: Option<Quantity>,

  /// For use when the numeric.
  #[serde(rename = "amountRatioLowLimit")]
  amount_ratio_low_limit: Option<Ratio>,

  /// A numeric factor for the relationship, for instance to express that the salt of
  /// a substance has some percentage of the active substance in relation to some
  /// other.
  #[serde(rename = "amountRatio")]
  amount_ratio: Option<Ratio>,

  /// For example "salt to parent", "active moiety", "starting material".
  relationship: Option<CodeableConcept>,

  /// Extensions for isDefining
  #[serde(rename = "_isDefining")]
  _is_defining: Option<Element>,

  /// A pointer to another substance, as a resource or just a representational code.
  #[serde(rename = "substanceReference")]
  substance_reference: Option<Box<Reference>>,

  /// A numeric factor for the relationship, for instance to express that the salt of
  /// a substance has some percentage of the active substance in relation to some
  /// other.
  #[serde(rename = "amountRange")]
  amount_range: Option<Range>,

  /// A numeric factor for the relationship, for instance to express that the salt of
  /// a substance has some percentage of the active substance in relation to some
  /// other.
  #[serde(rename = "amountString")]
  amount_string: Option<String>,

  /// Supporting literature.
  source: Option<Vec<Box<Reference>>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

}
