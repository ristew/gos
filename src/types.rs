use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub enum GosType {
    Symbol(String),
    Int(isize),
    Float(f64),
    Str(String),
}

pub struct GosObject {
    name: String,
    typ: GosType,
    parent: Rc<RefCell<GosObject>>,
    slots: HashMap<String, Rc<RefCell<GosObject>>>,
}
