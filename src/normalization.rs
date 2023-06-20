use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Normalization {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "XSection")]
    pub xsection: XSection,
    #[serde(rename = "Contribution")]
    pub contribution: Contribution,
    #[serde(rename = "NumberOfRejectedEvents")]
    pub number_of_rejected_events: String,
}

#[derive(Deserialize, Serialize)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct XSection {
    #[serde(rename = "XSNeg")]
    pub xs_neg: String,
    pub max_weight_neg: f64,
    pub total_events_neg: u64,
    pub accepted_events_neg: u64,
    pub factor_neg: String,
}

#[derive(Deserialize, Serialize)]
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Contribution {
    #[serde(rename = "@name")]
    pub name: String,
    pub xsection: String,
    pub rw: Reweight,
}

#[derive(Deserialize, Serialize)]
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Reweight {
    pub rwentry: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deser_xsection() {
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
        assert_eq!(
            norm.contribution.rw.rwentry,
            ["x1", "x2", "log(muR**2)", "log(muF**2)"]
        );
        assert_eq!(norm.xsection.max_weight_neg, 796475.);
        assert_eq!(norm.xsection.accepted_events_neg, 493310);
        assert_eq!(norm.xsection.total_events_neg, 488338734);
    }
}