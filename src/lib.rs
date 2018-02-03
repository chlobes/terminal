use std::sync::mpsc::{channel, Receiver, RecvError};
use std::thread;


pub struct Terminal<T: 'static + Parse + std::marker::Send> {
	recver: Receiver<T>,
}

impl <T: Parse + std::marker::Send> Terminal<T> {
	pub fn start() -> Self {
		let (sender, recver) = channel();
		
		thread::spawn(move|| {
			let mut line = String::new();
			
			loop {
				std::io::stdin().read_line(&mut line).expect("terminal failed to read line");
				
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
	
	pub fn try_next(&self) -> Option<T> {
		if let Ok(x) = self.recver.try_recv() {
			Some(x)
		} else {
			None
		}
	}
	
	pub fn next(&self) -> T {
		self.recver.recv().unwrap()
	}
}

pub trait Parse {
	fn parse(line: &String) -> Option<Self>
	where
		Self: std::marker::Sized;
}