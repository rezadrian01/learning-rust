use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

// use std::{cmp::Ordering, io};
// use std::io::{self, Write};
// use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

fn function1 () -> Result {
    Ok(())
}

fn function2 () -> IoResult<()>{
    Ok(())
}
