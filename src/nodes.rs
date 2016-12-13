pub trait Expr {
    fn auditor_stamps(&self);
}
pub trait Pattern {
    fn auditor_stamps(&self);
}

#[derive(Debug)]
pub struct FinalPattern;
impl Pattern for FinalPattern {
    fn auditor_stamps(&self) {
        println!("New FinalPattern!")
    }
}

#[derive(Debug)]
pub struct NullExpr;
impl Expr for NullExpr {
    fn auditor_stamps(&self) {
        println!("New NullExpr")
    }
}

#[derive(Debug)]
pub struct CharExpr {
    expr: char,
}
impl CharExpr {
    pub fn new(c: char) -> CharExpr {
        CharExpr {
            expr: c
        }
    }
}
impl Expr for CharExpr {
    fn auditor_stamps(&self) {
        println!("New CharExpr")
    }
}
