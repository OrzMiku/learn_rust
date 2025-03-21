use std::rc::Rc;

struct Database {}

struct AuthService {
    _db: Rc<Database>
}

struct ConnectionService {
    _db: Rc<Database>
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
    let db = Rc::new(Database {});

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
}