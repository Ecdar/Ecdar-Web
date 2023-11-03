use convert_case::*;
use ecdar_protobuf_transpiler::SERVICES as services;
use proc_macro::TokenStream;

trait ToBodyType {
    fn to_body_type(&self) -> String;
}

impl ToBodyType for ecdar_protobuf_transpiler::ProtobuffTypes {
    fn to_body_type(&self) -> String {
        let rust_type = self.to_rust_type();
        match rust_type.as_str() {
            "()" => "".into(),
            _ => format!("body : {rust_type}"),
        }
    }
}

#[proc_macro]
pub fn add_endpoints(app_name: TokenStream) -> TokenStream {
    (app_name.to_string()
        + ecdar_protobuf_transpiler::compile(|var| {
            format!(
                r#".route("/{}/{}", get({}))"#,
                var.service_name, var.endpoint_name, var.fn_name,
            )
        })
        .as_str())
    .print()
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn add_endpoint_functions(_: TokenStream) -> TokenStream {
    ecdar_protobuf_transpiler::compile(|var|
         [
            format!(
                "#[derive(serde::Serialize, serde::Deserialize)]\npub {in_struct}\n",
                in_struct = var.in_struct
            ),
            format!(
                "pub async fn {fn_name}(Json(payload) : Json<{in_struct_name}>) -> Result<Json<{rtn_struct}>, StatusCode> {{",
                fn_name = var.fn_name,
                in_struct_name = var.in_struct_name,
                rtn_struct = var.rtn_struct,
            ),
            format!(
                r#"let mut connect = {module}_client::{client}Client::connect(String::from("http://") + payload.ip.as_str()).await.map_err(|_| StatusCode::MISDIRECTED_REQUEST)?;"#,
                module = var.service_name.to_case(Case::Snake),
                client = var.service_name.to_case(Case::Pascal)
            ),
            format!(
                r#"Ok(Json(connect.{endpoint_fn}({endpoint_fn_in}).await.map_err(|_| StatusCode::BAD_REQUEST)?.into_inner()))"#, 
                endpoint_fn = var.endpoint_name.to_case(Case::Snake),
                endpoint_fn_in = if var.in_struct_has_body { "payload.body" } else { "()" },
            ),
            format!(
                "}}"
            )
        ].join("\n")
    ).print().parse().unwrap()
}

trait Print {
    fn print(self) -> Self;
}

impl Print for String {
    fn print(self) -> Self {
        println!("{self}");
        self
    }
}
