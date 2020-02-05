use serde::{Deserialize, Serialize};

/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DataRequirement_DateFilter {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The value of the filter. If period is specified, the filter will return only
  /// those data items that fall within the bounds determined by the Period, inclusive
  /// of the period boundaries. If dateTime is specified, the filter will return only
  /// those data items that are equal to the specified dateTime. If a Duration is
  /// specified, the filter will return only those data items that fall within
  /// Duration before now.
  #[serde(rename = "valuePeriod")]
  value_period: Period,

  /// The value of the filter. If period is specified, the filter will return only
  /// those data items that fall within the bounds determined by the Period, inclusive
  /// of the period boundaries. If dateTime is specified, the filter will return only
  /// those data items that are equal to the specified dateTime. If a Duration is
  /// specified, the filter will return only those data items that fall within
  /// Duration before now.
  #[serde(rename = "valueDuration")]
  value_duration: Duration,

  /// Extensions for searchParam
  #[serde(rename = "_searchParam")]
  _search_param: Element,

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
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The date-valued attribute of the filter. The specified path SHALL be a FHIRPath
  /// resolveable on the specified type of the DataRequirement, and SHALL consist only
  /// of identifiers, constant indexers, and .resolve(). The path is allowed to
  /// contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to
  /// traverse multiple-cardinality sub-elements (see the [Simple FHIRPath
  /// Profile](fhirpath.html#simple) for full details). Note that the index must be an
  /// integer constant. The path must resolve to an element of type date, dateTime,
  /// Period, Schedule, or Timing.
  path: String,

  /// The value of the filter. If period is specified, the filter will return only
  /// those data items that fall within the bounds determined by the Period, inclusive
  /// of the period boundaries. If dateTime is specified, the filter will return only
  /// those data items that are equal to the specified dateTime. If a Duration is
  /// specified, the filter will return only those data items that fall within
  /// Duration before now.
  #[serde(rename = "valueDateTime")]
  value_date_time: String,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Element,

  /// Extensions for path
  _path: Element,

  /// A date parameter that refers to a search parameter defined on the specified type
  /// of the DataRequirement, and which searches on elements of type date, dateTime,
  /// Period, Schedule, or Timing.
  #[serde(rename = "searchParam")]
  search_param: String,

}