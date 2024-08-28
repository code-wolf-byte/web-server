use std::process::{Command, Child};

struct Process{
    name: String,
    pid: u32,
    command: String,
    child: Optiion<Child>,
}
