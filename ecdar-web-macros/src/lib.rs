use convert_case::*;
use ecdar_protobuf_transpiler::SERVICES as services;
use proc_macro::TokenStream;

#[proc_macro]
pub fn add_endpoints(app_name: TokenStream) -> TokenStream {
    let mut rtn = app_name.to_string();
    for service in services {
        let name = service.name.to_case(Case::Pascal);
        for endpoint in service.endpoints {
            rtn += format!(
                r#".route("/{name}/{}", get({}))"#,
                endpoint.name,
                get_fn_name(service.name, endpoint.name)
            )
            .as_str();
        }
    }

    rtn.to_string().parse().unwrap()
}

#[proc_macro]
pub fn add_endpoint_functions(_: TokenStream) -> TokenStream {
    let mut rtn = String::new();

    for service in services {
        for endpoint in service.endpoints {
            let fn_name = get_fn_name(service.name, endpoint.name);
            let out_type = fn_name.to_case(Case::Pascal);
            let rtn_type = endpoint.output_type.to_case(Case::Pascal);
            let body = if endpoint.input_type != "()" {
                format!("body : {}", endpoint.input_type.to_case(Case::Pascal))
            } else {
                "".into()
            };

            let module = service.name.to_case(Case::Snake);
            let client = service.name.to_case(Case::Pascal);

            let endpoint_fn = endpoint.name.to_case(Case::Snake);
            let endpoint_fn_in = if endpoint.input_type != "()" {
                "payload.body"
            } else {
                "()"
            };

            rtn = rtn + [
                format!(
                    "#[derive(serde::Serialize, serde::Deserialize)]\npub struct In{out_type} {{ ip : String, {body}}}\n",
                ),
                format!(
                    "pub async fn {fn_name}(Json(payload) : Json<In{out_type}>) -> Result<Json<{rtn_type}>, StatusCode> {{",
                ),
                format!(
                    r#"let mut connect = {module}_client::{client}Client::connect(String::from("http://") + payload.ip.as_str()).await.map_err(|_| StatusCode::MISDIRECTED_REQUEST)?;"#,
                ),
                format!(
                    r#"Ok(Json(connect.{endpoint_fn}({endpoint_fn_in}).await.map_err(|_| StatusCode::BAD_REQUEST)?.into_inner()))"#, 
                ),
                format!(
                    "}}"
                )
            ].join("\n").as_str() + "\n";
        }
    }

    rtn.parse().unwrap()
}

fn get_fn_name(service_name: &str, enpoint_name: &str) -> String {
    format!(
        "{}_{}",
        service_name.to_case(Case::Snake),
        enpoint_name.to_case(Case::Snake)
    )
}
