use std::{
	io::Result,
	os::windows::{io::AsRawHandle},
	process::Command,
};

use crate::{winres::*, CommandGroup, GroupChild};

impl CommandGroup for Command {
	fn group_spawn(&mut self) -> Result<GroupChild> {
		let (job, completion_port) = job_object()?;
		let child = self.spawn()?;
		assign_child(child.as_raw_handle(), job)?;

		Ok(GroupChild::new(child, job, completion_port))
	}
}
