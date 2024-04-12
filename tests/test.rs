#![allow(clippy::expect_fun_call)]

use indexmap::IndexMap;
use newline_converter::dos2unix;
use openapirs::*;

enum FileType {
    YAML,
    JSON,
}

static TEST_CASES: &[(FileType, &str, &str)] = &[
    (
        FileType::YAML,
        "quayio.yaml",
        include_str!("../fixtures/v30/quayio.yaml"),
    ),
    (
        FileType::JSON,
        "quayio.json",
        include_str!("../fixtures/v30/quayio.json"),
    ),
    (
        FileType::YAML,
        "petstore.yaml",
        include_str!("../fixtures/v30/petstore.yaml"),
    ),
    (
        FileType::YAML,
        "petstore-discriminated.yaml",
        include_str!("../fixtures/v30/petstore-discriminated.yaml"),
    ),
    (
        FileType::YAML,
        "api-with-examples.yaml",
        include_str!("../fixtures/v30/api-with-examples.yaml"),
    ),
    (
        FileType::YAML,
        "link-example.yaml",
        include_str!("../fixtures/v30/link-example.yaml"),
    ),
    (
        FileType::YAML,
        "callback-example.yaml",
        include_str!("../fixtures/v30/callback-example.yaml"),
    ),
    (
        FileType::YAML,
        "docker.yaml",
        include_str!("../fixtures/v30/docker.yaml"),
    ),
    (
        FileType::YAML,
        "forge.yaml",
        include_str!("../fixtures/v30/forge.yaml"),
    ),
    (
        FileType::YAML,
        "adobe_aem.yaml",
        include_str!("../fixtures/v30/adobe_aem.yaml"),
    ),
    (
        FileType::YAML,
        "azure_advisor.yaml",
        include_str!("../fixtures/v30/azure_advisor.yaml"),
    ),
    (
        FileType::JSON,
        "polygon.json",
        include_str!("../fixtures/v30/polygon.json"),
    ),
    (
        FileType::JSON,
        "slack.json",
        include_str!("../fixtures/v30/slack.json"),
    ),
    (
        FileType::JSON,
        "swagger_generator.json",
        include_str!("../fixtures/v30/swagger_generator.json"),
    ),
    (
        FileType::JSON,
        "twilio.json",
        include_str!("../fixtures/v30/twilio.json"),
    ),
    (
        FileType::JSON,
        "fitbit.json",
        include_str!("../fixtures/v30/fitbit.json"),
    ),
    (
        FileType::JSON,
        "walmart.json",
        include_str!("../fixtures/v30/walmart.json"),
    ),
    (
        FileType::JSON,
        "xkcd.json",
        include_str!("../fixtures/v30/xkcd.json"),
    ),
    (
        FileType::YAML,
        "authentiq.yaml",
        include_str!("../fixtures/v30/authentiq.yaml"),
    ),
    (
        FileType::YAML,
        "stripe.yaml",
        include_str!("../fixtures/v30/stripe.yaml"),
    ),
];

#[test]
fn run_tests() {
    for (file_type, name, contents) in TEST_CASES {
        println!("{}", name);
        let openapi: OpenAPI = match file_type {
            FileType::YAML => serde_yaml::from_str(contents)
                .expect(&format!("Could not deserialize file {}", name)),
            FileType::JSON => serde_json::from_str(contents)
                .expect(&format!("Could not deserialize file {}", name)),
        };
        let _yaml =
            serde_yaml::to_string(&openapi).expect(&format!("Could not serialize YAML {}", name));
        let _json =
            serde_json::to_string(&openapi).expect(&format!("Could not serialize JSON {}", name));
    }
}

// macro_rules! map {
//     ( $( $key:expr => $value:expr ),* $(,)? ) => {{
//         #[allow(unused_mut)]
//         let mut m = IndexMap::new();
//         $(m.insert($key, $value);)*
//         m
//     }};
// }

// #[test]
// fn petstore_discriminated() {
//     let api = OpenAPI {
//         openapi: "3.0.0".to_owned(),
//         info: Info {
//             title: "Swagger Petstore".to_owned(),
//             license: Some(License {
//                 name: "MIT".to_owned(),
//                 url: None,
//                 ..Default::default()
//             }),
//             version: "1.0.0".to_owned(),
//             extensions: {
//                 let mut ext = IndexMap::new();
//                 ext.insert("x-hash".to_string(), serde_json::json!("abc123"));
//                 ext
//             },
//             ..Default::default()
//         },
//         servers: vec![Server {
//             url: "http://petstore.swagger.io/v1".to_owned(),
//             ..Default::default()
//         }],
//         components: Some(Components {
//             schemas: map! {
//                 "Cat".to_owned() => ReferenceOr::Item(Schema {
//                     schema_data: SchemaData {
//                         description: Some("A representation of a cat".to_owned()),
//                         ..Default::default()
//                     },
//                     schema_kind: SchemaKind::AllOf { all_of: vec![
//                         ReferenceOr::ref_("#/components/schemas/Pet"),
//                         ReferenceOr::Item(Schema {
//                             schema_data: Default::default(),
//                             schema_kind: SchemaKind::Type(Type::Object(ObjectType {
//                                 properties: map!{
//                                     "huntingSkill".to_owned() => ReferenceOr::boxed_item(Schema {
//                                         schema_data: SchemaData {
//                                             description: Some("The measured skill for hunting".to_owned()),
//                                             ..Default::default()
//                                         },
//                                         schema_kind: SchemaKind::Type(Type::String(StringType {
//                                             enumeration: vec![
//                                                 Some("clueless".to_owned()),
//                                                 Some("lazy".to_owned()),
//                                                 Some("adventurous".to_owned()),
//                                                 Some("aggressive".to_owned()),
//                                             ],
//                                             ..Default::default()
//                                         })),
//                                     }),
//                                 },
//                                 required: vec!["huntingSkill".to_owned()],
//                                 ..Default::default()
//                             })),
//                         }),
//                     ]},
//                 }),

