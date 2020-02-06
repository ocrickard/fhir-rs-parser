#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::TestReport_Setup::TestReport_Setup;
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::Identifier::Identifier;
use crate::model::TestReport_Participant::TestReport_Participant;
use crate::model::Extension::Extension;
use crate::model::TestReport_Test::TestReport_Test;
use crate::model::TestReport_Teardown::TestReport_Teardown;


/// A summary of information based on the results of executing a TestScript.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReport {
  /// Extensions for status
  _status: Element,

  /// The current state of this test report.
  status: TestReportStatus,

  /// The overall result from the execution of the TestScript.
  result: TestReportResult,

  /// Extensions for issued
  _issued: Element,

  /// A free text natural language name identifying the executed TestScript.
  name: String,

  /// When the TestScript was executed and this TestReport was generated.
  issued: String,

  /// Extensions for result
  _result: Element,

  /// The base language in which the resource is written.
  language: String,

  /// Identifier for the TestScript assigned for external purposes outside the context
  /// of FHIR.
  identifier: Identifier,

  /// Extensions for language
  _language: Element,

  /// Extensions for score
  _score: Element,

  /// A participant in the test execution, either the execution engine, a client, or a
  /// server.
  participant: Vec<TestReport_Participant>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A test executed from the test script.
  test: Vec<TestReport_Test>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for name
  _name: Element,

  /// The results of the series of required setup operations before the tests were
  /// executed.
  setup: TestReport_Setup,

  /// Name of the tester producing this report (Organization or individual).
  tester: String,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

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

  /// The final score (percentage of tests passed) resulting from the execution of the
  /// TestScript.
  score: f32,

  /// Ideally this is an absolute URL that is used to identify the version-specific
  /// TestScript that was executed, matching the `TestScript.url`.
  #[serde(rename = "testScript")]
  test_script: Box<Reference>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Extensions for tester
  _tester: Element,

  /// The results of the series of operations required to clean up after all the tests
  /// were executed (successfully or otherwise).
  teardown: TestReport_Teardown,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum TestReportStatus {
  #[serde(rename = "completed")]
  Completed,

  #[serde(rename = "in-progress")]
  InProgress,

  #[serde(rename = "waiting")]
  Waiting,

  #[serde(rename = "stopped")]
  Stopped,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum TestReportResult {
  #[serde(rename = "pass")]
  Pass,

  #[serde(rename = "fail")]
  Fail,

  #[serde(rename = "pending")]
  Pending,

}