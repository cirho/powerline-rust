use std::marker::PhantomData;

use super::Module;
use crate::{terminal::Color, utils, Segment, R};

pub struct Host<S: HostScheme> {
	show_on_local: bool,
	scheme: PhantomData<S>,
}

pub trait HostScheme {
	const HOSTNAME_FG: Color;
	const HOSTNAME_BG: Color;
}

impl<S: HostScheme> Host<S> {
	pub fn new() -> Host<S> {
		Host {
			show_on_local: true,
			scheme: PhantomData,
		}
	}

	pub fn show_on_remote_shell() -> Host<S> {
		Host {
			show_on_local: false,
			scheme: PhantomData,
		}
	}
}

impl<S: HostScheme> Module for Host<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
		if self.show_on_local || utils::is_remote_shell() {
			if let Some(host) = hostname::get_hostname() {
				segments.push(Segment::simple(format!(" {} ", host), S::HOSTNAME_FG, S::HOSTNAME_BG));
			}
		}
		Ok(())
	}
}
