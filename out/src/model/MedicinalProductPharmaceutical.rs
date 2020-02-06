#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::MedicinalProductPharmaceutical_RouteOfAdministration::MedicinalProductPharmaceutical_RouteOfAdministration;
use crate::model::Identifier::Identifier;
use crate::model::Reference::Reference;
use crate::model::MedicinalProductPharmaceutical_Characteristics::MedicinalProductPharmaceutical_Characteristics;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;


/// A pharmaceutical product described in terms of its composition and dose form.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductPharmaceutical {
  /// Characteristics e.g. a products onset of action.
  characteristics: Vec<MedicinalProductPharmaceutical_Characteristics>,

  /// Todo.
  #[serde(rename = "unitOfPresentation")]
  unit_of_presentation: CodeableConcept,

  /// Extensions for language
  _language: Element,

  /// The administrable dose form, after necessary reconstitution.
  #[serde(rename = "administrableDoseForm")]
  administrable_dose_form: CodeableConcept,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// An identifier for the pharmaceutical medicinal product.
  identifier: Vec<Identifier>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// Ingredient.
  ingredient: Vec<Box<Reference>>,

  /// Accompanying device.
  device: Vec<Box<Reference>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The path by which the pharmaceutical product is taken into or makes contact with
  /// the body.
  #[serde(rename = "routeOfAdministration")]
  route_of_administration: Vec<MedicinalProductPharmaceutical_RouteOfAdministration>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

}