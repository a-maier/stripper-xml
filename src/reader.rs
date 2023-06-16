
/// Reader for event files in the XML format defined by STRIPPER
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EventReader<T> {
    stream: T,
    info: EventRecordInfo
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EventRecordInfo {
    nevents: u64,
    nsubevents: u64,
    nreweights: u64,
    alpha_s: u64,
    name: String,
}
