use crate::command::Command;

pub struct Pwd {
    cmd: Command::Pwd
}

impl Pwd {
    pub fn handle(&self) -> Result<(), WinuxError> {
        let cur_dir: PathBuf = env::current_dir()
            .map_err(|e| WinuxError::SystemError{ err: e})?;
                
        Ok(println!("Current Directory: {}", cur_dir.display()))
    }
}