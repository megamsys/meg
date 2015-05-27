use std::env;

pub struct AccountFields {
    first_name:     Option<String>,
    last_name:      Option<String>,
    phone:          Option<String>,
    email:          Option<String>,
    api_key:        Option<String>,
    password:       Option<String>,
    authority:      Option<String>,
    password_reset_key: Option<String>,
    password_reset_sent_at: Option<String>,
}


pub struct AccountCreateFields {
    email: Option<String>,
    //first_name: Option<String>,
    //last_name: Option<String>,
    //phone: Option<String>,
    //password: Option<String>,


}
