#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ImplementationGuide_Parameter::ImplementationGuide_Parameter;
use crate::model::ImplementationGuide_Template::ImplementationGuide_Template;
use crate::model::ImplementationGuide_Resource::ImplementationGuide_Resource;
use crate::model::ImplementationGuide_Grouping::ImplementationGuide_Grouping;
use crate::model::ImplementationGuide_Page::ImplementationGuide_Page;
use crate::model::Extension::Extension;


/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuide_Definition {
  /// A template for building resources.
  template: Option<Vec<ImplementationGuide_Template>>,

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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// A logical group of resources. Logical groups can be used when building pages.
  grouping: Option<Vec<ImplementationGuide_Grouping>>,

  /// Defines how IG is built by tools.
  parameter: Option<Vec<ImplementationGuide_Parameter>>,

  /// A page / section in the implementation guide. The root page is the
  /// implementation guide home page.
  page: Option<ImplementationGuide_Page>,

  /// A resource that is part of the implementation guide. Conformance resources
  /// (value set, structure definition, capability statements etc.) are obvious
  /// candidates for inclusion, but any kind of resource can be included as an example
  /// resource.
  resource: Vec<ImplementationGuide_Resource>,

}
