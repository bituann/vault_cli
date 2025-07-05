enum InvalidCommand {
	
}

pub enum Check {
	Valid,
	Invalid(String),
}

pub enum Outcome<T> {
	Success(T),
	Fail(String),
}