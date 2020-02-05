use serde::{Deserialize, Serialize};

/// A set of healthcare-related information that is assembled together into a single
/// logical package that provides a single coherent statement of meaning,
/// establishes its own context and that has clinical attestation with regard to who
/// is making the statement. A Composition defines the structure and narrative
/// content necessary for a document. However, a Composition alone does not
/// constitute a document. Rather, the Composition must be the first entry in a
/// Bundle where Bundle.type=document, and any other resources referenced from
/// Composition must be included as subsequent entries in the Bundle (for example
/// Patient, Practitioner, Encounter, etc.).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Composition_Section {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// A nested sub-section within this section.
  section: Vec<Composition_Section>,

  /// Identifies who is responsible for the information in this section, not
  /// necessarily who typed it in.
  author: Vec<Reference>,

  /// A human-readable narrative that contains the attested content of the section,
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative.
  text: Narrative,

  /// Extensions for mode
  _mode: Element,

  /// If the section is empty, why the list is empty. An empty section typically has
  /// some text explaining the empty reason.
  #[serde(rename = "emptyReason")]
  empty_reason: CodeableConcept,

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

  /// How the entry list was prepared - whether it is a working list that is suitable
  /// for being maintained on an ongoing basis, or if it represents a snapshot of a
  /// list of items from another source, or whether it is a prepared list where items
  /// may be marked as added, modified or deleted.
  mode: String,

  /// A reference to the actual resource from which the narrative in the section is
  /// derived.
  entry: Vec<Reference>,

  /// The actual focus of the section when it is not the subject of the composition,
  /// but instead represents something or someone associated with the subject such as
  /// (for a patient subject) a spouse, parent, fetus, or donor. If not focus is
  /// specified, the focus is assumed to be focus of the parent section, or, for a
  /// section in the Composition itself, the subject of the composition. Sections with
  /// a focus SHALL only include resources where the logical subject (patient,
  /// subject, focus, etc.) matches the section focus, or the resources have no
  /// logical subject (few resources).
  focus: Reference,

  /// Specifies the order applied to the items in the section entries.
  #[serde(rename = "orderedBy")]
  ordered_by: CodeableConcept,

  /// Extensions for title
  _title: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The label for this particular section.  This will be part of the rendered
  /// content for the document, and is often used to build a table of contents.
  title: String,

  /// A code identifying the kind of content contained within the section. This must
  /// be consistent with the section title.
  code: CodeableConcept,

}