//                 "Dog".to_owned() => ReferenceOr::Item(Schema {
//                     schema_data: SchemaData {
//                         description: Some("A representation of a dog".to_owned()),
//                         ..Default::default()
//                     },
//                     schema_kind: SchemaKind::AllOf { all_of: vec![
//                         ReferenceOr::ref_("#/components/schemas/Pet"),
//                         ReferenceOr::Item(Schema {
//                             schema_data: Default::default(),
//                             schema_kind: SchemaKind::Type(Type::Object(ObjectType {
//                                 properties: map!{
//                                     "packSize".to_owned() => ReferenceOr::boxed_item(Schema {
//                                         schema_data: SchemaData {
//                                             description: Some("the size of the pack the dog is from".to_owned()),
//                                             ..Default::default()
//                                         },
//                                         schema_kind: SchemaKind::Type(Type::Integer(IntegerType {
//                                             format: VariantOrUnknownOrEmpty::Item(IntegerFormat::Int32),
//                                             minimum: Some(0),
//                                             ..Default::default()
//                                         })),
//                                     }),
//                                 },
//                                 required: vec!["packSize".to_owned()],
//                                 ..Default::default()
//                             })),
//                         }),
//                     ]},
//                 }),

//                 "Pet".to_owned() => ReferenceOr::Item(Schema {
//                     schema_data: SchemaData {
//                         discriminator: Some(Discriminator {
//                             property_name: "petType".to_owned(),
//                             ..Default::default()
//                         }),
//                         ..Default::default()
//                     },
//                     schema_kind: SchemaKind::Type(Type::Object(ObjectType {
//                         properties: map!{
//                             "name".to_owned() => ReferenceOr::boxed_item(Schema {
//                                 schema_data: Default::default(),
//                                 schema_kind: SchemaKind::Type(Type::String(Default::default())),
//                             }),
//                             "petType".to_owned() => ReferenceOr::boxed_item(Schema {
//                                 schema_data: Default::default(),
//                                 schema_kind: SchemaKind::Type(Type::String(Default::default())),
//                             }),
//                         },
//                         required: vec!["name".to_owned(), "petType".to_owned()],
//                         ..Default::default()
//                     })),
//                 }),
//             },
//             ..Default::default()
//         }),
//         ..Default::default()
//     };
//     let yaml = include_str!("../fixtures/v30/petstore-discriminated.yaml");
//     assert_eq!(serde_yaml::to_string(&api).unwrap(), dos2unix(yaml));
//     assert_eq!(api, serde_yaml::from_str(yaml).unwrap());
// }

// #[test]
// fn test_operation_extension_docs() {
//     let slack = TEST_CASES.iter().find(|x| x.1 == "slack.json").unwrap();
//     let api: OpenAPI =
//         serde_json::from_str(slack.2).expect(&format!("Could not deserialize file {}", slack.1));
//     let operation_extensions = api
//         .paths
//         .paths
//         .iter()
//         .filter_map(|(_, i)| match i {
//             ReferenceOr::Reference { .. } => None,
//             ReferenceOr::Item(item) => Some(item),
//         })
//         .flat_map(|item| item.iter())
//         .flat_map(|(_, o)| o.extensions.iter().filter(|e| !e.0.starts_with("x-")))
//         .collect::<Vec<_>>();

//     println!("{:#?}", operation_extensions);
// }

// /// Globally defined security may be removed on a per-operation basis
// /// by specifying an empty array for the `security` property. This
// /// test validates this against the `adobe_aem.yaml` specification
// /// which specifies `aemAuth` as the global security, and opting out
// /// on GET `/libs/granite/core/content/login.html` operation.
// #[test]
// fn global_security_removed_with_override() {
//     let openapi: OpenAPI = serde_yaml::from_str(include_str!("../fixtures/v30/adobe_aem.yaml"))
//         .expect("Could not deserialize adobe_aem.yaml");

//     // Global security is set
//     assert!(openapi.security.is_some());

//     // Security is overridden on one path. This path opts out of global security.
//     let path_with_security_override = "/libs/granite/core/content/login.html";
//     if let ReferenceOr::Item(path_item) = openapi
//         .paths
//         .paths
//         .get(path_with_security_override)
//         .unwrap()
//     {
//         assert!(
//             path_item.get.as_ref().unwrap().security.is_some(),
//             "Spec removes global security with empty array."
//         );
//         assert!(
//             path_item
//                 .get
//                 .as_ref()
//                 .unwrap()
//                 .security
//                 .as_ref()
//                 .unwrap()
//                 .is_empty(),
//             "Spec removes global security with empty array."
//         );
//     } else {
//         panic!("Path not found")
//     }

//     // Security is NOT overridden on other paths. Callers must uses global security.
//     let path_no_security_override = "/libs/granite/security/truststore.json";
//     if let ReferenceOr::Item(path_item) =
//         openapi.paths.paths.get(path_no_security_override).unwrap()
//     {
//         assert!(
//             path_item.get.as_ref().unwrap().security.is_none(),
//             "Spec does not specify security on this path."
//         );
//     } else {
//         panic!("Path not found")
//     }
// }
