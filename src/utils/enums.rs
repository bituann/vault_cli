enum InvalidCommand {
	
}

pub enum Check {
	Valid,
	Invalid(String),
}

pub enum Outcome {
	Success(String),
	Fail(String),
}