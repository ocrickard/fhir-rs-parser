#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Measure_Group::Measure_Group;
use crate::model::Measure_SupplementalData::Measure_SupplementalData;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::ResourceList::ResourceList;
use crate::model::UsageContext::UsageContext;
use serde_json::value::Value;

/// The Measure resource provides the definition of a quality measure.

#[derive(Debug)]
pub struct Measure<'a> {
    pub value: &'a Value,
}

impl Measure<'_> {
    /// Extensions for definition
    pub fn _definition(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_definition") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for disclaimer
    pub fn _disclaimer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_disclaimer") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Provides a summary of relevant clinical guidelines or other clinical
    /// recommendations supporting the measure.
    pub fn clinical_recommendation_statement(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("clinicalRecommendationStatement") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for usage
    pub fn _usage(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_usage") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The date on which the resource content was last reviewed. Review happens
    /// periodically after approval but does not change the original approval date.
    pub fn last_review_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lastReviewDate") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for riskAdjustment
    pub fn _risk_adjustment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_riskAdjustment") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A description of the risk adjustment factors that may impact the resulting score
    /// for the measure and how they may be accounted for when computing and reporting
    /// measure results.
    pub fn risk_adjustment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("riskAdjustment") {
            return Some(string);
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate measure instances.
    pub fn use_context(&self) -> Option<Vec<UsageContext>> {
        if let Some(Value::Array(val)) = self.value.get("useContext") {
            return Some(
                val.into_iter()
                    .map(|e| UsageContext { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for purpose
    pub fn _purpose(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_purpose") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A Boolean value to indicate that this measure is authored for testing purposes
    /// (or education/evaluation/marketing) and is not intended to be used for genuine
    /// usage.
    pub fn experimental(&self) -> Option<bool> {
        if let Some(val) = self.value.get("experimental") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// If this is a composite measure, the scoring method used to combine the component
    /// measures to determine the composite score.
    pub fn composite_scoring(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("compositeScoring") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for rateAggregation
    pub fn _rate_aggregation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_rateAggregation") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A copyright statement relating to the measure and/or its contents. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// measure.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// A reference to a Library resource containing the formal logic used by the
    /// measure.
    pub fn library(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("library") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for copyright
    pub fn _copyright(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_copyright") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// An individual or organization primarily responsible for internal coherence of
    /// the content.
    pub fn editor(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("editor") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for experimental
    pub fn _experimental(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_experimental") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Explanation of why this measure is needed and why it has been designed as it
    /// has.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
        }
        return None;
    }

    /// An individual or organization responsible for officially endorsing the content
    /// for use in some setting.
    pub fn endorser(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("endorser") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Notices and disclaimers regarding the use of the measure or related to
    /// intellectual property (such as code systems) referenced by the measure.
    pub fn disclaimer(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("disclaimer") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for clinicalRecommendationStatement
    pub fn _clinical_recommendation_statement(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_clinicalRecommendationStatement") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Information on whether an increase or decrease in score is the preferred result
    /// (e.g., a higher score indicates better quality OR a lower score indicates better
    /// quality OR quality is within a range).
    pub fn improvement_notation(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("improvementNotation") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub fn approval_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("approvalDate") {
            return Some(string);
        }
        return None;
    }

    /// The period during which the measure content was or is planned to be in active
    /// use.
    pub fn effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectivePeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// The supplemental data criteria for the measure report, specified as either the
    /// name of a valid CQL expression within a referenced library, or a valid FHIR
    /// Resource Path.
    pub fn supplemental_data(&self) -> Option<Vec<Measure_SupplementalData>> {
        if let Some(Value::Array(val)) = self.value.get("supplementalData") {
            return Some(
                val.into_iter()
                    .map(|e| Measure_SupplementalData { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The date  (and optionally time) when the measure was published. The date must
    /// change when the business version changes and it must change if the status code
    /// changes. In addition, it should change when the substantive content of the
    /// measure changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for rationale
    pub fn _rationale(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_rationale") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for publisher
    pub fn _publisher(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publisher") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A natural language name identifying the measure. This name should be usable as
    /// an identifier for the module by machine processing applications such as code
    /// generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The intended subjects for the measure. If this element is not provided, a
    /// Patient subject is assumed, but the subject of the measure can be anything.
    pub fn subject_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subjectReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

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
    pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Indicates whether the measure is used to examine a process, an outcome over
    /// time, a patient-reported outcome, or a structure measure such as utilization.
    pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("type") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for subtitle
    pub fn _subtitle(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subtitle") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A legal or geographic region in which the measure is intended to be used.
    pub fn jurisdiction(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("jurisdiction") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Indicates how the calculation is performed for the measure, including
    /// proportion, ratio, continuous-variable, and cohort. The value set is extensible,
    /// allowing additional measure scoring types to be represented.
    pub fn scoring(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("scoring") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, and nor can they
    /// have their own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Related artifacts such as additional documentation, justification, or
    /// bibliographic references.
    pub fn related_artifact(&self) -> Option<Vec<RelatedArtifact>> {
        if let Some(Value::Array(val)) = self.value.get("relatedArtifact") {
            return Some(
                val.into_iter()
                    .map(|e| RelatedArtifact { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Describes how to combine the information calculated, based on logic in each of
    /// several populations, into one summarized result.
    pub fn rate_aggregation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("rateAggregation") {
            return Some(string);
        }
        return None;
    }

    /// Additional guidance for the measure including how it can be used in a clinical
    /// context, and the intent of the measure.
    pub fn guidance(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("guidance") {
            return Some(string);
        }
        return None;
    }

    /// A group of population criteria for the measure.
    pub fn group(&self) -> Option<Vec<Measure_Group>> {
        if let Some(Value::Array(val)) = self.value.get("group") {
            return Some(
                val.into_iter()
                    .map(|e| Measure_Group { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Descriptive topics related to the content of the measure. Topics provide a high-
    /// level categorization grouping types of measures that can be useful for
    /// filtering and searching.
    pub fn topic(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("topic") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for guidance
    pub fn _guidance(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_guidance") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The identifier that is used to identify this version of the measure when it is
    /// referenced in a specification, model, design or instance. This is an arbitrary
    /// value managed by the measure author and is not expected to be globally unique.
    /// For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not
    /// available. There is also no expectation that versions can be placed in a
    /// lexicographical sequence. To provide a version consistent with the Decision
    /// Support Service specification, use the format Major.Minor.Revision (e.g. 1.0.0).
    /// For more information on versioning knowledge assets, refer to the Decision
    /// Support Service specification. Note that a version is required for non-
    /// experimental active artifacts.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    /// An explanatory or alternate title for the measure giving additional information
    /// about its content.
    pub fn subtitle(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("subtitle") {
            return Some(string);
        }
        return None;
    }

    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative { value: val });
        }
        return None;
    }

    /// Provides a succinct statement of the need for the measure. Usually includes
    /// statements pertaining to importance criterion: impact, gap in care, and
    /// evidence.
    pub fn rationale(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("rationale") {
            return Some(string);
        }
        return None;
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// A free text natural language description of the measure from a consumer's
    /// perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// A short, descriptive, user-friendly title for the measure.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// A detailed description, from a clinical perspective, of how the measure is used.
    pub fn usage(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("usage") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// An individiual or organization primarily involved in the creation and
    /// maintenance of the content.
    pub fn author(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("author") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A formal identifier that is used to identify this measure when it is represented
    /// in other formats, or referenced in a specification, model, design or an
    /// instance.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The status of this measure. Enables tracking the life-cycle of the content.
    pub fn status(&self) -> Option<MeasureStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(MeasureStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for lastReviewDate
    pub fn _last_review_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastReviewDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Provides a description of an individual term used within the measure.
    pub fn definition(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("definition") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for approvalDate
    pub fn _approval_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_approvalDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// An individual or organization primarily responsible for review of some aspect of
    /// the content.
    pub fn reviewer(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("reviewer") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An absolute URI that is used to identify this measure when it is referenced in a
    /// specification, model, design or an instance; also called its canonical
    /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
    /// which at which an authoritative instance of this measure is (or will be)
    /// published. This URL can be the target of a canonical reference. It SHALL remain
    /// the same when the measure is stored on different servers.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub fn contact(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The name of the organization or individual that published the measure.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// The intended subjects for the measure. If this element is not provided, a
    /// Patient subject is assumed, but the subject of the measure can be anything.
    pub fn subject_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subjectCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._definition() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._disclaimer() {
            _val.validate();
        }
        if let Some(_val) = self.clinical_recommendation_statement() {}
        if let Some(_val) = self._usage() {
            _val.validate();
        }
        if let Some(_val) = self.last_review_date() {}
        if let Some(_val) = self._risk_adjustment() {
            _val.validate();
        }
        if let Some(_val) = self.risk_adjustment() {}
        if let Some(_val) = self.use_context() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self._purpose() {
            _val.validate();
        }
        if let Some(_val) = self._version() {
            _val.validate();
        }
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self.composite_scoring() {
            _val.validate();
        }
        if let Some(_val) = self._rate_aggregation() {
            _val.validate();
        }
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self.library() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._copyright() {
            _val.validate();
        }
        if let Some(_val) = self.editor() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self._date() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self._experimental() {
            _val.validate();
        }
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self.endorser() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.disclaimer() {}
        if let Some(_val) = self._clinical_recommendation_statement() {
            _val.validate();
        }
        if let Some(_val) = self.improvement_notation() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.approval_date() {}
        if let Some(_val) = self.effective_period() {
            _val.validate();
        }
        if let Some(_val) = self.supplemental_data() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._rationale() {
            _val.validate();
        }
        if let Some(_val) = self._publisher() {
            _val.validate();
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.subject_reference() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._subtitle() {
            _val.validate();
        }
        if let Some(_val) = self.jurisdiction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._url() {
            _val.validate();
        }
        if let Some(_val) = self.scoring() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.related_artifact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.rate_aggregation() {}
        if let Some(_val) = self.guidance() {}
        if let Some(_val) = self.group() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.topic() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._guidance() {
            _val.validate();
        }
        if let Some(_val) = self.version() {}
        if let Some(_val) = self.subtitle() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.rationale() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.usage() {}
        if let Some(_val) = self._title() {
            _val.validate();
        }
        if let Some(_val) = self.author() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self._last_review_date() {
            _val.validate();
        }
        if let Some(_val) = self.definition() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self._approval_date() {
            _val.validate();
        }
        if let Some(_val) = self.reviewer() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.contact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.subject_codeable_concept() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum MeasureStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl MeasureStatus {
    pub fn from_string(string: &str) -> Option<MeasureStatus> {
        match string {
            "draft" => Some(MeasureStatus::Draft),
            "active" => Some(MeasureStatus::Active),
            "retired" => Some(MeasureStatus::Retired),
            "unknown" => Some(MeasureStatus::Unknown),
            _ => None,
        }
    }
}
