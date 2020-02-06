#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CapabilityStatement_Interaction1::CapabilityStatement_Interaction1;
use crate::model::CapabilityStatement_Resource::CapabilityStatement_Resource;
use crate::model::CapabilityStatement_Operation::CapabilityStatement_Operation;
use crate::model::CapabilityStatement_Security::CapabilityStatement_Security;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CapabilityStatement_SearchParam::CapabilityStatement_SearchParam;


/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatement_Rest {
  /// Definition of an operation or a named query together with its parameters and
  /// their meaning and type.
  operation: Vec<CapabilityStatement_Operation>,

  /// Extensions for mode
  _mode: Element,

  /// A specification of the restful capabilities of the solution for a specific
  /// resource type.
  resource: Vec<CapabilityStatement_Resource>,

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

  /// Information about the system's restful capabilities that apply across all
  /// applications, such as security.
  documentation: String,

  /// Information about security implementation from an interface perspective - what a
  /// client needs to know.
  security: CapabilityStatement_Security,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// An absolute URI which is a reference to the definition of a compartment that the
  /// system supports. The reference is to a CompartmentDefinition resource by its
  /// canonical URL .
  compartment: Vec<String>,

  /// Extensions for documentation
  _documentation: Element,

  /// A specification of restful operations supported by the system.
  interaction: Vec<CapabilityStatement_Interaction1>,

  /// Search parameters that are supported for searching all resources for
  /// implementations to support and/or make use of - either references to ones
  /// defined in the specification, or additional ones defined for/by the
  /// implementation.
  #[serde(rename = "searchParam")]
  search_param: Vec<CapabilityStatement_SearchParam>,

  /// Identifies whether this portion of the statement is describing the ability to
  /// initiate or receive restful operations.
  mode: CapabilityStatement_RestMode,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CapabilityStatement_RestMode {
  #[serde(rename = "client")]
  Client,

  #[serde(rename = "server")]
  Server,

}