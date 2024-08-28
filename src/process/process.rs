use std::process::{Command, Child};

struct Process{
    name: String,
    pid: u32,
    command: String,
    child: Optiion<Child>,
}


impl Process {

    fn new(name: String, pid: u32, command: String) -> Self {
        Process{
            name: name,
            pid: pid,
            command: command,
            child: None,
        }
    }

}