// --------- Decorator ---------

// # Dynamic

use std::rc::Rc;

trait Bomb {
    fn boom(&self) -> String;
}

struct C4 {
    size: usize,
}

impl Bomb for C4 {
    fn boom(&self) -> String {
        format!("Detonate C4, size - {}", self.size)
    }
}

struct DefaultBomb {
    size: usize,
}

impl Bomb for DefaultBomb {
    fn boom(&self) -> String {
        format!("Detonate DefaultBomb, size - {}", self.size)
    }
}

trait Decorator {
    fn new(some_bomb: Rc<dyn Bomb>) -> Self;
}

struct DDecorator {
    bomb: Rc<dyn Bomb>,
}

impl Decorator for DDecorator {
    fn new(some_bomb: Rc<dyn Bomb>) -> Self {
        Self { bomb: some_bomb }
    }
}

impl Bomb for DDecorator {
    fn boom(&self) -> String {
        format!("{} in SDecorator", self.bomb.boom())
    }
}

// # Static

trait StDecorator {
    fn new(bomb: DefaultBomb) -> Self;
}

struct SDecorator {
    bomb: DefaultBomb,
}

impl StDecorator for SDecorator {
    fn new(bomb: DefaultBomb) -> Self {
        Self { bomb: bomb }
    }
}

impl Bomb for SDecorator {
    fn boom(&self) -> String {
        format!("From SDecorator {}", self.bomb.boom())
    }
}

fn apply<F, A, B>(func: F, arg: A) -> B
where
    F: Fn(A) -> B,
{
    func(arg)
}

fn main() {
    // # Dynamic
    let default_bomb = Rc::new(DefaultBomb { size: 10 });
    let c4_bomb = Rc::new(C4 { size: 100 });
    let decorated_default_bomb = DDecorator::new(default_bomb);
    let decorated_c4_bomb = DDecorator::new(c4_bomb);
    println!("{}", decorated_default_bomb.boom());
    println!("{}", decorated_c4_bomb.boom());

    // # Static
    let s_default_bomb = DefaultBomb { size: 10 };
    let s_decorated_default_bomb = SDecorator::new(s_default_bomb);
    println!("{}", s_decorated_default_bomb.boom());
}
