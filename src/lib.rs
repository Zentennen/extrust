#![allow(non_upper_case_globals)]

use std::error::Error;
use std::io;
use std::str::FromStr;
use std::fmt::Debug;

pub type Er = anyhow::Error;
pub type Res<T> = anyhow::Result<T>;
pub type Maybe = Res<()>;
pub const ok: Maybe = Ok(());

pub fn cin<T>() -> Res<T> 
where T: FromStr, <T as FromStr>::Err: Error + Send + Sync + 'static
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}

pub fn cout<T: Debug>(output: T){
    println!("{output:?}");
}

pub trait Utf8Container {
    fn slice(&self, start: usize, end: usize) -> Option<&str>;
    fn from(&self, start: usize) -> Option<&str>;
    fn to(&self, end: usize) -> Option<&str>;
}

impl Utf8Container for str {
    fn slice(&self, start: usize, end: usize) -> Option<&str> {
        let start = self.char_indices().nth(start)?.0;
        let end  = self.char_indices().nth(end)?.0;
        Some(&self[start..end])
    }
    fn from(&self, start: usize) -> Option<&str> {
        let start = self.char_indices().nth(start)?.0;
        Some(&self[start..])
    }
    fn to(&self, end: usize) -> Option<&str> {
        let end  = self.char_indices().nth(end)?.0;
        Some(&self[..end])
    }
}

impl Utf8Container for String {
    fn slice(&self, start: usize, end: usize) -> Option<&str> {
        let start = self.char_indices().nth(start)?.0;
        let end  = self.char_indices().nth(end)?.0;
        Some(&self[start..end])
    }
    fn from(&self, start: usize) -> Option<&str> {
        let start = self.char_indices().nth(start)?.0;
        Some(&self[start..])
    }
    fn to(&self, end: usize) -> Option<&str> {
        let end  = self.char_indices().nth(end)?.0;
        Some(&self[..end])
    }
}