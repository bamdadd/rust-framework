pub trait HasId {
    fn get_id(&self) -> String;
    fn set_id(&mut self, id: String);

}