use std::fmt::{Debug, Display};


pub trait DisplayPrint {
    fn print(&self);
    fn println(&self);
}

pub trait DebugPrint {
    fn dprint(&self);
    fn dprintln(&self);
}

impl<T: Display>  DisplayPrint for T {
    fn print(&self) {
        print!("{}", self);
    }

    fn println(&self) {
        println!("{}", self);
    }
}

impl<T: Debug> DebugPrint for T {
    fn dprint(&self) {
        print!("{:?}", self);
    }

    fn dprintln(&self) {
        println!("{:?}", self);
    }
}
