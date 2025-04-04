pub trait Consumer:Send+Sync{
    fn consume(&self,message:String);
}