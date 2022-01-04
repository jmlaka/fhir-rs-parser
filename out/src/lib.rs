pub mod model;
pub mod parser;

#[cfg(test)]
mod tests {
  use crate::model::*;
  use std::fs;
  use std::time::Instant;

  #[test]
  fn test_parsing_json_samples() {
    let paths = fs::read_dir("examples-json/").unwrap();

    println!(
      "sizeof element definition:{:?}",
      std::mem::size_of::<ElementDefinition::ElementDefinition>()
    );

    for path in paths {
      let unwrapped_path = path.unwrap().path();
      println!("Beginning {}", &unwrapped_path.to_str().unwrap());
      let schema_contents =
        fs::read_to_string(&unwrapped_path).expect("Something went wrong reading the file");
      let string_parse_begin = Instant::now();
      let resource = crate::parser::fhir_parse(&schema_contents).unwrap();
      println!(
        "Parsed resource {}: {}us",
        &unwrapped_path.to_str().unwrap(),
        string_parse_begin.elapsed().as_micros()
      );
      let resource_validate_begin = Instant::now();
      assert!(resource.validate());
      println!(
        "Successfully validated resource {}: {}us",
        &unwrapped_path.to_str().unwrap(),
        resource_validate_begin.elapsed().as_micros()
      );
    }
  }

  #[test]
  fn test_generate_json() {
    let mut vision_builder = VisionPrescription::VisionPrescriptionBuilder::new(
      vec![
        VisionPrescription_LensSpecification::VisionPrescription_LensSpecificationBuilder::new(
          CodeableConcept::CodeableConceptBuilder::new().build(),
        )
        .build(),
      ],
      Reference::ReferenceBuilder::new()
        .identifier(
          Identifier::IdentifierBuilder::new()
            .id("id")
            .value("value")
            .build(),
        )
        .build(),
      Reference::ReferenceBuilder::new().build(),
    );
    assert_eq!(
      vision_builder.build().to_json().to_string(),
      r#"{"lensSpecification":[{"product":{}}],"patient":{"identifier":{"id":"id","value":"value"}},"prescriber":{},"resourceType":"VisionPrescription"}"#
    );
  }

  #[test]
  fn test_mutate_json() {
    let schema_contents = fs::read_to_string("examples-json/visionprescription-example.json")
      .expect("Something went wrong reading the file");
    let resource = crate::parser::fhir_parse(&schema_contents).unwrap();
    if let Some(ResourceList::ResourceListEnum::ResourceVisionPrescription(vision_prescription)) =
      resource.resource()
    {
      let mut builder = VisionPrescription::VisionPrescriptionBuilder::with(vision_prescription);
      builder.language("Pirate");
      assert_eq!(
        builder.build().to_json().to_string(),
        r#"{"created":"2014-06-15","dateWritten":"2014-06-15","id":"33123","identifier":[{"system":"http://www.happysight.com/prescription","value":"15013"}],"language":"Pirate","lensSpecification":[{"add":2.0,"eye":"right","prism":[{"amount":0.5,"base":"down"}],"product":{"coding":[{"code":"lens","system":"http://terminology.hl7.org/CodeSystem/ex-visionprescriptionproduct"}]},"sphere":-2.0},{"add":2.0,"axis":180,"cylinder":-0.5,"eye":"left","prism":[{"amount":0.5,"base":"up"}],"product":{"coding":[{"code":"lens","system":"http://terminology.hl7.org/CodeSystem/ex-visionprescriptionproduct"}]},"sphere":-1.0}],"meta":{"tag":[{"code":"HTEST","display":"test health data","system":"http://terminology.hl7.org/CodeSystem/v3-ActReason"}]},"patient":{"reference":"Patient/example"},"prescriber":{"reference":"Practitioner/example"},"resourceType":"VisionPrescription","status":"active","text":{"div":"<div xmlns=\"http://www.w3.org/1999/xhtml\">\n\t\t\t<p>OD -2.00 SPH         +2.00 add    0.5 p.d. BD</p>\n\t\t\t<p>OS -1.00 -0.50 x 180 +2.00 add    0.5 p.d. BU</p>\n\t\t</div>","status":"generated"}}"#
      );
    } else {
      assert!(false, "Didn't get a vision prescription");
    }
  }
}
