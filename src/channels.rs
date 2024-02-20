use itertools::Itertools;
use serde::{Deserialize, Serialize, Serializer};

#[derive(
    Deserialize,
    Serialize,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[serde(rename_all = "PascalCase")]
pub struct Init {
    pub incoming: String,
    pub scales: String,
    pub channels: Channels,
}

#[derive(
    Deserialize,
    Serialize,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[serde(rename_all = "PascalCase")]
pub struct Channels {
    pub channel: Vec<Channel>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Channel(pub Vec<i32>);

impl<'de> Deserialize<'de> for Channel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let channel_str = String::deserialize(deserializer)?;
        let entries: Result<Vec<i32>, _> =
            channel_str.split(',').map(|e| e.parse()).collect();
        let entries = entries.map_err(Error::custom)?;
        Ok(Self(entries))
    }
}

impl Serialize for Channel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let res = self.0.iter().map(ToString::to_string).join(",");
        serializer.serialize_str(&res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deser_init() {
        const REF_CHANNELS: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Init>
  <Incoming> p p with NNPDF31_nnlo_as_0118/0 </Incoming>
  <Scales> muR = HT, muF = HT </Scales>
  <Channels>
    <Channel> 0,5,1,1,2,2,3,3,4,4,5,5 </Channel>
    <Channel> 1,5,1,-1,2,-2,3,-3,4,-4,5,-5 </Channel>
    <Channel> 2,20,1,2,1,3,1,4,1,5,2,1,2,3,2,4,2,5,3,1,3,2,3,4,3,5,4,1,4,2,4,3,4,5,5,1,5,2,5,3,5,4 </Channel>
    <Channel> 3,20,1,-2,1,-3,1,-4,1,-5,2,-1,2,-3,2,-4,2,-5,3,-1,3,-2,3,-4,3,-5,4,-1,4,-2,4,-3,4,-5,5,-1,5,-2,5,-3,5,-4 </Channel>
    <Channel> 4,5,1,21,2,21,3,21,4,21,5,21 </Channel>
    <Channel> 5,5,-1,1,-2,2,-3,3,-4,4,-5,5 </Channel>
    <Channel> 6,5,-1,-1,-2,-2,-3,-3,-4,-4,-5,-5 </Channel>
    <Channel> 7,20,-1,2,-1,3,-1,4,-1,5,-2,1,-2,3,-2,4,-2,5,-3,1,-3,2,-3,4,-3,5,-4,1,-4,2,-4,3,-4,5,-5,1,-5,2,-5,3,-5,4 </Channel>
    <Channel> 8,20,-1,-2,-1,-3,-1,-4,-1,-5,-2,-1,-2,-3,-2,-4,-2,-5,-3,-1,-3,-2,-3,-4,-3,-5,-4,-1,-4,-2,-4,-3,-4,-5,-5,-1,-5,-2,-5,-3,-5,-4 </Channel>
    <Channel> 9,5,-1,21,-2,21,-3,21,-4,21,-5,21 </Channel>
    <Channel> 10,5,21,1,21,2,21,3,21,4,21,5 </Channel>
    <Channel> 11,5,21,-1,21,-2,21,-3,21,-4,21,-5 </Channel>
    <Channel> 12,1,21,21 </Channel>
  </Channels>
</Init>
"#;
        let init: Init = quick_xml::de::from_str(REF_CHANNELS).unwrap();
        assert_eq!(
            init.channels.channel[5].0,
            [5, 5, -1, 1, -2, 2, -3, 3, -4, 4, -5, 5]
        );
    }
}
