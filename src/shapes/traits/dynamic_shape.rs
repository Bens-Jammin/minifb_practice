use super::{drawable::Drawable, movable::Movable};


/// to have a vector of objects that are drawable AND movable, i need a composite trait 
pub trait DynamicShape: Drawable + Movable {}
impl<T: Drawable + Movable> DynamicShape for T {}