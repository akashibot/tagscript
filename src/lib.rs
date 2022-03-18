mod action;
pub mod block;
mod errors;
mod interface;
mod interpreter;
mod verb;

pub use action::Action;
pub use errors::Error;
pub use interface::{Adapter, Block};
pub use interpreter::{Context, Interpreter, Node};
pub use verb::Verb;

use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, errors::Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
