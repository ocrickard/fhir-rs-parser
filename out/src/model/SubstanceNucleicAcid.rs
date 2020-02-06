#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::SubstanceNucleicAcid_Subunit::SubstanceNucleicAcid_Subunit;
use crate::model::Narrative::Narrative;


/// Nucleic acids are defined by three distinct elements: the base, sugar and
/// linkage. Individual substance/moiety IDs will be created for each of these
/// elements. The nucleotide sequence will be always entered in the 5’-3’ direction.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceNucleicAcid {
  /// Extensions for areaOfHybridisation
  #[serde(rename = "_areaOfHybridisation")]
  _area_of_hybridisation: Element,

  /// (TBC).
  #[serde(rename = "oligoNucleotideType")]
  oligo_nucleotide_type: CodeableConcept,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The type of the sequence shall be specified based on a controlled vocabulary.
  #[serde(rename = "sequenceType")]
  sequence_type: CodeableConcept,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Subunits are listed in order of decreasing length; sequences of the same length
  /// will be ordered by molecular weight; subunits that have identical sequences will
  /// be repeated multiple times.
  subunit: Vec<SubstanceNucleicAcid_Subunit>,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// The number of linear sequences of nucleotides linked through phosphodiester
  /// bonds shall be described. Subunits would be strands of nucleic acids that are
  /// tightly associated typically through Watson-Crick base pairing. NOTE: If not
  /// specified in the reference source, the assumption is that there is 1 subunit.
  #[serde(rename = "numberOfSubunits")]
  number_of_subunits: i32,

  /// Extensions for numberOfSubunits
  #[serde(rename = "_numberOfSubunits")]
  _number_of_subunits: Element,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// The area of hybridisation shall be described if applicable for double stranded
  /// RNA or DNA. The number associated with the subunit followed by the number
  /// associated to the residue shall be specified in increasing order. The underscore
  /// “” shall be used as separator as follows: “Subunitnumber Residue”.
  #[serde(rename = "areaOfHybridisation")]
  area_of_hybridisation: String,

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

  /// Extensions for language
  _language: Element,

}