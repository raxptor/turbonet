pub type UserId = u64;

pub trait GameInstance
{
    fn connect(user: UserId);
    fn disconnect(user: UserId);
}