use serde::{Deserialize, Serialize};

/// Raw data describing a biological sequence.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MolecularSequence_Quality {
  /// Extensions for queryFP
  #[serde(rename = "_queryFP")]
  _query_f_p: Element,

  /// End position of the sequence. If the coordinate system is 0-based then end is
  /// exclusive and does not include the last position. If the coordinate system is 1-
  /// base, then end is inclusive and includes the last position.
  end: integer,

  /// Extensions for precision
  _precision: Element,

  /// Extensions for truthTP
  #[serde(rename = "_truthTP")]
  _truth_t_p: Element,

  /// Receiver Operator Characteristic (ROC) Curve  to give sensitivity/specificity
  /// tradeoff.
  roc: MolecularSequence_Roc,

  /// Start position of the sequence. If the coordinate system is either 0-based or 1-
  /// based, then start position is inclusive.
  start: integer,

  /// Extensions for queryTP
  #[serde(rename = "_queryTP")]
  _query_t_p: Element,

  /// INDEL / SNP / Undefined variant.
  type: MolecularSequence_QualityType,

  /// The score of an experimentally derived feature such as a p-value
  /// ([SO:0001685](http://www.sequenceontology.org/browser/current_svn/term/SO:000168
  /// 5)).
  score: Quantity,

  /// Extensions for fScore
  #[serde(rename = "_fScore")]
  _f_score: Element,

  /// Extensions for type
  _type: Element,

  /// Extensions for start
  _start: Element,

  /// True positives, from the perspective of the query data, i.e. the number of sites
  /// in the Query Call Set for which there are paths through the Truth Call Set that
  /// are consistent with all of the alleles at this site, and for which there is an
  /// accurate genotype call for the event.
  #[serde(rename = "queryTP")]
  query_t_p: decimal,

  /// Gold standard sequence used for comparing against.
  #[serde(rename = "standardSequence")]
  standard_sequence: CodeableConcept,

  /// Extensions for end
  _end: Element,

  /// Which method is used to get sequence quality.
  method: CodeableConcept,

  /// Extensions for recall
  _recall: Element,

  /// Harmonic mean of Recall and Precision, computed as: 2 * precision * recall /
  /// (precision + recall).
  #[serde(rename = "fScore")]
  f_score: decimal,

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

  /// The number of false positives where the non-REF alleles in the Truth and Query
  /// Call Sets match (i.e. cases where the truth is 1/1 and the query is 0/1 or
  /// similar).
  #[serde(rename = "gtFP")]
  gt_f_p: decimal,

  /// QUERY.TP / (QUERY.TP + QUERY.FP).
  precision: decimal,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// False negatives, i.e. the number of sites in the Truth Call Set for which there
  /// is no path through the Query Call Set that is consistent with all of the alleles
  /// at this site, or sites for which there is an inaccurate genotype call for the
  /// event. Sites with correct variant but incorrect genotype are counted here.
  #[serde(rename = "truthFN")]
  truth_f_n: decimal,

  /// False positives, i.e. the number of sites in the Query Call Set for which there
  /// is no path through the Truth Call Set that is consistent with this site. Sites
  /// with correct variant but incorrect genotype are counted here.
  #[serde(rename = "queryFP")]
  query_f_p: decimal,

  /// Extensions for truthFN
  #[serde(rename = "_truthFN")]
  _truth_f_n: Element,

  /// True positives, from the perspective of the truth data, i.e. the number of sites
  /// in the Truth Call Set for which there are paths through the Query Call Set that
  /// are consistent with all of the alleles at this site, and for which there is an
  /// accurate genotype call for the event.
  #[serde(rename = "truthTP")]
  truth_t_p: decimal,

  /// TRUTH.TP / (TRUTH.TP + TRUTH.FN).
  recall: decimal,

  /// Extensions for gtFP
  #[serde(rename = "_gtFP")]
  _gt_f_p: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum MolecularSequence_QualityType {
  #[serde(rename = "indel")]
  Indel,

  #[serde(rename = "snp")]
  Snp,

  #[serde(rename = "unknown")]
  Unknown,

}