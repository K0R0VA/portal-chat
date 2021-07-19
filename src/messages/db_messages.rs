use actix::Message;

#[derive(Message)]
#[rtype(result = "i32")]
pub struct SignUp {
    pub name: String,
}

#[derive(Message)]
#[rtype(result = "i32")]
pub struct SignIn {
    pub name: String,
}


