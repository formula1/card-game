
pub struct ObjectWithEvent {
  listeners: Vec<Box<dyn EventListener>>
}

impl ObjectWithEvent {
  fn addListener(&mut self, listener: Box<dyn EventListener>){
    self.listeners.push(listener);
  }
  fn emitEvent(self){
    for listener in self.listeners {
      let output = listener.run();
      let output2 = listener.run2();
    }
  }
}

struct EventOutput{}

pub trait EventListener {
  fn run(&self)->EventOutput;
  fn run2(self: Box<Self>)->EventOutput;
}


struct ExampleListener {}
impl EventListener for ExampleListener {
  fn run(&self)->EventOutput {
    println!("ok!");
    return EventOutput {}
  }
  fn run2(self: Box<Self>)->EventOutput {
    println!("ok!");
    return EventOutput {}
  }
}

fn main(){
  let mut obj = ObjectWithEvent{ listeners: vec![] };
  obj.addListener(Box::new(ExampleListener{}));
}

