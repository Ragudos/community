use inflector::Inflector;
use proc_macro::TokenStream;

#[proc_macro_derive(SqlxDbTable, attributes(id))]
pub fn sqlx_db_table_macro(input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        ident, data, attrs, ..
    } = syn::parse_macro_input!(input);

    match data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
            ..
        }) => {
            let config = Config::new(&attrs, &ident, &named);
            let static_schema = build_static_schema(&config);
            let id_ty = config
                .named
                .iter()
                .find(|f| f.ident.as_ref() == Some(&config.id_column_name))
                .map(|f| &f.ty)
                .expect("Expected type of primary id");
            let model_schema_ident = &config.model_schema_ident;
            let id_column_name = &config.id_column_name;

            quote::quote! (
                pub struct SqlxDbMetadata<'a, C: usize> {
                    table_name: &'a str,
                    id_column_name: &'a str,
                    columns: [&'a str; C],
                    select_sql: &'a str,
                    select_by_id_sql: &'a str
                }

                pub trait SqlxDbSchema {
                    type Id: Copy + Send + Sync;
                    fn table_name() -> &'static str;
                    fn id(&self) -> Self::Id;
                    fn id_column_name() -> &'static str;
                    fn columns() -> &'static [&'static str];
                    fn select_sql() -> &'static str;
                    fn select_by_id_sql() -> &'static str;
                }

                #static_schema

                impl SqlxDbSchema for #ident {
                    type Id = #id_ty;
                    fn table_name() -> &'static str {
                        #model_schema_ident.table_name
                    }
                    fn id(&self) -> Self::Id {
                        self.#id_column_name
                    }
                    fn id_column_name() -> &'static str {
                        #model_schema_ident.id_column_name
                    }
                    fn columns() -> &'static [&'static str] {
                        &#model_schema_ident.columns
                    }
                    fn select_sql() -> &'static str {
                        #model_schema_ident.select_sql
                    }
                    fn select_by_id_sql() -> &'static str {
                        #model_schema_ident.select_by_id_sql
                    }
                }
            )
            .into()
        }
        _ => panic!("Only structs with named fields are supported"),
    }
}

fn build_static_schema(config: &Config) -> proc_macro2::TokenStream {
    let model_schema_ident = &config.model_schema_ident;
    let table_name = &config.table_name;

    let id_column_name = config.id_column_name.to_string();
    let col_len = config.named.iter().count();
    let cols = config
        .named
        .iter()
        .flat_map(|f| &f.ident)
        .map(|f| syn::LitStr::new(format!("{}", f).as_str(), f.span()));
    let sql_queries = build_sql_queries(config);

    quote::quote! (
        #[automatically_derived]
        static #model_schema_ident: SqlxDbMetadata<'static, #col_len> = SqlxDbMetadata {
            table_name: #table_name,
            id_column_name: #id_column_name,
            columns: [#(#cols),*],
            #sql_queries
        };
    )
}

fn build_sql_queries(config: &Config) -> proc_macro2::TokenStream {
    let table_name = &config.table_name;

    let id_column_name = format!("{}.{}", &config.table_name, config.id_column_name);

    let col_list = config
        .named
        .iter()
        .flat_map(|f| &f.ident)
        .map(|f| format!("{}.{}", &config.table_name, &f.to_string()))
        .collect::<Vec<_>>()
        .join(", ");

    let select_sql = format!("SELECT {} FROM {}", col_list, table_name);
    let select_by_id_sql = format!(
        "SELECT {} FROM {} WHERE {} = ?",
        col_list, table_name, id_column_name
    );

    quote::quote! (
        select_sql: #select_sql,
        select_by_id_sql: #select_by_id_sql,
    )
}

#[allow(dead_code)]
struct Config<'a> {
    crate_name: String,
    ident: &'a syn::Ident,
    named: &'a syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    id_column_name: syn::Ident,
    model_schema_ident: syn::Ident,
    table_name: String,
}

impl<'a> Config<'a> {
    fn new(
        attrs: &[syn::Attribute],
        ident: &'a syn::Ident,
        named: &'a syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    ) -> Self {
        let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
        // find the field with #[id] attr
        let id_attr = &named
            .iter()
            .find(|f| f.attrs.iter().any(|a| a.path().is_ident("a")))
            .and_then(|f| f.ident.as_ref());
        // Get the field with attr #[id] or the very first field
        let id = id_attr
            .unwrap_or_else(|| {
                named
                    .iter()
                    .flat_map(|f| &f.ident)
                    .next()
                    .expect("There should be a field in the struct.")
            })
            .clone();

        let model_schema_ident =
            quote::format_ident!("{}_SCHEMA", ident.to_string().to_screaming_snake_case());

        let table_name = ident.to_string().to_table_case();

        Config {
            crate_name,
            ident,
            id_column_name: id,
            named,
            model_schema_ident,
            table_name,
        }
    }
}
