use std::rc::Rc; 
use std::cell::RefCell;

struct Database {
    max_connections: u32,
}

struct AuthService {
    _db: Rc<RefCell<Database>>
}

struct ConnectionService {
    _db: Rc<RefCell<Database>>
}

impl AuthService {
    fn auth(&self) {
        println!("Auth");
    }    
}

impl ConnectionService {
    fn connect(&self) {
        println!("Connect");
    }
}

// Rc is a reference counting pointer, it is used to share the ownership of the data.
fn main() {
    let db = Rc::new(RefCell::new(Database { max_connections: 100 }));

    // Rc::clone increments the reference count of the Rc instance.
    // It does not create a new instance of the Rc, it just increments the reference count.
    let auth_service = AuthService {
        _db: Rc::clone(&db)
    };

    let connection_service = ConnectionService {
        _db: Rc::clone(&db)
    };
    
    auth_service.auth();
    connection_service.connect();

    // Rc is used to share the ownership of the data. But we cannot mutate the data inside the Rc instance.
    // We need to use RefCell to mutate the data inside the Rc instance.
    // It's a inside mutability pattern.
    // Compiler will check the borrow rules at runtime.
    let mut r1 = db.borrow_mut();
    r1.max_connections = 200;
    // We cannot have more than one mutable reference to the data at the same time. But we will not get any compile time error.
    // let mut r2 = db.borrow_mut(); // This will panic at runtime, but not at compile time.
    println!("Max connections: {}", r1.max_connections);
}