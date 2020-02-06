#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Element::Element;


/// Raw data describing a biological sequence.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequence_ReferenceSeq {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// End position of the window on the reference sequence. If the coordinate system
  /// is 0-based then end is exclusive and does not include the last position. If the
  /// coordinate system is 1-base, then end is inclusive and includes the last
  /// position.
  #[serde(rename = "windowEnd")]
  window_end: Option<i32>,

  /// An absolute reference to a strand. The Watson strand is the strand whose 5'-end
  /// is on the short arm of the chromosome, and the Crick strand as the one whose
  /// 5'-end is on the long arm.
  strand: Option<MolecularSequence_ReferenceSeqStrand>,

  /// The Genome Build used for reference, following GRCh build versions e.g. 'GRCh
  /// 37'.  Version number must be included if a versioned release of a primary build
  /// was used.
  #[serde(rename = "genomeBuild")]
  genome_build: Option<String>,

  /// Extensions for windowStart
  #[serde(rename = "_windowStart")]
  _window_start: Option<Element>,

  /// A string like "ACGT".
  #[serde(rename = "referenceSeqString")]
  reference_seq_string: Option<String>,

  /// A relative reference to a DNA strand based on gene orientation. The strand that
  /// contains the open reading frame of the gene is the "sense" strand, and the
  /// opposite complementary strand is the "antisense" strand.
  orientation: Option<MolecularSequence_ReferenceSeqOrientation>,

  /// Extensions for windowEnd
  #[serde(rename = "_windowEnd")]
  _window_end: Option<Element>,

  /// Structural unit composed of a nucleic acid molecule which controls its own
  /// replication through the interaction of specific proteins at one or more origins
  /// of replication
  /// ([SO:0000340](http://www.sequenceontology.org/browser/current_svn/term/SO:000034
  /// 0)).
  chromosome: Option<CodeableConcept>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for genomeBuild
  #[serde(rename = "_genomeBuild")]
  _genome_build: Option<Element>,

  /// Extensions for orientation
  #[serde(rename = "_orientation")]
  _orientation: Option<Element>,

  /// Reference identifier of reference sequence submitted to NCBI. It must match the
  /// type in the MolecularSequence.type field. For example, the prefix, “NG_”
  /// identifies reference sequence for genes, “NM_” for messenger RNA transcripts,
  /// and “NP_” for amino acid sequences.
  #[serde(rename = "referenceSeqId")]
  reference_seq_id: Option<CodeableConcept>,

  /// Extensions for strand
  #[serde(rename = "_strand")]
  _strand: Option<Element>,

  /// Start position of the window on the reference sequence. If the coordinate system
  /// is either 0-based or 1-based, then start position is inclusive.
  #[serde(rename = "windowStart")]
  window_start: Option<i32>,

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

  /// Extensions for referenceSeqString
  #[serde(rename = "_referenceSeqString")]
  _reference_seq_string: Option<Element>,

  /// A pointer to another MolecularSequence entity as reference sequence.
  #[serde(rename = "referenceSeqPointer")]
  reference_seq_pointer: Option<Box<Reference>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum MolecularSequence_ReferenceSeqStrand {
  #[serde(rename = "watson")]
  Watson,

  #[serde(rename = "crick")]
  Crick,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum MolecularSequence_ReferenceSeqOrientation {
  #[serde(rename = "sense")]
  Sense,

  #[serde(rename = "antisense")]
  Antisense,

}
