use ecdar_protobuf_transpiler::CompileVariables;
use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream;
use quote::quote;
use convert_case::*;

#[proc_macro]
pub fn add_endpoints(app_name: TS) -> TS{
    let app_name = TokenStream::from(app_name);

    let endpoints = ecdar_protobuf_transpiler::compile(|var| {
        let CompileVariables {
            fn_name,
            endpoint_name,
            service_name,
            ..
        } = var;

        let route = format!("/{}/{}", 
            service_name.to_string().to_case(Case::Snake), 
            endpoint_name.to_string().to_case(Case::Snake)
        );

        quote!{
            route(#route, post(#fn_name))
        }
    });

    quote!{
        #app_name.#(#endpoints).*
    }.into()
}

#[proc_macro]
pub fn add_endpoint_functions(_: TS) -> TS{
    let functions = ecdar_protobuf_transpiler::compile(|var|{
        let CompileVariables {
            in_struct,
            in_struct_name,
            in_struct_has_body,
            rtn_struct,
            fn_name,
            client,
            endpoint_name,
            ..
        } = var;

        let payload_body = if in_struct_has_body {
            quote!{ payload.body }
        }  else {
            quote!{()}
        };
        
        quote!{
            #in_struct
            pub async fn #fn_name(Json(payload) : Json<#in_struct_name>)
                -> Result<Json<#rtn_struct>, StatusCode> {
                let mut connect = #client::connect(format!("http://{}", payload.ip.as_str()))
                    .await
                    .map_err(|_| StatusCode::MISDIRECTED_REQUEST)?;
                let res = connect.#endpoint_name(#payload_body)
                    .await
                    .map_err(|_| StatusCode::BAD_REQUEST)?
                    .into_inner();
                Ok(Json(res))
            }
        }
    });

    quote!{
        #(#functions)*
    }.into()
}

