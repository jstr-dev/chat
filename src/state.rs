use druid::{Data, Lens};
use druid::im::Vector;

#[derive(Clone, Data, Lens)]
pub struct Message
{
    pub text: String,
    pub date: u64 
}

#[derive(Clone, Data, Lens)]
pub struct AppState
{
    pub message: String,
    pub history: Vector<Message>
}