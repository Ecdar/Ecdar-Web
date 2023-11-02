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
                ".route(\"/{name}/{}\", get({}))",
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

            let function_body = 
                format!(
                    "\tlet mut connect = {}_client::{}Client::connect(String::from(\"http://\") + payload.ip.as_str()).await.map_err(|_| StatusCode::MISDIRECTED_REQUEST)?;",
                    service.name.to_case(Case::Snake),
                    service.name.to_case(Case::Pascal)
                )
                + format!(
                    "\n\tOk(Json(connect.{}({}).await.map_err(|_| StatusCode::BAD_REQUEST)?.into_inner()))", 
                    endpoint.name.to_case(Case::Snake),
                    if endpoint.input_type != "()" { "payload.body" } else { "()" }
                ).as_str();
            rtn = rtn 
            + format!(
                "#[derive(serde::Serialize, serde::Deserialize)]\npub struct In{} {{ ip : String, {}}}\n",
                fn_name.to_case(Case::Pascal),
                if endpoint.input_type != "()" { format!("body : {}", endpoint.input_type.to_case(Case::Pascal)) } else { "".into() }
            ).as_str()
            + format!(
                "pub async fn {}(Json(payload) : Json<In{}>) -> Result<Json<{}>, StatusCode>{{\n{function_body}\n}}\n",
                fn_name,
                fn_name.to_case(Case::Pascal),
                endpoint.output_type.to_case(Case::Pascal),
            )
            .as_str();
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
