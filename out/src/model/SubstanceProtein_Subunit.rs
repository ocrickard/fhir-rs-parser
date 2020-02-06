#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Attachment::Attachment;
use crate::model::Identifier::Identifier;
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A SubstanceProtein is defined as a single unit of a linear amino acid sequence,
/// or a combination of subunits that are either covalently linked or have a defined
/// invariant stoichiometric relationship. This includes all synthetic, recombinant
/// and purified SubstanceProteins of defined sequence, whether the use is
/// therapeutic or prophylactic. This set of elements will be used to describe
/// albumins, coagulation factors, cytokines, growth factors,
/// peptide/SubstanceProtein hormones, enzymes, toxins, toxoids, recombinant
/// vaccines, and immunomodulators.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceProtein_Subunit {
  /// The modification at the C-terminal shall be specified.
  #[serde(rename = "cTerminalModification")]
  c_terminal_modification: String,

  /// The name of the fragment modified at the N-terminal of the SubstanceProtein
  /// shall be specified.
  #[serde(rename = "nTerminalModification")]
  n_terminal_modification: String,

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

  /// Unique identifier for molecular fragment modification based on the ISO 11238
  /// Substance ID.
  #[serde(rename = "nTerminalModificationId")]
  n_terminal_modification_id: Identifier,

  /// Length of linear sequences of amino acids contained in the subunit.
  length: i32,

  /// The sequence information shall be provided enumerating the amino acids from N-
  /// to C-terminal end using standard single-letter amino acid codes. Uppercase shall
  /// be used for L-amino acids and lowercase for D-amino acids. Transcribed
  /// SubstanceProteins will always be described using the translated sequence; for
  /// synthetic peptide containing amino acids that are not represented with a single
  /// letter code an X should be used within the sequence. The modified amino acids
  /// will be distinguished by their position in the sequence.
  #[serde(rename = "sequenceAttachment")]
  sequence_attachment: Attachment,

  /// Extensions for subunit
  _subunit: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for sequence
  _sequence: Element,

  /// Index of primary sequences of amino acids linked through peptide bonds in order
  /// of decreasing length. Sequences of the same length will be ordered by molecular
  /// weight. Subunits that have identical sequences will be repeated and have
  /// sequential subscripts.
  subunit: i32,

  /// Extensions for length
  _length: Element,

  /// The sequence information shall be provided enumerating the amino acids from N-
  /// to C-terminal end using standard single-letter amino acid codes. Uppercase shall
  /// be used for L-amino acids and lowercase for D-amino acids. Transcribed
  /// SubstanceProteins will always be described using the translated sequence; for
  /// synthetic peptide containing amino acids that are not represented with a single
  /// letter code an X should be used within the sequence. The modified amino acids
  /// will be distinguished by their position in the sequence.
  sequence: String,

  /// Extensions for cTerminalModification
  #[serde(rename = "_cTerminalModification")]
  _c_terminal_modification: Element,

  /// Extensions for nTerminalModification
  #[serde(rename = "_nTerminalModification")]
  _n_terminal_modification: Element,

  /// Unique identifier for molecular fragment modification based on the ISO 11238
  /// Substance ID.
  #[serde(rename = "cTerminalModificationId")]
  c_terminal_modification_id: Identifier,

}