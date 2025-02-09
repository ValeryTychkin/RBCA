pub trait EntityFilterableTrait {
    fn to_condition<E>(&self) -> sea_orm::Condition
    where
        E: sea_orm::EntityTrait;
}
