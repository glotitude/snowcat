pub trait Blueprint {
    fn get_blueprint() -> String;

    fn get_table() -> String;
}

pub trait Transferable {
    fn serialize(&self) -> String;
}