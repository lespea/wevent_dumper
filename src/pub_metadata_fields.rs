use winapi::um::winevt;

pub struct PubMetaField {
    pub id: u32,
    pub name: &'static str,
}

pub const META_PUBLISHER_GUID: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataPublisherGuid,
    name: "GUID",
};

pub const META_PUBLISHER_RESOURCE_FPATH: PubMetaField = PubMetaField {
    id: winevt::EvtPublisherMetadataHelpLink,
    name: "Resource File Path",
};
