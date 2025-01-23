pub trait EntityFilterableTrait {
    fn to_condition<E>(&self, base_condition: sea_orm::Condition) -> sea_orm::Condition
    where
        E: sea_orm::EntityTrait;
}
