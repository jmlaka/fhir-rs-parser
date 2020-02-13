#![allow(unused_imports, non_camel_case_types)]

use crate::model::Practitioner::Practitioner;
use crate::model::ResearchSubject::ResearchSubject;
use crate::model::ChargeItemDefinition::ChargeItemDefinition;
use crate::model::SubstanceProtein::SubstanceProtein;
use crate::model::Media::Media;
use crate::model::RelatedPerson::RelatedPerson;
use crate::model::DetectedIssue::DetectedIssue;
use crate::model::BiologicallyDerivedProduct::BiologicallyDerivedProduct;
use crate::model::EvidenceVariable::EvidenceVariable;
use crate::model::MedicinalProduct::MedicinalProduct;
use crate::model::NutritionOrder::NutritionOrder;
use crate::model::SearchParameter::SearchParameter;
use crate::model::SubstanceSpecification::SubstanceSpecification;
use crate::model::Task::Task;
use crate::model::DiagnosticReport::DiagnosticReport;
use crate::model::BodyStructure::BodyStructure;
use crate::model::CoverageEligibilityResponse::CoverageEligibilityResponse;
use crate::model::RiskAssessment::RiskAssessment;
use crate::model::MedicinalProductIngredient::MedicinalProductIngredient;
use crate::model::PaymentReconciliation::PaymentReconciliation;
use crate::model::CodeSystem::CodeSystem;
use crate::model::Patient::Patient;
use crate::model::ResearchStudy::ResearchStudy;
use crate::model::Contract::Contract;
use crate::model::Device::Device;
use crate::model::DocumentManifest::DocumentManifest;
use crate::model::DeviceMetric::DeviceMetric;
use crate::model::CatalogEntry::CatalogEntry;
use crate::model::ExplanationOfBenefit::ExplanationOfBenefit;
use crate::model::Coverage::Coverage;
use crate::model::SubstanceSourceMaterial::SubstanceSourceMaterial;
use crate::model::MedicinalProductUndesirableEffect::MedicinalProductUndesirableEffect;
use crate::model::OperationOutcome::OperationOutcome;
use crate::model::Group::Group;
use crate::model::TestScript::TestScript;
use crate::model::Appointment::Appointment;
use crate::model::SpecimenDefinition::SpecimenDefinition;
use crate::model::VisionPrescription::VisionPrescription;
use crate::model::CapabilityStatement::CapabilityStatement;
use crate::model::Encounter::Encounter;
use crate::model::StructureMap::StructureMap;
use crate::model::ImagingStudy::ImagingStudy;
use crate::model::EffectEvidenceSynthesis::EffectEvidenceSynthesis;
use crate::model::GraphDefinition::GraphDefinition;
use crate::model::ImmunizationRecommendation::ImmunizationRecommendation;
use crate::model::MedicationAdministration::MedicationAdministration;
use crate::model::PlanDefinition::PlanDefinition;
use crate::model::PractitionerRole::PractitionerRole;
use crate::model::ServiceRequest::ServiceRequest;
use crate::model::Location::Location;
use crate::model::Consent::Consent;
use crate::model::Evidence::Evidence;
use crate::model::MedicinalProductAuthorization::MedicinalProductAuthorization;
use crate::model::CarePlan::CarePlan;
use crate::model::SupplyDelivery::SupplyDelivery;
use crate::model::MedicationKnowledge::MedicationKnowledge;
use crate::model::Organization::Organization;
use crate::model::Person::Person;
use crate::model::Specimen::Specimen;
use crate::model::EventDefinition::EventDefinition;
use crate::model::ConceptMap::ConceptMap;
use crate::model::MedicationStatement::MedicationStatement;
use crate::model::ValueSet::ValueSet;
use crate::model::OrganizationAffiliation::OrganizationAffiliation;
use crate::model::SubstancePolymer::SubstancePolymer;
use crate::model::Condition::Condition;
use crate::model::Schedule::Schedule;
use crate::model::SubstanceNucleicAcid::SubstanceNucleicAcid;
use crate::model::Basic::Basic;
use crate::model::Medication::Medication;
use crate::model::MedicinalProductContraindication::MedicinalProductContraindication;
use crate::model::MedicinalProductInteraction::MedicinalProductInteraction;
use crate::model::Parameters::Parameters;
use crate::model::NamingSystem::NamingSystem;
use crate::model::QuestionnaireResponse::QuestionnaireResponse;
use crate::model::CoverageEligibilityRequest::CoverageEligibilityRequest;
use crate::model::ObservationDefinition::ObservationDefinition;
use crate::model::Account::Account;
use crate::model::PaymentNotice::PaymentNotice;
use crate::model::DeviceDefinition::DeviceDefinition;
use crate::model::Questionnaire::Questionnaire;
use crate::model::ActivityDefinition::ActivityDefinition;
use crate::model::MedicationDispense::MedicationDispense;
use crate::model::MedicinalProductPackaged::MedicinalProductPackaged;
use crate::model::AdverseEvent::AdverseEvent;
use crate::model::ResearchElementDefinition::ResearchElementDefinition;
use crate::model::EnrollmentResponse::EnrollmentResponse;
use crate::model::AuditEvent::AuditEvent;
use crate::model::OperationDefinition::OperationDefinition;
use crate::model::StructureDefinition::StructureDefinition;
use crate::model::TerminologyCapabilities::TerminologyCapabilities;
use crate::model::MedicinalProductIndication::MedicinalProductIndication;
use crate::model::Bundle::Bundle;
use crate::model::ImmunizationEvaluation::ImmunizationEvaluation;
use crate::model::Flag::Flag;
use crate::model::Endpoint::Endpoint;
use crate::model::ChargeItem::ChargeItem;
use crate::model::ImplementationGuide::ImplementationGuide;
use crate::model::MolecularSequence::MolecularSequence;
use crate::model::MessageHeader::MessageHeader;
use crate::model::ResearchDefinition::ResearchDefinition;
use crate::model::HealthcareService::HealthcareService;
use crate::model::Goal::Goal;
use crate::model::CareTeam::CareTeam;
use crate::model::CommunicationRequest::CommunicationRequest;
use crate::model::RequestGroup::RequestGroup;
use crate::model::VerificationResult::VerificationResult;
use crate::model::Observation::Observation;
use crate::model::MedicinalProductManufactured::MedicinalProductManufactured;
use crate::model::ClaimResponse::ClaimResponse;
use crate::model::Slot::Slot;
use crate::model::DeviceUseStatement::DeviceUseStatement;
use crate::model::DocumentReference::DocumentReference;
use crate::model::FamilyMemberHistory::FamilyMemberHistory;
use crate::model::Linkage::Linkage;
use crate::model::InsurancePlan::InsurancePlan;
use crate::model::ExampleScenario::ExampleScenario;
use crate::model::CompartmentDefinition::CompartmentDefinition;
use crate::model::GuidanceResponse::GuidanceResponse;
use crate::model::Communication::Communication;
use crate::model::Immunization::Immunization;
use crate::model::AllergyIntolerance::AllergyIntolerance;
use crate::model::RiskEvidenceSynthesis::RiskEvidenceSynthesis;
use crate::model::ClinicalImpression::ClinicalImpression;
use crate::model::Procedure::Procedure;
use crate::model::Claim::Claim;
use crate::model::MedicinalProductPharmaceutical::MedicinalProductPharmaceutical;
use crate::model::SupplyRequest::SupplyRequest;
use crate::model::TestReport::TestReport;
use crate::model::EpisodeOfCare::EpisodeOfCare;
use crate::model::Binary::Binary;
use crate::model::EnrollmentRequest::EnrollmentRequest;
use crate::model::Invoice::Invoice;
use crate::model::Composition::Composition;
use crate::model::List::List;
use crate::model::MeasureReport::MeasureReport;
use crate::model::Measure::Measure;
use crate::model::Provenance::Provenance;
use crate::model::SubstanceReferenceInformation::SubstanceReferenceInformation;
use crate::model::MedicationRequest::MedicationRequest;
use crate::model::Subscription::Subscription;
use crate::model::Substance::Substance;
use crate::model::Library::Library;
use crate::model::MessageDefinition::MessageDefinition;
use crate::model::AppointmentResponse::AppointmentResponse;
use crate::model::DeviceRequest::DeviceRequest;
use serde_json::value::Value;


