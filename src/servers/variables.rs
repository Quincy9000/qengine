use std::{
    any::{Any, TypeId},
    collections::HashMap,
    net::TcpListener,
};

const IP: &str = "127.0.0.1";
const PORT: &str = "8720";

pub struct Server {
    data: HashMap<TypeId, Box<dyn Any>>,
    server: TcpListener,
}

impl Server {
    pub fn start() -> Self {
        Self {
            data: HashMap::new(),
            server: TcpListener::bind(format!("{}:{}", IP, PORT)).unwrap(),
        }
    }

    pub fn add<T: 'static>(&mut self, t: T) {
        let id = TypeId::of::<T>();
        self.data.insert(id, Box::new(t));
    }

    pub fn remove<T: 'static>(&mut self) -> Option<Box<T>> {
        let id = TypeId::of::<T>();
        let data = self.data.remove(&id);
        if let Some(d) = data {
            Some(d.downcast().unwrap())
        } else {
            None
        }
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        let id = TypeId::of::<T>();
        if let Some(t) = self.data.get(&id) {
            Some(t.downcast_ref().unwrap())
        } else {
            None
        }
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let id = TypeId::of::<T>();
        if let Some(t) = self.data.get_mut(&id) {
            Some(t.downcast_mut().unwrap())
        } else {
            None
        }
    }
}

#[test]
fn get_then_remove() {
    let mut server = Server::start();

    server.add(1i32);
    server.add(123f32);

    println!("{:?}", server.get::<i32>());
    println!("{:?}", server.get::<f32>());

    println!("{:?}", server.remove::<i32>());
    println!("{:?}", server.remove::<f32>());
}
