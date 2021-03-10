#[cfg(feature = "f64")]
pub type Float = f64;

#[cfg(not(feature = "f64"))]
pub type Float = f32;

#[cfg(feature = "rayon")]
pub type Rc<T> = std::sync::Arc<T>;

#[cfg(not(feature = "rayon"))]
pub type Rc<T> = std::rc::Rc<T>;
