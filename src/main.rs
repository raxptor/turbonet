mod defs;

use defs::*;

struct Cucumber
{

}

impl GameInstance for Cucumber
{
    fn connect(user: defs::UserId) 
    {

    }
}

impl Cucumber {
    pub fn make() -> Cucumber
    {
        return Cucumber {
        }
    }
}

fn main() 
{
    let k = Cucumber::make();
    println!("Hello, world!");
}