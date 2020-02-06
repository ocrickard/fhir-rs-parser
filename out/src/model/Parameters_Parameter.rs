#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Address::Address;
use crate::model::Meta::Meta;
use crate::model::Contributor::Contributor;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::Period::Period;
use crate::model::UsageContext::UsageContext;
use crate::model::ContactDetail::ContactDetail;
use crate::model::HumanName::HumanName;
use crate::model::Distance::Distance;
use crate::model::Range::Range;
use crate::model::Attachment::Attachment;
use crate::model::ParameterDefinition::ParameterDefinition;
use crate::model::Duration::Duration;
use crate::model::Extension::Extension;
use crate::model::DataRequirement::DataRequirement;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Timing::Timing;
use crate::model::Annotation::Annotation;
use crate::model::Dosage::Dosage;
use crate::model::ResourceList::ResourceList;
use crate::model::Signature::Signature;
use crate::model::Reference::Reference;
use crate::model::Count::Count;
use crate::model::Money::Money;
use crate::model::SampledData::SampledData;
use crate::model::Coding::Coding;
use crate::model::Quantity::Quantity;
use crate::model::Identifier::Identifier;
use crate::model::Age::Age;
use crate::model::ContactPoint::ContactPoint;
use crate::model::TriggerDefinition::TriggerDefinition;
use crate::model::Ratio::Ratio;
use crate::model::Expression::Expression;


