#[derive(Debug)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}
impl Status {
    pub fn is_done(&self) -> bool {
        matches!(self, Status::Completed)
    }
    //call next methodes it will every time to the next status
    pub fn next(&self) -> Self {
        match self {
            Status::Pending => Status::InProgress,
            Status::InProgress => Status::Completed,
            Status::Completed => Status::Completed,
        }
    }
}

pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: Status,
}
impl Task {
    //constructor
    pub fn new(id: u32, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            status: Status::Pending,
        }
    }
    pub fn mark_completed(&mut self) {
        self.status = Status::Completed;
    }
    pub fn print(&self) {
        println!(
            "[{}] - {} - {} - ({:#?}) ",
            self.id, self.title, self.description, self.status
        );
    }
}
