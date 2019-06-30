use shared::guiddef::GUID;

pub struct Channel {
    pub name: Option<String>,
    pub index: Option<u32>,
    pub id: Option<u32>,
    pub imported: bool,
    pub message_id: Option<u32>,
}

pub struct Level {
    pub name: Option<String>,
    pub id: Option<u32>,
    pub message_id: Option<u32>,
}

pub struct Task {
    pub name: Option<String>,
    pub guid: Option<String>,
    pub value: Option<u32>,
    pub message_id: Option<u32>,
}

pub struct OpCode {
    pub name: Option<String>,
    pub opcode_value: Option<u16>,
    pub task_id: Option<u16>,
    pub message_id: Option<u32>,
}

pub struct Keyword {
    pub name: Option<String>,
    pub mask: Option<u64>,
    pub message_id: Option<u32>,
}

pub struct PubMetadata {
    pub guid: Option<String>,

    pub resource_file_path: Option<String>,
    pub parameter_file_path: Option<String>,
    pub message_file_path: Option<String>,

    pub help_link: Option<String>,

    pub message_id: Option<u32>,

    pub channels: Vec<Channel>,
    pub levels: Vec<Level>,
    pub tasks: Vec<Task>,
    pub opcodes: Vec<OpCode>,
    pub keywords: Vec<Keyword>,
}