use askama::Template;

#[derive(Debug, Clone, PartialEq)]
pub enum TimingType {
    // Activity,
    Movement,
    Consume,
}

#[derive(Debug, Clone)]
pub struct Timing {
    pub timing_type: TimingType,
    pub start: u64,
    pub stop: u64,
    pub id: usize,
}

#[derive(Template)]
#[template(path = "timer.html")]
pub struct Timer {
     pub oob: bool,
     pub msg: String 
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub(crate) timings: Vec<Timing>,
}