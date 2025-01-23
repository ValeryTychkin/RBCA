mod entity;
use entity::filter;
use proc_macro::TokenStream;

/// Custom derive macro for implementing the `EntityFilterableTrait` on a struct.
///
/// This macro automatically generates filtering logic for the fields of the struct. It uses the `filter`
/// attribute to specify rules and options for filtering fields, and maps them to conditions based on the
/// rules like `eq`, `ne`, `like`, etc.
///
/// This solution is highly flexible due to the use of generics. It doesn't bind the struct to a specific database model;
/// instead, it requires the struct to have fields that match those in the model. The `to_condition` method can then be
/// used with any model that contains the same fields, allowing you to construct dynamic queries without coupling the struct
/// to a particular model.
///
/// # Example
///
/// ```rust
/// #[derive(JsonSchema, FromForm, EntityFilterable)]
/// pub struct BaseQuery {
///     id: Option<Uuid>,
///     #[filter(rule = "like")]
///     title: Option<String>,
///     #[filter(rule = "gte", value_prepare = "v.to_time()", column = "created_at")]
///     pub created_start: Option<OffsetDateTimeForm>,
///     #[filter(rule = "lt", value_prepare = "v.to_time()", column = "created_at")]
///     pub created_end: Option<OffsetDateTimeForm>,
///     #[filter(ignore)]
///     #[field(default = Some(OFFSET_DEFAULT))]
///     pub offset: Option<u64>,
///     #[filter(ignore)]
///     #[field(default = Some(LIMIT_DEFAULT))]
///     pub limit: Option<u64>,
/// }
///
/// // These can be used with any model that contains the same fields:
/// let condition_1 = base_query.to_condition::<message_entity::Entity>(Condition::all());
/// let condition_2 = base_query.to_condition::<post_entity::Entity>(Condition::all());
/// ```
#[proc_macro_derive(EntityFilterable, attributes(filter))]
pub fn derive_entity_filter(input: TokenStream) -> TokenStream {
    filter::impl_entity_filterable(input)
}
