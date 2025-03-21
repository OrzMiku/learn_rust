trait UIComponent {
    fn render(&self) {
        println!("Rendering UIComponent");
    }
}

struct Button {
    label: String
}

struct Text {
    content: String
}

struct Container {
    name: String,
    // child: Container, // This will cause infinite size of Container
    // This is a usecase of Box, to store the heap data and the size of Box is fixed.
    // child: Box<Container> // Because child may be null, so we can use Option
    child: Option<Box<Container>>
}

impl Button {
    fn new(label: &str) -> Button {
        Button {
            label: label.to_string()
        }
    }
}

impl Text {
    fn new(content: &str) -> Text {
        Text {
            content: content.to_string()
        }
    }
}

impl Container {
    fn new(name: &str) -> Container {
        Container {
            name: name.to_string(),
            child: None
        }
    }
}

impl UIComponent for Button {
    fn render(&self) {
        println!("Rendering Button with label: {}", self.label);
    }
}

impl UIComponent for Text {
    fn render(&self) {
        println!("Rendering Text with content: {}", self.content);
    }
}

impl UIComponent for Container {
    fn render(&self) {
        let mut depth: usize = 0;
        println!("|- {}", self.name);
        if let Some(child) = &self.child {
            depth += 1;
            for _ in 0..depth {
                print!("|-");
            }            
            child.render();
        }
    }
}

fn main() {
    // Button A is stored on the stack
    let button_a = Button::new("Button A");
    button_a.render();
    // Button B is stored on the heap and Box is a smart pointer pointing to the heap
    let button_b = Box::new(Button::new("Button B"));
    button_b.render();

    // Button a transfer ownership to button_c, all the data is copied.
    let button_c = button_a;
    button_c.render();
    // Button b transfer ownership to button_d, but the data is not copied, only the pointer is copied.
    let button_d = button_b;
    button_d.render();

    // Text A is stored on the heap
    let text_a = Box::new(Text::new("Text A"));
    text_a.render();

    // The another usecase of Box is to store different types implementing the same trait
    let components: Vec<Box<dyn UIComponent>> = vec![
        Box::new(button_c),
        button_d,
        text_a
    ];

    println!("=====Render Start=====");
    for component in components {
        component.render();
    }
    println!("======Render End======");

    let mut container_a = Container::new("Container A");
    let container_b = Container::new("Container B");
    container_a.child = Some(Box::new(container_b));
    container_a.render();
}