/// This resource is a non-persisted resource used to pass information into and back
/// from an [operation](operations.html). It has no other use, and there is no
/// RESTful endpoint associated with it.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters_Parameter {
  /// Extensions for valueTime
  #[serde(rename = "_valueTime")]
  _value_time: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// If the parameter is a data type.
  #[serde(rename = "valueInteger")]
  value_integer: i32,

  /// If the parameter is a data type.
  #[serde(rename = "valueDate")]
  value_date: String,

  /// If the parameter is a data type.
  #[serde(rename = "valueDosage")]
  value_dosage: Dosage,

  /// If the parameter is a data type.
  #[serde(rename = "valuePeriod")]
  value_period: Period,

  /// If the parameter is a data type.
  #[serde(rename = "valueRange")]
  value_range: Range,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Element,

  /// If the parameter is a data type.
  #[serde(rename = "valueCodeableConcept")]
  value_codeable_concept: CodeableConcept,

  /// If the parameter is a data type.
  #[serde(rename = "valueCount")]
  value_count: Count,

  /// Extensions for valueMarkdown
  #[serde(rename = "_valueMarkdown")]
  _value_markdown: Element,

  /// If the parameter is a data type.
  #[serde(rename = "valueCanonical")]
  value_canonical: String,

  /// If the parameter is a data type.
  #[serde(rename = "valueTriggerDefinition")]
  value_trigger_definition: TriggerDefinition,

  /// If the parameter is a data type.
  #[serde(rename = "valueMeta")]
  value_meta: Meta,

  /// If the parameter is a data type.
  #[serde(rename = "valueDataRequirement")]
  value_data_requirement: DataRequirement,

  /// If the parameter is a data type.
  #[serde(rename = "valueAnnotation")]
  value_annotation: Annotation,

  /// Extensions for valueUnsignedInt
  #[serde(rename = "_valueUnsignedInt")]
  _value_unsigned_int: Element,

  /// If the parameter is a data type.
  #[serde(rename = "valueParameterDefinition")]
  value_parameter_definition: ParameterDefinition,

  /// If the parameter is a data type.
  #[serde(rename = "valueQuantity")]
  value_quantity: Quantity,

  /// If the parameter is a data type.
  #[serde(rename = "valueUrl")]
  value_url: String,

  /// If the parameter is a data type.
  #[serde(rename = "valueDistance")]
  value_distance: Distance,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Element,

  /// If the parameter is a data type.
  #[serde(rename = "valueDecimal")]
  value_decimal: i32,

  /// If the parameter is a data type.
  #[serde(rename = "valueMarkdown")]
  value_markdown: String,

  /// A named part of a multi-part parameter.
  part: Vec<Parameters_Parameter>,

  /// If the parameter is a data type.
  #[serde(rename = "valueMoney")]
  value_money: Money,

  /// If the parameter is a data type.
  #[serde(rename = "valueId")]
  value_id: String,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Element,

  /// If the parameter is a data type.
  #[serde(rename = "valueUri")]
  value_uri: String,

  /// If the parameter is a data type.
  #[serde(rename = "valueIdentifier")]
  value_identifier: Identifier,

  /// If the parameter is a data type.
  #[serde(rename = "valueCoding")]
  value_coding: Coding,

  /// Extensions for valueInstant
  #[serde(rename = "_valueInstant")]
  _value_instant: Element,

  /// Extensions for valueUrl
  #[serde(rename = "_valueUrl")]
  _value_url: Element,

  /// If the parameter is a data type.
  #[serde(rename = "valueExpression")]
  value_expression: Expression,

  /// If the parameter is a data type.
  #[serde(rename = "valueCode")]
  value_code: String,

  /// If the parameter is a data type.
  #[serde(rename = "valueUsageContext")]
  value_usage_context: UsageContext,

  /// If the parameter is a data type.
  #[serde(rename = "valueRatio")]
  value_ratio: Ratio,

  /// If the parameter is a data type.
  #[serde(rename = "valueHumanName")]
  value_human_name: HumanName,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Element,

  /// If the parameter is a data type.
  #[serde(rename = "valueTiming")]
  value_timing: Timing,

  /// Extensions for valueBase64Binary
  #[serde(rename = "_valueBase64Binary")]
  _value_base_6_4_binary: Element,

  /// If the parameter is a data type.
  #[serde(rename = "valueBoolean")]
  value_boolean: bool,

  /// Extensions for valueId
  #[serde(rename = "_valueId")]
  _value_id: Element,

  /// The name of the parameter (reference to the operation definition).
  name: String,

  /// If the parameter is a data type.
  #[serde(rename = "valueInstant")]
  value_instant: String,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Element,

  /// Extensions for valueUuid
  #[serde(rename = "_valueUuid")]
  _value_uuid: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// If the parameter is a data type.
  #[serde(rename = "valueContributor")]
  value_contributor: Contributor,

  /// If the parameter is a data type.
  #[serde(rename = "valueBase64Binary")]
  value_base_6_4_binary: String,

  /// If the parameter is a data type.
  #[serde(rename = "valueDateTime")]
  value_date_time: String,

  /// Extensions for valueCanonical
  #[serde(rename = "_valueCanonical")]
  _value_canonical: Element,

  /// Extensions for valuePositiveInt
  #[serde(rename = "_valuePositiveInt")]
  _value_positive_int: Element,

  /// If the parameter is a data type.
  #[serde(rename = "valueString")]
  value_string: String,

  /// Extensions for valueUri
  #[serde(rename = "_valueUri")]
  _value_uri: Element,

  /// If the parameter is a data type.
  #[serde(rename = "valueTime")]
  value_time: String,

  /// If the parameter is a data type.
  #[serde(rename = "valueContactPoint")]
  value_contact_point: ContactPoint,

  /// If the parameter is a data type.
  #[serde(rename = "valueSampledData")]
  value_sampled_data: SampledData,

  /// If the parameter is a data type.
  #[serde(rename = "valuePositiveInt")]
  value_positive_int: i32,

  /// If the parameter is a data type.
  #[serde(rename = "valueReference")]
  value_reference: Box<Reference>,

  /// If the parameter is a data type.
  #[serde(rename = "valueUnsignedInt")]
  value_unsigned_int: i32,

  /// Extensions for name
  _name: Element,

  /// If the parameter is a data type.
  #[serde(rename = "valueAge")]
  value_age: Age,

  /// If the parameter is a data type.
  #[serde(rename = "valueAttachment")]
  value_attachment: Attachment,

  /// If the parameter is a data type.
  #[serde(rename = "valueContactDetail")]
  value_contact_detail: ContactDetail,

  /// If the parameter is a whole resource.
  resource: ResourceList,

  /// If the parameter is a data type.
  #[serde(rename = "valueOid")]
  value_oid: String,

  /// Extensions for valueCode
  #[serde(rename = "_valueCode")]
  _value_code: Element,

  /// If the parameter is a data type.
  #[serde(rename = "valueUuid")]
  value_uuid: String,

  /// If the parameter is a data type.
  #[serde(rename = "valueSignature")]
  value_signature: Signature,

  /// If the parameter is a data type.
  #[serde(rename = "valueAddress")]
  value_address: Address,

  /// If the parameter is a data type.
  #[serde(rename = "valueRelatedArtifact")]
  value_related_artifact: RelatedArtifact,

  /// If the parameter is a data type.
  #[serde(rename = "valueDuration")]
  value_duration: Duration,

  /// Extensions for valueDate
  #[serde(rename = "_valueDate")]
  _value_date: Element,

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

  /// Extensions for valueOid
  #[serde(rename = "_valueOid")]
  _value_oid: Element,

}