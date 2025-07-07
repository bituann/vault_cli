enum InvalidCommand {
	
}

pub enum Check<T> {
	Valid(T),
	Invalid(String),
}

pub enum Outcome<T> {
	Success(T),
	Fail(String),
}