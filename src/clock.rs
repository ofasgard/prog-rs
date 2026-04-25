use std::cmp::min;
use std::cmp::max;
use uuid::Uuid;

use std::sync::Arc;
use std::sync::Mutex;
use serde::Serialize;
use serde::Deserialize;

pub type ProgressClockMutex = Arc<Mutex<ProgressClock>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressClock {
	name: String,
	size: i32,
	ticks: i32,
	positive: bool,
	id: Uuid
}

impl ProgressClock {
	pub fn new(name : &str, positive: bool) -> ProgressClock {
		ProgressClock {
			name: name.to_string(),
			size: 4,
			ticks: 0,
			positive: positive,
			id: Uuid::new_v4()
		}
	}

	pub fn to_string(&self) -> String {
		serde_json::to_string(&self).unwrap()

	}
	
	pub fn from_string(s : String) -> ProgressClock {
		serde_json::from_str(&s).unwrap()
	}

	pub fn get_name(&self) -> String { self.name.to_string() }
	pub fn get_size(&self) -> i32 { self.size }
	pub fn get_ticks(&self) -> i32 { self.ticks }
	pub fn is_positive(&self) -> bool { self.positive }
	pub fn get_id(&self) -> String { self.id.to_string() }
	
	pub fn set_name(&mut self, name : &str) { self.name = name.to_string(); }
	
	pub fn enlarge(&mut self) {
		self.size = min(max(2, self.size+2), 12);
	}
	
	pub fn reduce(&mut self) {
		self.size = min(max(2, self.size-2), 12);
		if self.ticks > self.size { self.ticks = self.size; }
	}
	
	pub fn set_tick(&mut self, amount : i32) {
		self.ticks = min(max(0, amount), self.size);
	}
	
	pub fn tick(&mut self) {
		self.set_tick(self.ticks + 1);
	}
	
	pub fn untick(&mut self) {
		self.set_tick(self.ticks - 1);
	}
	
	pub fn process_click(&mut self, clicked : i32) {
		if clicked != self.ticks { 
			self.set_tick(clicked);
		} else {
			self.untick();
		}
	}
}
