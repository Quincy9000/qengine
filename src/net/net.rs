#![allow(unused)]

use std::{
    borrow::Cow,
    collections::HashMap,
    io::*,
    net::{TcpListener, TcpStream},
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread::{spawn, JoinHandle},
};

const IP: &str = "127.0.0.1";
const PORT: &str = "8720";

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
}

impl Variant {
    pub fn size(&self) -> usize {
        match self {
            Variant::Int(_) => std::mem::size_of::<i32>(),
            Variant::Float(_) => std::mem::size_of::<f32>(),
            Variant::Bool(_) => std::mem::size_of::<bool>(),
            Variant::String(_) => std::mem::size_of::<String>(),
        }
    }

    pub fn kind(&self) -> &str {
        match self {
            Variant::Int(_) => "i",
            Variant::Float(_) => "f",
            Variant::Bool(_) => "b",
            Variant::String(_) => "s",
        }
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Variant::Bool(b) => b.to_string(),
                Variant::Float(f) => f.to_string(),
                Variant::Int(i) => i.to_string(),
                Variant::String(s) => s.to_string(),
            }
        )
    }
}

impl From<i32> for Variant {
    fn from(i: i32) -> Self {
        Variant::Int(i)
    }
}

impl From<f32> for Variant {
    fn from(i: f32) -> Self {
        Variant::Float(i)
    }
}

impl From<bool> for Variant {
    fn from(i: bool) -> Self {
        Variant::Bool(i)
    }
}

impl From<String> for Variant {
    fn from(i: String) -> Self {
        Variant::String(i)
    }
}

pub struct Server {
    data: Arc<Mutex<HashMap<String, Variant>>>,
    handle: Option<JoinHandle<()>>,
}

impl Drop for Server {
    fn drop(&mut self) {
        self.handle.take().unwrap().join().unwrap()
    }
}

impl Server {
    pub fn start() -> Server {
        let data = Arc::new(Mutex::new(HashMap::new()));
        let thread_data = data.clone();

        Server {
            data,
            handle: Some(spawn(move || {
                let data = thread_data;
                let listener = TcpListener::bind(format!("{}:{}", IP, PORT)).unwrap();

                loop {
                    if let Ok((mut stream, addr)) = listener.accept() {
                        let mut vec = Vec::new();
                        stream.read_to_end(&mut vec).unwrap();
                        if let Cow::Borrowed(s) = String::from_utf8_lossy(&vec) {
                            match s {
                                s if s.starts_with("a") => {
                                    let mut splits = s.split(":").skip(1); //skip first because we know its "a" aka add
                                    let kind = splits.next();
                                    let name = splits.next().unwrap();
                                    let variant = splits.next().unwrap();
                                    match kind {
                                        Some("i") => {
                                            if let Ok(mut guard) = data.lock() {
                                                guard.insert(
                                                    name.to_string(),
                                                    variant.parse::<i32>().unwrap().into(),
                                                );
                                            }
                                        }
                                        Some("f") => {
                                            if let Ok(mut guard) = data.lock() {
                                                guard.insert(
                                                    name.to_string(),
                                                    variant.parse::<f32>().unwrap().into(),
                                                );
                                            }
                                        }
                                        Some("b") => {
                                            if let Ok(mut guard) = data.lock() {
                                                guard.insert(
                                                    name.to_string(),
                                                    variant.parse::<bool>().unwrap().into(),
                                                );
                                            }
                                        }
                                        Some("s") => {
                                            if let Ok(mut guard) = data.lock() {
                                                guard.insert(
                                                    name.to_string(),
                                                    variant.to_string().into(),
                                                );
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                s if s.starts_with("q") => break,
                                _ => {}
                            }
                        }
                    }
                }
            })),
        }
    }
}

impl Server {
    pub fn add<T: Into<Variant>>(&mut self, s: &str, t: T) {
        if let Ok(mut guard) = self.data.lock() {
            guard.insert(s.to_string(), t.into());
        }
    }

    pub fn remove(&mut self, name: &str) -> Option<Variant> {
        if let Ok(mut guard) = self.data.lock() {
            guard.remove(name)
        } else {
            None
        }
    }

    pub fn get(&self, name: &str) -> Option<Variant> {
        if let Ok(guard) = self.data.lock() {
            if let Some(data) = guard.get(name) {
                Some(data.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn quit(self) {
        let mut connection = TcpStream::connect(format!("{}:{}", IP, PORT)).unwrap();
        connection.write(b"q").unwrap();
    }
}

pub struct ServerConnection {
    connection: TcpStream,
}

impl ServerConnection {
    pub fn new() -> Self {
        Self {
            connection: TcpStream::connect(format!("{}:{}", IP, PORT))
                .expect("Failed to connect to server"),
        }
    }

    fn add<T: Into<Variant>>(&mut self, name: &str, t: T) {
        let variant = t.into();
        self.connection
            .write(format!("a:{}:{}:{}", &variant.kind(), name, variant).as_bytes())
            .unwrap();
    }

    fn remove(&mut self, name: &str) -> Option<Variant> {
        todo!()
    }

    fn get(&self, name: &str) -> Option<&Variant> {
        todo!()
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut Variant> {
        todo!()
    }
}

#[test]
fn server_get_add() {
    let mut server = Server::start();

    server.add("Health", 1i32);
    server.add("Magic", 123f32);

    assert!(Variant::from(123f32) == server.get("Magic").unwrap());
    assert!(Variant::from(1i32) == server.get("Health").unwrap());

    {
        let mut c = ServerConnection::new();
        c.add("Armor", 12);
    }

    assert!(Variant::from(123f32) == server.get("Magic").unwrap());
    assert!(Variant::from(1i32) == server.get("Health").unwrap());
    assert!(Variant::from(12i32) == server.get("Armor").unwrap());

    server.quit();
}
