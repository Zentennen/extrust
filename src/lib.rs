use std::error::Error;
use std::io;
use std::str::FromStr;
use std::fmt::Display;

pub type Er = Box<dyn Error>;
pub type Res<T> = Result<T, Er>;

pub fn cin<T>() -> Res<T> 
where T: FromStr, <T as FromStr>::Err: Error + 'static
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}

pub fn cout<T: Display>(output: &T){
    println!("{}", output);
}