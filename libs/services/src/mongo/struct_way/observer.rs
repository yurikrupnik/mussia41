use std::cell::RefCell;
use std::rc::Rc;

trait Observer {
    fn update(&self);
}

trait Subject {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn notify_observers(&self);
}

struct ConcreteSubject {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
}

impl ConcreteSubject {
    fn new() -> Self {
        ConcreteSubject {
            observers: Vec::new(),
        }
    }
}

impl Subject for ConcreteSubject {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn notify_observers(&self) {
        for observer in &self.observers {
            observer.borrow().update();
        }
    }
}

struct ConcreteObserver;

impl Observer for ConcreteObserver {
    fn update(&self) {
        println!("Observer notified!");
    }
}
