#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;
use crate::model::Coding::Coding;
use crate::model::AuditEvent_Detail::AuditEvent_Detail;


/// A record of an event made for purposes of maintaining a security log. Typical
/// uses include detection of intrusion attempts and monitoring for inappropriate
/// usage.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEvent_Entity {
  /// A name of the entity in the audit event.
  name: String,

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

  /// Identifies a specific instance of the entity. The reference should be version
  /// specific.
  what: Box<Reference>,

  /// Extensions for description
  _description: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The query parameters for a query-type entities.
  query: String,

  /// Extensions for query
  _query: Element,

  /// Tagged value pairs for conveying additional information about the entity.
  detail: Vec<AuditEvent_Detail>,

  /// Code representing the role the entity played in the event being audited.
  role: Coding,

  /// Text that describes the entity in more detail.
  description: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The type of the object that was involved in this audit event.
  #[serde(rename = "type")]
  fhir_type: Coding,

  /// Identifier for the data life-cycle stage for the entity.
  lifecycle: Coding,

  /// Extensions for name
  _name: Element,

  /// Security labels for the identified entity.
  #[serde(rename = "securityLabel")]
  security_label: Vec<Coding>,

}