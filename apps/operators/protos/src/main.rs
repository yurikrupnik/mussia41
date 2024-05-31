fn main() {
    println!("Hello, world!!");
}

// #[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
// #[kube(group = "kube.rs", version = "v1", kind = "Document", namespaced)]
// pub struct DocumentSpec {
//     title: String,
//     content: String,
// }
//
// fn das() {
//     let docs: Api<Document> = Api::default_namespaced(client);
//     let d = Document::new("guide", DocumentSpec::default());
//     println!("doc: {:?}", d);
//     println!("crd: {:?}", serde_yaml::to_string(&Document::crd()));
// }
