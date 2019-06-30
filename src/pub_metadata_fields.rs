use winapi::um::winevt;

pub struct PubMetaField {
    pub id: u32,
    pub name: &'static str,
}

pub static PUB_META_FIELDS: [PubMetaField; 29] = [
    PUBLISHER_GUID,
    PARAMETER_FILE_PATH,
    MESSAGE_FILE_PATH,
    HELP_LINK,
    PUBLISHER_MESSAGE_ID,
    CHANNEL_REFERENCES,
    CHANNEL_REFERENCE_PATH,
    CHANNEL_REFERENCE_INDEX,
    CHANNEL_REFERENCE_ID,
    CHANNEL_REFERENCE_FLAGS,
    CHANNEL_REFERENCE_MESSAGE_ID,
    LEVELS,
    LEVEL_NAME,
    LEVEL_VALUE,
    LEVEL_MESSAGE_ID,
    TASKS,
    TASK_NAME,
    TASK_EVENT_GUID,
    TASK_VALUE,
    TASK_MESSAGE_ID,
    OPCODES,
    OPCODE_NAME,
    OPCODE_VALUE,
    OPCODE_MESSAGE_ID,
    KEYWORDS,
    KEYWORD_NAME,
    KEYWORD_VALUE,
    KEYWORD_MESSAGE_ID,
    PROPERTY_ID_END,
];

pub const PUBLISHER_GUID: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataPublisherGuid,
    name: "Publisher Guid",
};

pub const PARAMETER_FILE_PATH: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataParameterFilePath,
    name: "Parameter File Path",
};

pub const MESSAGE_FILE_PATH: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataMessageFilePath,
    name: "Message File Path",
};

pub const HELP_LINK: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataHelpLink,
    name: "Help Link",
};

pub const PUBLISHER_MESSAGE_ID: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataPublisherMessageID,
    name: "Publisher Message Id",
};

pub const CHANNEL_REFERENCES: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataChannelReferences,
    name: "Channel References",
};

pub const CHANNEL_REFERENCE_PATH: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataChannelReferencePath,
    name: "Channel Reference Path",
};

pub const CHANNEL_REFERENCE_INDEX: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataChannelReferenceIndex,
    name: "Channel Reference Index",
};

pub const CHANNEL_REFERENCE_ID: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataChannelReferenceID,
    name: "Channel Reference Id",
};

pub const CHANNEL_REFERENCE_FLAGS: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataChannelReferenceFlags,
    name: "Channel Reference Flags",
};

pub const CHANNEL_REFERENCE_MESSAGE_ID: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataChannelReferenceMessageID,
    name: "Channel Reference Message Id",
};

pub const LEVELS: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataLevels,
    name: "Levels",
};

pub const LEVEL_NAME: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataLevelName,
    name: "Level Name",
};

pub const LEVEL_VALUE: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataLevelValue,
    name: "Level Value",
};

pub const LEVEL_MESSAGE_ID: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataLevelMessageID,
    name: "Level Message Id",
};

pub const TASKS: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataTasks,
    name: "Tasks",
};

pub const TASK_NAME: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataTaskName,
    name: "Task Name",
};

pub const TASK_EVENT_GUID: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataTaskEventGuid,
    name: "Task Event Guid",
};

pub const TASK_VALUE: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataTaskValue,
    name: "Task Value",
};

pub const TASK_MESSAGE_ID: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataTaskMessageID,
    name: "Task Message Id",
};

pub const OPCODES: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataOpcodes,
    name: "Opcodes",
};

pub const OPCODE_NAME: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataOpcodeName,
    name: "Opcode Name",
};

pub const OPCODE_VALUE: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataOpcodeValue,
    name: "Opcode Value",
};

pub const OPCODE_MESSAGE_ID: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataOpcodeMessageID,
    name: "Opcode Message Id",
};

pub const KEYWORDS: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataKeywords,
    name: "Keywords",
};

pub const KEYWORD_NAME: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataKeywordName,
    name: "Keyword Name",
};

pub const KEYWORD_VALUE: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataKeywordValue,
    name: "Keyword Value",
};

pub const KEYWORD_MESSAGE_ID: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataKeywordMessageID,
    name: "Keyword Message Id",
};

pub const PROPERTY_ID_END: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataPropertyIdEND,
    name: "Property Id End",
};