#[derive(Debug)]
pub struct ResourceList<'a> {
  pub value: &'a Value,
}

#[derive(Debug)]
pub enum ResourceListEnum<'a> {
  ResourceAccount(Account<'a>),
  ResourceActivityDefinition(ActivityDefinition<'a>),
  ResourceAdverseEvent(AdverseEvent<'a>),
  ResourceAllergyIntolerance(AllergyIntolerance<'a>),
  ResourceAppointment(Appointment<'a>),
  ResourceAppointmentResponse(AppointmentResponse<'a>),
  ResourceAuditEvent(AuditEvent<'a>),
  ResourceBasic(Basic<'a>),
  ResourceBinary(Binary<'a>),
  ResourceBiologicallyDerivedProduct(BiologicallyDerivedProduct<'a>),
  ResourceBodyStructure(BodyStructure<'a>),
  ResourceBundle(Bundle<'a>),
  ResourceCapabilityStatement(CapabilityStatement<'a>),
  ResourceCarePlan(CarePlan<'a>),
  ResourceCareTeam(CareTeam<'a>),
  ResourceCatalogEntry(CatalogEntry<'a>),
  ResourceChargeItem(ChargeItem<'a>),
  ResourceChargeItemDefinition(ChargeItemDefinition<'a>),
  ResourceClaim(Claim<'a>),
  ResourceClaimResponse(ClaimResponse<'a>),
  ResourceClinicalImpression(ClinicalImpression<'a>),
  ResourceCodeSystem(CodeSystem<'a>),
  ResourceCommunication(Communication<'a>),
  ResourceCommunicationRequest(CommunicationRequest<'a>),
  ResourceCompartmentDefinition(CompartmentDefinition<'a>),
  ResourceComposition(Composition<'a>),
  ResourceConceptMap(ConceptMap<'a>),
  ResourceCondition(Condition<'a>),
  ResourceConsent(Consent<'a>),
  ResourceContract(Contract<'a>),
  ResourceCoverage(Coverage<'a>),
  ResourceCoverageEligibilityRequest(CoverageEligibilityRequest<'a>),
  ResourceCoverageEligibilityResponse(CoverageEligibilityResponse<'a>),
  ResourceDetectedIssue(DetectedIssue<'a>),
  ResourceDevice(Device<'a>),
  ResourceDeviceDefinition(DeviceDefinition<'a>),
  ResourceDeviceMetric(DeviceMetric<'a>),
  ResourceDeviceRequest(DeviceRequest<'a>),
  ResourceDeviceUseStatement(DeviceUseStatement<'a>),
  ResourceDiagnosticReport(DiagnosticReport<'a>),
  ResourceDocumentManifest(DocumentManifest<'a>),
  ResourceDocumentReference(DocumentReference<'a>),
  ResourceEffectEvidenceSynthesis(EffectEvidenceSynthesis<'a>),
  ResourceEncounter(Encounter<'a>),
  ResourceEndpoint(Endpoint<'a>),
  ResourceEnrollmentRequest(EnrollmentRequest<'a>),
  ResourceEnrollmentResponse(EnrollmentResponse<'a>),
  ResourceEpisodeOfCare(EpisodeOfCare<'a>),
  ResourceEventDefinition(EventDefinition<'a>),
  ResourceEvidence(Evidence<'a>),
  ResourceEvidenceVariable(EvidenceVariable<'a>),
  ResourceExampleScenario(ExampleScenario<'a>),
  ResourceExplanationOfBenefit(ExplanationOfBenefit<'a>),
  ResourceFamilyMemberHistory(FamilyMemberHistory<'a>),
  ResourceFlag(Flag<'a>),
  ResourceGoal(Goal<'a>),
  ResourceGraphDefinition(GraphDefinition<'a>),
  ResourceGroup(Group<'a>),
  ResourceGuidanceResponse(GuidanceResponse<'a>),
  ResourceHealthcareService(HealthcareService<'a>),
  ResourceImagingStudy(ImagingStudy<'a>),
  ResourceImmunization(Immunization<'a>),
  ResourceImmunizationEvaluation(ImmunizationEvaluation<'a>),
  ResourceImmunizationRecommendation(ImmunizationRecommendation<'a>),
  ResourceImplementationGuide(ImplementationGuide<'a>),
  ResourceInsurancePlan(InsurancePlan<'a>),
  ResourceInvoice(Invoice<'a>),
  ResourceLibrary(Library<'a>),
  ResourceLinkage(Linkage<'a>),
  ResourceList(List<'a>),
  ResourceLocation(Location<'a>),
  ResourceMeasure(Measure<'a>),
  ResourceMeasureReport(MeasureReport<'a>),
  ResourceMedia(Media<'a>),
  ResourceMedication(Medication<'a>),
  ResourceMedicationAdministration(MedicationAdministration<'a>),
  ResourceMedicationDispense(MedicationDispense<'a>),
  ResourceMedicationKnowledge(MedicationKnowledge<'a>),
  ResourceMedicationRequest(MedicationRequest<'a>),
  ResourceMedicationStatement(MedicationStatement<'a>),
  ResourceMedicinalProduct(MedicinalProduct<'a>),
  ResourceMedicinalProductAuthorization(MedicinalProductAuthorization<'a>),
  ResourceMedicinalProductContraindication(MedicinalProductContraindication<'a>),
  ResourceMedicinalProductIndication(MedicinalProductIndication<'a>),
  ResourceMedicinalProductIngredient(MedicinalProductIngredient<'a>),
  ResourceMedicinalProductInteraction(MedicinalProductInteraction<'a>),
  ResourceMedicinalProductManufactured(MedicinalProductManufactured<'a>),
  ResourceMedicinalProductPackaged(MedicinalProductPackaged<'a>),
  ResourceMedicinalProductPharmaceutical(MedicinalProductPharmaceutical<'a>),
  ResourceMedicinalProductUndesirableEffect(MedicinalProductUndesirableEffect<'a>),
  ResourceMessageDefinition(MessageDefinition<'a>),
  ResourceMessageHeader(MessageHeader<'a>),
  ResourceMolecularSequence(MolecularSequence<'a>),
  ResourceNamingSystem(NamingSystem<'a>),
  ResourceNutritionOrder(NutritionOrder<'a>),
  ResourceObservation(Observation<'a>),
  ResourceObservationDefinition(ObservationDefinition<'a>),
  ResourceOperationDefinition(OperationDefinition<'a>),
  ResourceOperationOutcome(OperationOutcome<'a>),
  ResourceOrganization(Organization<'a>),
  ResourceOrganizationAffiliation(OrganizationAffiliation<'a>),
  ResourceParameters(Parameters<'a>),
  ResourcePatient(Patient<'a>),
  ResourcePaymentNotice(PaymentNotice<'a>),
  ResourcePaymentReconciliation(PaymentReconciliation<'a>),
  ResourcePerson(Person<'a>),
  ResourcePlanDefinition(PlanDefinition<'a>),
  ResourcePractitioner(Practitioner<'a>),
  ResourcePractitionerRole(PractitionerRole<'a>),
  ResourceProcedure(Procedure<'a>),
  ResourceProvenance(Provenance<'a>),
  ResourceQuestionnaire(Questionnaire<'a>),
  ResourceQuestionnaireResponse(QuestionnaireResponse<'a>),
  ResourceRelatedPerson(RelatedPerson<'a>),
  ResourceRequestGroup(RequestGroup<'a>),
  ResourceResearchDefinition(ResearchDefinition<'a>),
  ResourceResearchElementDefinition(ResearchElementDefinition<'a>),
  ResourceResearchStudy(ResearchStudy<'a>),
  ResourceResearchSubject(ResearchSubject<'a>),
  ResourceRiskAssessment(RiskAssessment<'a>),
  ResourceRiskEvidenceSynthesis(RiskEvidenceSynthesis<'a>),
  ResourceSchedule(Schedule<'a>),
  ResourceSearchParameter(SearchParameter<'a>),
  ResourceServiceRequest(ServiceRequest<'a>),
  ResourceSlot(Slot<'a>),
  ResourceSpecimen(Specimen<'a>),
  ResourceSpecimenDefinition(SpecimenDefinition<'a>),
  ResourceStructureDefinition(StructureDefinition<'a>),
  ResourceStructureMap(StructureMap<'a>),
  ResourceSubscription(Subscription<'a>),
  ResourceSubstance(Substance<'a>),
  ResourceSubstanceNucleicAcid(SubstanceNucleicAcid<'a>),
  ResourceSubstancePolymer(SubstancePolymer<'a>),
  ResourceSubstanceProtein(SubstanceProtein<'a>),
  ResourceSubstanceReferenceInformation(SubstanceReferenceInformation<'a>),
  ResourceSubstanceSourceMaterial(SubstanceSourceMaterial<'a>),
  ResourceSubstanceSpecification(SubstanceSpecification<'a>),
  ResourceSupplyDelivery(SupplyDelivery<'a>),
  ResourceSupplyRequest(SupplyRequest<'a>),
  ResourceTask(Task<'a>),
  ResourceTerminologyCapabilities(TerminologyCapabilities<'a>),
  ResourceTestReport(TestReport<'a>),
  ResourceTestScript(TestScript<'a>),
  ResourceValueSet(ValueSet<'a>),
  ResourceVerificationResult(VerificationResult<'a>),
  ResourceVisionPrescription(VisionPrescription<'a>),
}

impl ResourceList<'_> {
  pub fn resource(&self) -> Option<ResourceListEnum> {
    let fhir_type = self.value["resourceType"].as_str().unwrap();
    match fhir_type {
      "Account" => Some(ResourceListEnum::ResourceAccount(Account { value: self.value })),
      "ActivityDefinition" => Some(ResourceListEnum::ResourceActivityDefinition(ActivityDefinition { value: self.value })),
      "AdverseEvent" => Some(ResourceListEnum::ResourceAdverseEvent(AdverseEvent { value: self.value })),
      "AllergyIntolerance" => Some(ResourceListEnum::ResourceAllergyIntolerance(AllergyIntolerance { value: self.value })),
      "Appointment" => Some(ResourceListEnum::ResourceAppointment(Appointment { value: self.value })),
      "AppointmentResponse" => Some(ResourceListEnum::ResourceAppointmentResponse(AppointmentResponse { value: self.value })),
      "AuditEvent" => Some(ResourceListEnum::ResourceAuditEvent(AuditEvent { value: self.value })),
      "Basic" => Some(ResourceListEnum::ResourceBasic(Basic { value: self.value })),
      "Binary" => Some(ResourceListEnum::ResourceBinary(Binary { value: self.value })),
      "BiologicallyDerivedProduct" => Some(ResourceListEnum::ResourceBiologicallyDerivedProduct(BiologicallyDerivedProduct { value: self.value })),
      "BodyStructure" => Some(ResourceListEnum::ResourceBodyStructure(BodyStructure { value: self.value })),
      "Bundle" => Some(ResourceListEnum::ResourceBundle(Bundle { value: self.value })),
      "CapabilityStatement" => Some(ResourceListEnum::ResourceCapabilityStatement(CapabilityStatement { value: self.value })),
      "CarePlan" => Some(ResourceListEnum::ResourceCarePlan(CarePlan { value: self.value })),
      "CareTeam" => Some(ResourceListEnum::ResourceCareTeam(CareTeam { value: self.value })),
      "CatalogEntry" => Some(ResourceListEnum::ResourceCatalogEntry(CatalogEntry { value: self.value })),
      "ChargeItem" => Some(ResourceListEnum::ResourceChargeItem(ChargeItem { value: self.value })),
      "ChargeItemDefinition" => Some(ResourceListEnum::ResourceChargeItemDefinition(ChargeItemDefinition { value: self.value })),
      "Claim" => Some(ResourceListEnum::ResourceClaim(Claim { value: self.value })),
      "ClaimResponse" => Some(ResourceListEnum::ResourceClaimResponse(ClaimResponse { value: self.value })),
      "ClinicalImpression" => Some(ResourceListEnum::ResourceClinicalImpression(ClinicalImpression { value: self.value })),
      "CodeSystem" => Some(ResourceListEnum::ResourceCodeSystem(CodeSystem { value: self.value })),
      "Communication" => Some(ResourceListEnum::ResourceCommunication(Communication { value: self.value })),
      "CommunicationRequest" => Some(ResourceListEnum::ResourceCommunicationRequest(CommunicationRequest { value: self.value })),
      "CompartmentDefinition" => Some(ResourceListEnum::ResourceCompartmentDefinition(CompartmentDefinition { value: self.value })),
      "Composition" => Some(ResourceListEnum::ResourceComposition(Composition { value: self.value })),
      "ConceptMap" => Some(ResourceListEnum::ResourceConceptMap(ConceptMap { value: self.value })),
      "Condition" => Some(ResourceListEnum::ResourceCondition(Condition { value: self.value })),
      "Consent" => Some(ResourceListEnum::ResourceConsent(Consent { value: self.value })),
      "Contract" => Some(ResourceListEnum::ResourceContract(Contract { value: self.value })),
      "Coverage" => Some(ResourceListEnum::ResourceCoverage(Coverage { value: self.value })),
      "CoverageEligibilityRequest" => Some(ResourceListEnum::ResourceCoverageEligibilityRequest(CoverageEligibilityRequest { value: self.value })),
      "CoverageEligibilityResponse" => Some(ResourceListEnum::ResourceCoverageEligibilityResponse(CoverageEligibilityResponse { value: self.value })),
      "DetectedIssue" => Some(ResourceListEnum::ResourceDetectedIssue(DetectedIssue { value: self.value })),
      "Device" => Some(ResourceListEnum::ResourceDevice(Device { value: self.value })),
      "DeviceDefinition" => Some(ResourceListEnum::ResourceDeviceDefinition(DeviceDefinition { value: self.value })),
      "DeviceMetric" => Some(ResourceListEnum::ResourceDeviceMetric(DeviceMetric { value: self.value })),
      "DeviceRequest" => Some(ResourceListEnum::ResourceDeviceRequest(DeviceRequest { value: self.value })),
      "DeviceUseStatement" => Some(ResourceListEnum::ResourceDeviceUseStatement(DeviceUseStatement { value: self.value })),
      "DiagnosticReport" => Some(ResourceListEnum::ResourceDiagnosticReport(DiagnosticReport { value: self.value })),
      "DocumentManifest" => Some(ResourceListEnum::ResourceDocumentManifest(DocumentManifest { value: self.value })),
      "DocumentReference" => Some(ResourceListEnum::ResourceDocumentReference(DocumentReference { value: self.value })),
      "EffectEvidenceSynthesis" => Some(ResourceListEnum::ResourceEffectEvidenceSynthesis(EffectEvidenceSynthesis { value: self.value })),
      "Encounter" => Some(ResourceListEnum::ResourceEncounter(Encounter { value: self.value })),
      "Endpoint" => Some(ResourceListEnum::ResourceEndpoint(Endpoint { value: self.value })),
      "EnrollmentRequest" => Some(ResourceListEnum::ResourceEnrollmentRequest(EnrollmentRequest { value: self.value })),
      "EnrollmentResponse" => Some(ResourceListEnum::ResourceEnrollmentResponse(EnrollmentResponse { value: self.value })),
      "EpisodeOfCare" => Some(ResourceListEnum::ResourceEpisodeOfCare(EpisodeOfCare { value: self.value })),
      "EventDefinition" => Some(ResourceListEnum::ResourceEventDefinition(EventDefinition { value: self.value })),
      "Evidence" => Some(ResourceListEnum::ResourceEvidence(Evidence { value: self.value })),
      "EvidenceVariable" => Some(ResourceListEnum::ResourceEvidenceVariable(EvidenceVariable { value: self.value })),
      "ExampleScenario" => Some(ResourceListEnum::ResourceExampleScenario(ExampleScenario { value: self.value })),
      "ExplanationOfBenefit" => Some(ResourceListEnum::ResourceExplanationOfBenefit(ExplanationOfBenefit { value: self.value })),
      "FamilyMemberHistory" => Some(ResourceListEnum::ResourceFamilyMemberHistory(FamilyMemberHistory { value: self.value })),
      "Flag" => Some(ResourceListEnum::ResourceFlag(Flag { value: self.value })),
      "Goal" => Some(ResourceListEnum::ResourceGoal(Goal { value: self.value })),
      "GraphDefinition" => Some(ResourceListEnum::ResourceGraphDefinition(GraphDefinition { value: self.value })),
      "Group" => Some(ResourceListEnum::ResourceGroup(Group { value: self.value })),
      "GuidanceResponse" => Some(ResourceListEnum::ResourceGuidanceResponse(GuidanceResponse { value: self.value })),
      "HealthcareService" => Some(ResourceListEnum::ResourceHealthcareService(HealthcareService { value: self.value })),
      "ImagingStudy" => Some(ResourceListEnum::ResourceImagingStudy(ImagingStudy { value: self.value })),
      "Immunization" => Some(ResourceListEnum::ResourceImmunization(Immunization { value: self.value })),
      "ImmunizationEvaluation" => Some(ResourceListEnum::ResourceImmunizationEvaluation(ImmunizationEvaluation { value: self.value })),
      "ImmunizationRecommendation" => Some(ResourceListEnum::ResourceImmunizationRecommendation(ImmunizationRecommendation { value: self.value })),
      "ImplementationGuide" => Some(ResourceListEnum::ResourceImplementationGuide(ImplementationGuide { value: self.value })),
      "InsurancePlan" => Some(ResourceListEnum::ResourceInsurancePlan(InsurancePlan { value: self.value })),
      "Invoice" => Some(ResourceListEnum::ResourceInvoice(Invoice { value: self.value })),
      "Library" => Some(ResourceListEnum::ResourceLibrary(Library { value: self.value })),
      "Linkage" => Some(ResourceListEnum::ResourceLinkage(Linkage { value: self.value })),
      "List" => Some(ResourceListEnum::ResourceList(List { value: self.value })),
      "Location" => Some(ResourceListEnum::ResourceLocation(Location { value: self.value })),
      "Measure" => Some(ResourceListEnum::ResourceMeasure(Measure { value: self.value })),
      "MeasureReport" => Some(ResourceListEnum::ResourceMeasureReport(MeasureReport { value: self.value })),
      "Media" => Some(ResourceListEnum::ResourceMedia(Media { value: self.value })),
      "Medication" => Some(ResourceListEnum::ResourceMedication(Medication { value: self.value })),
      "MedicationAdministration" => Some(ResourceListEnum::ResourceMedicationAdministration(MedicationAdministration { value: self.value })),
      "MedicationDispense" => Some(ResourceListEnum::ResourceMedicationDispense(MedicationDispense { value: self.value })),
      "MedicationKnowledge" => Some(ResourceListEnum::ResourceMedicationKnowledge(MedicationKnowledge { value: self.value })),
      "MedicationRequest" => Some(ResourceListEnum::ResourceMedicationRequest(MedicationRequest { value: self.value })),
      "MedicationStatement" => Some(ResourceListEnum::ResourceMedicationStatement(MedicationStatement { value: self.value })),
      "MedicinalProduct" => Some(ResourceListEnum::ResourceMedicinalProduct(MedicinalProduct { value: self.value })),
      "MedicinalProductAuthorization" => Some(ResourceListEnum::ResourceMedicinalProductAuthorization(MedicinalProductAuthorization { value: self.value })),
      "MedicinalProductContraindication" => Some(ResourceListEnum::ResourceMedicinalProductContraindication(MedicinalProductContraindication { value: self.value })),
      "MedicinalProductIndication" => Some(ResourceListEnum::ResourceMedicinalProductIndication(MedicinalProductIndication { value: self.value })),
      "MedicinalProductIngredient" => Some(ResourceListEnum::ResourceMedicinalProductIngredient(MedicinalProductIngredient { value: self.value })),
      "MedicinalProductInteraction" => Some(ResourceListEnum::ResourceMedicinalProductInteraction(MedicinalProductInteraction { value: self.value })),
      "MedicinalProductManufactured" => Some(ResourceListEnum::ResourceMedicinalProductManufactured(MedicinalProductManufactured { value: self.value })),
      "MedicinalProductPackaged" => Some(ResourceListEnum::ResourceMedicinalProductPackaged(MedicinalProductPackaged { value: self.value })),
      "MedicinalProductPharmaceutical" => Some(ResourceListEnum::ResourceMedicinalProductPharmaceutical(MedicinalProductPharmaceutical { value: self.value })),
      "MedicinalProductUndesirableEffect" => Some(ResourceListEnum::ResourceMedicinalProductUndesirableEffect(MedicinalProductUndesirableEffect { value: self.value })),
      "MessageDefinition" => Some(ResourceListEnum::ResourceMessageDefinition(MessageDefinition { value: self.value })),
      "MessageHeader" => Some(ResourceListEnum::ResourceMessageHeader(MessageHeader { value: self.value })),
      "MolecularSequence" => Some(ResourceListEnum::ResourceMolecularSequence(MolecularSequence { value: self.value })),
      "NamingSystem" => Some(ResourceListEnum::ResourceNamingSystem(NamingSystem { value: self.value })),
      "NutritionOrder" => Some(ResourceListEnum::ResourceNutritionOrder(NutritionOrder { value: self.value })),
      "Observation" => Some(ResourceListEnum::ResourceObservation(Observation { value: self.value })),
      "ObservationDefinition" => Some(ResourceListEnum::ResourceObservationDefinition(ObservationDefinition { value: self.value })),
      "OperationDefinition" => Some(ResourceListEnum::ResourceOperationDefinition(OperationDefinition { value: self.value })),
      "OperationOutcome" => Some(ResourceListEnum::ResourceOperationOutcome(OperationOutcome { value: self.value })),
      "Organization" => Some(ResourceListEnum::ResourceOrganization(Organization { value: self.value })),
      "OrganizationAffiliation" => Some(ResourceListEnum::ResourceOrganizationAffiliation(OrganizationAffiliation { value: self.value })),
      "Parameters" => Some(ResourceListEnum::ResourceParameters(Parameters { value: self.value })),
      "Patient" => Some(ResourceListEnum::ResourcePatient(Patient { value: self.value })),
      "PaymentNotice" => Some(ResourceListEnum::ResourcePaymentNotice(PaymentNotice { value: self.value })),
      "PaymentReconciliation" => Some(ResourceListEnum::ResourcePaymentReconciliation(PaymentReconciliation { value: self.value })),
      "Person" => Some(ResourceListEnum::ResourcePerson(Person { value: self.value })),
      "PlanDefinition" => Some(ResourceListEnum::ResourcePlanDefinition(PlanDefinition { value: self.value })),
      "Practitioner" => Some(ResourceListEnum::ResourcePractitioner(Practitioner { value: self.value })),
      "PractitionerRole" => Some(ResourceListEnum::ResourcePractitionerRole(PractitionerRole { value: self.value })),
      "Procedure" => Some(ResourceListEnum::ResourceProcedure(Procedure { value: self.value })),
      "Provenance" => Some(ResourceListEnum::ResourceProvenance(Provenance { value: self.value })),
      "Questionnaire" => Some(ResourceListEnum::ResourceQuestionnaire(Questionnaire { value: self.value })),
      "QuestionnaireResponse" => Some(ResourceListEnum::ResourceQuestionnaireResponse(QuestionnaireResponse { value: self.value })),
      "RelatedPerson" => Some(ResourceListEnum::ResourceRelatedPerson(RelatedPerson { value: self.value })),
      "RequestGroup" => Some(ResourceListEnum::ResourceRequestGroup(RequestGroup { value: self.value })),
      "ResearchDefinition" => Some(ResourceListEnum::ResourceResearchDefinition(ResearchDefinition { value: self.value })),
      "ResearchElementDefinition" => Some(ResourceListEnum::ResourceResearchElementDefinition(ResearchElementDefinition { value: self.value })),
      "ResearchStudy" => Some(ResourceListEnum::ResourceResearchStudy(ResearchStudy { value: self.value })),
      "ResearchSubject" => Some(ResourceListEnum::ResourceResearchSubject(ResearchSubject { value: self.value })),
      "RiskAssessment" => Some(ResourceListEnum::ResourceRiskAssessment(RiskAssessment { value: self.value })),
      "RiskEvidenceSynthesis" => Some(ResourceListEnum::ResourceRiskEvidenceSynthesis(RiskEvidenceSynthesis { value: self.value })),
      "Schedule" => Some(ResourceListEnum::ResourceSchedule(Schedule { value: self.value })),
      "SearchParameter" => Some(ResourceListEnum::ResourceSearchParameter(SearchParameter { value: self.value })),
      "ServiceRequest" => Some(ResourceListEnum::ResourceServiceRequest(ServiceRequest { value: self.value })),
      "Slot" => Some(ResourceListEnum::ResourceSlot(Slot { value: self.value })),
      "Specimen" => Some(ResourceListEnum::ResourceSpecimen(Specimen { value: self.value })),
      "SpecimenDefinition" => Some(ResourceListEnum::ResourceSpecimenDefinition(SpecimenDefinition { value: self.value })),
      "StructureDefinition" => Some(ResourceListEnum::ResourceStructureDefinition(StructureDefinition { value: self.value })),
      "StructureMap" => Some(ResourceListEnum::ResourceStructureMap(StructureMap { value: self.value })),
      "Subscription" => Some(ResourceListEnum::ResourceSubscription(Subscription { value: self.value })),
      "Substance" => Some(ResourceListEnum::ResourceSubstance(Substance { value: self.value })),
      "SubstanceNucleicAcid" => Some(ResourceListEnum::ResourceSubstanceNucleicAcid(SubstanceNucleicAcid { value: self.value })),
      "SubstancePolymer" => Some(ResourceListEnum::ResourceSubstancePolymer(SubstancePolymer { value: self.value })),
      "SubstanceProtein" => Some(ResourceListEnum::ResourceSubstanceProtein(SubstanceProtein { value: self.value })),
      "SubstanceReferenceInformation" => Some(ResourceListEnum::ResourceSubstanceReferenceInformation(SubstanceReferenceInformation { value: self.value })),
      "SubstanceSourceMaterial" => Some(ResourceListEnum::ResourceSubstanceSourceMaterial(SubstanceSourceMaterial { value: self.value })),
      "SubstanceSpecification" => Some(ResourceListEnum::ResourceSubstanceSpecification(SubstanceSpecification { value: self.value })),
      "SupplyDelivery" => Some(ResourceListEnum::ResourceSupplyDelivery(SupplyDelivery { value: self.value })),
      "SupplyRequest" => Some(ResourceListEnum::ResourceSupplyRequest(SupplyRequest { value: self.value })),
      "Task" => Some(ResourceListEnum::ResourceTask(Task { value: self.value })),
      "TerminologyCapabilities" => Some(ResourceListEnum::ResourceTerminologyCapabilities(TerminologyCapabilities { value: self.value })),
      "TestReport" => Some(ResourceListEnum::ResourceTestReport(TestReport { value: self.value })),
      "TestScript" => Some(ResourceListEnum::ResourceTestScript(TestScript { value: self.value })),
      "ValueSet" => Some(ResourceListEnum::ResourceValueSet(ValueSet { value: self.value })),
      "VerificationResult" => Some(ResourceListEnum::ResourceVerificationResult(VerificationResult { value: self.value })),
      "VisionPrescription" => Some(ResourceListEnum::ResourceVisionPrescription(VisionPrescription { value: self.value })),
      _ => None,
    }
  }
}

