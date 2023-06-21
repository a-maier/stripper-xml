use serde::{Deserialize, Serialize, Serializer};

use crate::ParseErr;

#[derive(Deserialize, Serialize)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Normalization {
    #[serde(rename = "@name")]
    pub name: String,
    //#[serde(rename = "$field")]
    #[serde(rename = "XSection")]
    pub xsection: XSection,
    #[serde(rename = "Contribution")]
    pub contribution: Contribution,
    #[serde(rename = "NumberOfRejectedEvents")]
    pub number_of_rejected_events: String,
}

// // TODO: this would be the proper way to do this, but it doesn't work
// #[derive(Deserialize, Serialize)]
// #[derive(Clone, Debug, PartialEq, PartialOrd)]
// #[serde(untagged)]
// pub enum XSection {
//     Neg(XSectionNeg),
//     Pos(XSectionPos),
// }

// #[derive(Deserialize, Serialize)]
// #[derive(Clone, Debug, PartialEq, PartialOrd)]
// #[serde(rename_all = "PascalCase")]
// pub struct XSectionNeg {
//     #[serde(rename = "XSNeg")]
//     pub xs_neg: XSScale,
//     pub max_weight_neg: f64,
//     pub total_events_neg: u64,
//     pub accepted_events_neg: u64,
//     pub factor_neg: String,
// }

// #[derive(Deserialize, Serialize)]
// #[derive(Clone, Debug, PartialEq, PartialOrd)]
// #[serde(rename_all = "PascalCase")]
// pub struct XSectionPos {
//     #[serde(rename = "XSPos")]
//     pub xs_pos: XSScale,
//     pub max_weight_pos: f64,
//     pub total_events_pos: u64,
//     pub accepted_events_pos: u64,
//     pub factor_pos: String,
// }

#[derive(Deserialize, Serialize)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct XSection {
    // TODO: the `alias` is an evil hack and breaks serialization
    #[serde(rename = "XSPos", alias = "XSNeg")]
    pub xs_pos: XSScale,
    #[serde(alias = "MaxWeightNeg")]
    pub max_weight_pos: f64,
    #[serde(alias = "TotalEventsNeg")]
    pub total_events_pos: u64,
    #[serde(alias = "AcceptedEventsNeg")]
    pub accepted_events_pos: u64,
    #[serde(alias = "FactorNeg")]
    pub factor_pos: String,
}

#[derive(Deserialize, Serialize)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Contribution {
    #[serde(rename = "@name")]
    pub name: String,
    pub xsection: XSScale,
    pub rw: Reweight,
}

#[derive(Deserialize, Serialize)]
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Reweight {
    pub rwentry: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XSScale (pub [f64; 2]);

impl<'de> Deserialize<'de> for XSScale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let xs_scale_str = String::deserialize(deserializer)?;
        let mut entries = xs_scale_str.split(',');
        let mut xs_scale = [0.; 2];
        for q in &mut xs_scale {
            let Some(p) = entries.next() else {
                return Err(serde::de::Error::custom(
                    ParseErr::NumEntries(xs_scale_str, 2)
                ));
            };
            *q = p.parse().map_err(serde::de::Error::custom)?;
        }
        if entries.next().is_some() {
            return Err(serde::de::Error::custom(
                ParseErr::NumEntries(xs_scale_str, 2)
            ));
        }
        Ok(Self(xs_scale))
    }
}

impl Serialize for XSScale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let p = self.0;
        serializer.serialize_str(
            &format!("{},{}", p[0], p[1])
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deser_xsection_neg() {
        pub const REF_XS: &str = r#"<XSection>
 <XSNeg> 687.103,0.978277 </XSNeg>
 <MaxWeightNeg> 796475 </MaxWeightNeg>
 <TotalEventsNeg> 488338734 </TotalEventsNeg>
 <AcceptedEventsNeg> 493310 </AcceptedEventsNeg>
 <FactorNeg> 803.98,1.14468 </FactorNeg>
</XSection>"#;
        let _xs: XSection = quick_xml::de::from_str(REF_XS).unwrap();
    }

    #[test]
    fn deser_xsection_pos() {
        pub const REF_XS: &str = r#"<XSection>
 <XSPos> 687.103,0.978277 </XSPos>
 <MaxWeightPos> 796475 </MaxWeightPos>
 <TotalEventsPos> 488338734 </TotalEventsPos>
 <AcceptedEventsPos> 493310 </AcceptedEventsPos>
 <FactorPos> 803.98,1.14468 </FactorPos>
</XSection>"#;
        let _xs: XSection = quick_xml::de::from_str(REF_XS).unwrap();
    }

    #[test]
    fn deser_normalization() {
        pub const REF_NORM: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Normalization name="Cm">
<!--
File generated with STRIPPER v0.1 for online data base
-->
<XSection>
 <XSNeg> 687.103,0.978277 </XSNeg>
 <MaxWeightNeg> 796475 </MaxWeightNeg>
 <TotalEventsNeg> 488338734 </TotalEventsNeg>
 <AcceptedEventsNeg> 493310 </AcceptedEventsNeg>
 <FactorNeg> 803.98,1.14468 </FactorNeg>
</XSection>
<Contribution name="Cm">
  <xsection> 687.103,0.978277</xsection>
  <rw>
    <rwentry> x1 </rwentry>
    <rwentry> x2 </rwentry>
    <rwentry> log(muR**2) </rwentry>
    <rwentry> log(muF**2) </rwentry>
  </rw>
</Contribution>
<NumberOfRejectedEvents> 0 , 100</NumberOfRejectedEvents>
</Normalization>
"#;
        let norm: Normalization = quick_xml::de::from_str(REF_NORM).unwrap();
        assert_eq!(norm.name, "Cm");
        assert_eq!(norm.contribution.name, "Cm");
        assert_eq!(norm.contribution.xsection.0, [687.103,0.978277]);
        assert_eq!(
            norm.contribution.rw.rwentry,
            ["x1", "x2", "log(muR**2)", "log(muF**2)"]
        );
        let XSection {
            xs_pos,
            max_weight_pos,
            total_events_pos,
            accepted_events_pos,
            factor_pos
        } = norm.xsection;
        assert_eq!(xs_pos, XSScale([687.103, 0.978277]));
        assert_eq!(max_weight_pos, 796475.);
        assert_eq!(accepted_events_pos, 493310);
        assert_eq!(total_events_pos, 488338734);
        assert_eq!(factor_pos, "803.98,1.14468");
    }
}
