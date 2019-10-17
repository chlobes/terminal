use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::io::{BufRead, stdin};

pub struct Terminal<T: 'static + Parse + std::marker::Send> {
	recver: Receiver<T>,
}

impl <T: Parse + std::marker::Send> Terminal<T> {
	pub fn start() -> Self {
		let (sender, recver) = channel();
		
		thread::spawn(move|| {
			let stdin = stdin();
			for line in stdin.lock().lines() {
				let line = line.expect("terminal failed to read line");
				
				if let Some(command) = T::parse(&line) {
					if let Err(_) = sender.send(command) {
						break
					}
				}
			}
		});
		
		Terminal {
			recver: recver,
		}
	}
	
	pub fn next(&self) -> T {
		self.recver.recv().unwrap()
	}
	
	pub fn try_next(&self) -> Option<T> {
		if let Ok(x) = self.recver.try_recv() {
			Some(x)
		} else {
			None
		}
	}
	
	pub fn iter(&self) -> ::std::sync::mpsc::Iter<T> {
		self.recver.iter()
	}
	
	pub fn try_iter(&self) -> ::std::sync::mpsc::TryIter<T> {
		self.recver.try_iter()
	}
}

pub trait Parse {
	fn parse(line: &String) -> Option<Self>
	where	Self: std::marker::Sized;
}

impl Parse for String {
	fn parse(line: &String) -> Option<Self> {
		Some(line.clone())
	}
}
