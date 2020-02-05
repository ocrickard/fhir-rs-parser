use serde::{Deserialize, Serialize};

/// Details and position information for a physical place where services are
/// provided and resources and participants may be stored, found, contained, or
/// accommodated.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Location {
  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The status property covers the general availability of the resource, not the
  /// current value which may be covered by the operationStatus, or by a
  /// schedule/slots if they are configured for the location.
  status: LocationStatus,

  /// The operational status covers operation values most relevant to beds (but can
  /// also apply to rooms/units/chairs/etc. such as an isolation unit/dialysis chair).
  /// This typically covers concepts such as contamination, housekeeping, and other
  /// activities like maintenance.
  #[serde(rename = "operationalStatus")]
  operational_status: Coding,

  /// The contact details of communication devices available at the location. This can
  /// include phone numbers, fax numbers, mobile numbers, email addresses and web
  /// sites.
  telecom: Vec<ContactPoint>,

  /// Name of the location as used by humans. Does not need to be unique.
  name: String,

  /// Extensions for language
  _language: Element,

  /// Indicates the type of function performed at the location.
  type: Vec<CodeableConcept>,

  /// Extensions for mode
  _mode: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// A description of when the locations opening ours are different to normal, e.g.
  /// public holiday availability. Succinctly describing all possible exceptions to
  /// normal site availability as detailed in the opening hours Times.
  #[serde(rename = "availabilityExceptions")]
  availability_exceptions: String,

  /// Physical location.
  address: Address,

  /// Technical endpoints providing access to services operated for the location.
  endpoint: Vec<Reference>,

  /// The absolute geographic location of the Location, expressed using the WGS84
  /// datum (This is the same co-ordinate system used in KML).
  position: Location_Position,

  /// A list of alternate names that the location is known as, or was known as, in the
  /// past.
  alias: Vec<String>,

  /// Description of the Location, which helps in finding or referencing the place.
  description: String,

  /// Unique code or number identifying the location to its users.
  identifier: Vec<Identifier>,

  /// Extensions for status
  _status: Element,

  /// Extensions for name
  _name: Element,

  /// Extensions for description
  _description: Element,

  /// Indicates whether a resource instance represents a specific location or a class
  /// of locations.
  mode: LocationMode,

  /// Extensions for availabilityExceptions
  #[serde(rename = "_availabilityExceptions")]
  _availability_exceptions: Element,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Another Location of which this Location is physically a part of.
  #[serde(rename = "partOf")]
  part_of: Reference,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for alias
  _alias: Vec<Element>,

  /// Physical form of the location, e.g. building, room, vehicle, road.
  #[serde(rename = "physicalType")]
  physical_type: CodeableConcept,

  /// The organization responsible for the provisioning and upkeep of the location.
  #[serde(rename = "managingOrganization")]
  managing_organization: Reference,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// What days/times during a week is this location usually open.
  #[serde(rename = "hoursOfOperation")]
  hours_of_operation: Vec<Location_HoursOfOperation>,

}

#[derive(Debug, Serialize, Deserialize)]
enum LocationStatus {
  #[serde(rename = "active")]
  Active,

  #[serde(rename = "suspended")]
  Suspended,

  #[serde(rename = "inactive")]
  Inactive,

}

#[derive(Debug, Serialize, Deserialize)]
enum LocationMode {
  #[serde(rename = "instance")]
  Instance,

  #[serde(rename = "kind")]
  Kind,

}