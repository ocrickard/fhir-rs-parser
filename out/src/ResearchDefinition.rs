use serde::{Deserialize, Serialize};

/// The ResearchDefinition resource describes the conditional state (population and
/// any exposures being compared within the population) and outcome (if specified)
/// that the knowledge (evidence, assertion, recommendation) is about.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResearchDefinition {
  /// Extensions for publisher
  _publisher: Element,

  /// An individual or organization responsible for officially endorsing the content
  /// for use in some setting.
  endorser: Vec<ContactDetail>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

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

  /// The intended subjects for the ResearchDefinition. If this element is not
  /// provided, a Patient subject is assumed, but the subject of the
  /// ResearchDefinition can be anything.
  #[serde(rename = "subjectReference")]
  subject_reference: Reference,

  /// A detailed description, from a clinical perspective, of how the
  /// ResearchDefinition is used.
  usage: String,

  /// A copyright statement relating to the research definition and/or its contents.
  /// Copyright statements are generally legal restrictions on the use and publishing
  /// of the research definition.
  copyright: markdown,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for usage
  _usage: Element,

  /// An individiual or organization primarily involved in the creation and
  /// maintenance of the content.
  author: Vec<ContactDetail>,

  /// The intended subjects for the ResearchDefinition. If this element is not
  /// provided, a Patient subject is assumed, but the subject of the
  /// ResearchDefinition can be anything.
  #[serde(rename = "subjectCodeableConcept")]
  subject_codeable_concept: CodeableConcept,

  /// Extensions for lastReviewDate
  #[serde(rename = "_lastReviewDate")]
  _last_review_date: Element,

  /// Extensions for shortTitle
  #[serde(rename = "_shortTitle")]
  _short_title: Element,

  /// An individual or organization primarily responsible for review of some aspect of
  /// the content.
  reviewer: Vec<ContactDetail>,

  /// Explanation of why this research definition is needed and why it has been
  /// designed as it has.
  purpose: markdown,

  /// The period during which the research definition content was or is planned to be
  /// in active use.
  #[serde(rename = "effectivePeriod")]
  effective_period: Period,

  /// Extensions for description
  _description: Element,

  /// An individual or organization primarily responsible for internal coherence of
  /// the content.
  editor: Vec<ContactDetail>,

  /// Extensions for subtitle
  _subtitle: Element,

  /// Extensions for approvalDate
  #[serde(rename = "_approvalDate")]
  _approval_date: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Extensions for purpose
  _purpose: Element,

  /// A reference to a ResearchElementDefinition resource that defines the population
  /// for the research.
  population: Reference,

  /// A Boolean value to indicate that this research definition is authored for
  /// testing purposes (or education/evaluation/marketing) and is not intended to be
  /// used for genuine usage.
  experimental: bool,

  /// A formal identifier that is used to identify this research definition when it is
  /// represented in other formats, or referenced in a specification, model, design or
  /// an instance.
  identifier: Vec<Identifier>,

  /// A reference to a ResearchElementDefinition resomece that defines the outcome for
  /// the research.
  outcome: Reference,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A natural language name identifying the research definition. This name should be
  /// usable as an identifier for the module by machine processing applications such
  /// as code generation.
  name: String,

  /// Extensions for title
  _title: Element,

  /// The short title provides an alternate title for use in informal descriptive
  /// contexts where the full, formal title is not necessary.
  #[serde(rename = "shortTitle")]
  short_title: String,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate research
  /// definition instances.
  #[serde(rename = "useContext")]
  use_context: Vec<UsageContext>,

  /// A reference to a ResearchElementDefinition resource that defines the exposure
  /// for the research.
  exposure: Reference,

  /// A reference to a ResearchElementDefinition resource that defines the
  /// exposureAlternative for the research.
  #[serde(rename = "exposureAlternative")]
  exposure_alternative: Reference,

  /// A human-readable string to clarify or explain concepts about the resource.
  comment: Vec<String>,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Vec<ContactDetail>,

  /// Extensions for url
  _url: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The name of the organization or individual that published the research
  /// definition.
  publisher: String,

  /// A reference to a Library resource containing the formal logic used by the
  /// ResearchDefinition.
  library: Vec<canonical>,

  /// The date  (and optionally time) when the research definition was published. The
  /// date must change when the business version changes and it must change if the
  /// status code changes. In addition, it should change when the substantive content
  /// of the research definition changes.
  date: dateTime,

  /// Extensions for copyright
  _copyright: Element,

  /// Extensions for comment
  _comment: Vec<Element>,

  /// A free text natural language description of the research definition from a
  /// consumer's perspective.
  description: markdown,

  /// Extensions for version
  _version: Element,

  /// Descriptive topics related to the content of the ResearchDefinition. Topics
  /// provide a high-level categorization grouping types of ResearchDefinitions that
  /// can be useful for filtering and searching.
  topic: Vec<CodeableConcept>,

  /// Extensions for experimental
  _experimental: Element,

  /// A legal or geographic region in which the research definition is intended to be
  /// used.
  jurisdiction: Vec<CodeableConcept>,

  /// The date on which the resource content was last reviewed. Review happens
  /// periodically after approval but does not change the original approval date.
  #[serde(rename = "lastReviewDate")]
  last_review_date: date,

  /// Extensions for date
  _date: Element,

  /// The identifier that is used to identify this version of the research definition
  /// when it is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the research definition author and is not expected to
  /// be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence. To provide a version consistent with
  /// the Decision Support Service specification, use the format Major.Minor.Revision
  /// (e.g. 1.0.0). For more information on versioning knowledge assets, refer to the
  /// Decision Support Service specification. Note that a version is required for non-
  /// experimental active artifacts.
  version: String,

  /// A short, descriptive, user-friendly title for the research definition.
  title: String,

  /// Extensions for status
  _status: Element,

  /// An explanatory or alternate title for the ResearchDefinition giving additional
  /// information about its content.
  subtitle: String,

  /// An absolute URI that is used to identify this research definition when it is
  /// referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this research definition
  /// is (or will be) published. This URL can be the target of a canonical reference.
  /// It SHALL remain the same when the research definition is stored on different
  /// servers.
  url: String,

  /// Related artifacts such as additional documentation, justification, or
  /// bibliographic references.
  #[serde(rename = "relatedArtifact")]
  related_artifact: Vec<RelatedArtifact>,

  /// The date on which the resource content was approved by the publisher. Approval
  /// happens once when the content is officially approved for usage.
  #[serde(rename = "approvalDate")]
  approval_date: date,

  /// Extensions for language
  _language: Element,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for name
  _name: Element,

  /// The status of this research definition. Enables tracking the life-cycle of the
  /// content.
  status: ResearchDefinitionStatus,

}

#[derive(Debug, Serialize, Deserialize)]
enum ResearchDefinitionStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}