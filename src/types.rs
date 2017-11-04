use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct GosType {
    typ: Rc<RefCell<GosType>>,
    parent: Rc<RefCell<GosType>>,
    slots: HashMap<String, Rc<RefCell<GosObject>>>,
}

trait GosObject {
    fn get_type(&self) -> Rc<RefCell<GosType>>;
    /* obj.type => obj.get_type() */
    fn accept(&mut self, msg: Rc<RefCell<GosObject>>) -> Rc<RefCell<GosObject>>;
    /* (obj msg) => obj.accept(msg) */
    fn get_slot(&self, slot: String) -> Option<Rc<RefCell<GosObject>>>;
    /* obj.s => obj.get_slot("s") */
}

impl GosObject for GosType {
    fn get_type(&self) -> Rc<RefCell<GosType>> {
        self.typ.clone()
    }

    fn accept(&mut self, msg: Rc<RefCell<GosObject>>) -> Rc<RefCell<GosObject>> {
        msg.clone()
    }

    fn get_slot(&self, slot: String) -> Option<Rc<RefCell<GosObject>>> {
        if let Some(attr) = self.slots.get(&slot) {
            Some(attr.clone())
        } else {
            None
        }
    }
}

pub struct GosVal {
    typ: Rc<RefCell<GosType>>,
    slots: HashMap<String, Rc<RefCell<GosObject>>>,
}
        
