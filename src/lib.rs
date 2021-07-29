#[macro_use]
extern crate log;
extern crate log4rs;

pub mod action;
pub mod discovery;
pub mod eventsourced;
pub mod value_entity;

pub mod handlers;
pub mod permastate;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
