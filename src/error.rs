//pub(crate) struct Error {
//    status: i8,
//    message: String
//}

pub(crate) fn result_handler<T, E>(res: Result<T, E>) -> Option<T>{
    match res {
        Ok(t) => Some(t),
        Err(_e) => {
            println!("An error surfaced, verify command arguments and try again");
        None
        }
    }
} // TODO: Improve to print specific errors, probably with a struct that holds a status and a message that will be a value of an enum containing a string