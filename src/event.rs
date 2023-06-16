use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Eventrecord {
    #[serde(rename = "@nevents")]
    pub nevents: u64,
    #[serde(rename = "@nsubevents")]
    pub nsubevents: u64,
    #[serde(rename = "@nreweights")]
    pub nreweights: u64,
    #[serde(rename = "@as")]
    pub alpha_s_power: u64,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "e")]
    pub events: Vec<Event>,
}

#[derive(Deserialize, Serialize)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
#[serde(rename = "e")]
pub struct Event {
    #[serde(rename = "se")]
    pub subevents: Vec<SubEvent>,
}

#[derive(Deserialize, Serialize)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
#[serde(rename = "se")]
pub struct SubEvent {
    #[serde(rename = "@w")]
    pub weight: f64,
    #[serde(rename = "@muR")]
    pub mu_r: f64,
    #[serde(rename = "@muF")]
    pub mu_f: f64,
    #[serde(rename = "p")]
    pub particles: Vec<Particle>,
    #[serde(rename = "rw")]
    pub reweight: Vec<Reweight>
}

#[derive(Deserialize, Serialize)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
#[serde(rename = "p")]
pub struct Particle {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "$text")]
    momentum: String,
}

#[derive(Deserialize, Serialize)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
#[serde(rename = "rw")]
pub struct Reweight {
    #[serde(rename = "@ch")]
    channel: u32,
    #[serde(rename = "$text")]
    reweights: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ser_subevent() {
        let mut ev = SubEvent::default();
        ev.particles.push(Default::default());
        ev.particles.push(Default::default());
        ev.reweight.push(Default::default());
        ev.reweight.push(Default::default());
        let ev_str = quick_xml::se::to_string(&ev).unwrap();
        eprintln!("{ev_str}");
    }

    #[test]
    fn deser_subevent() {
        let txt = r#"<se w="-0.0002369763508" muR="91.16253934" muF="91.16253934">
<p id="1,21"> 5780.608219,0,0,5780.608219 </p>
<p id="1,21"> 334.3891359,0,0,-334.3891359 </p>
<p id="0,6"> 357.9061187,-58.25473457,9.621341818,-307.9843429 </p>
<p id="0,-6"> 5757.091237,58.25473457,-9.621341818,5754.203426 </p>
<rw ch="12"> 0.8893243414,0.05144448245,-0.0002369763508 </rw>
</se>"#;
        let ref_event = SubEvent {
            weight: -0.0002369763508,
            mu_r: 91.16253934,
            mu_f: 91.16253934,
            particles: vec![
                Particle{
                    id: "1,21".to_string(),
                    momentum: "5780.608219,0,0,5780.608219".to_string()
                },
                Particle{
                    id: "1,21".to_string(),
                    momentum: "334.3891359,0,0,-334.3891359".to_string()
                },
                Particle{
                    id: "0,6".to_string(),
                    momentum: "357.9061187,-58.25473457,9.621341818,-307.9843429".to_string()
                },
                Particle{
                    id: "0,-6".to_string(),
                    momentum: "5757.091237,58.25473457,-9.621341818,5754.203426".to_string()
                }
            ],
            reweight: vec![ Reweight {
                channel: 12,
                reweights: "0.8893243414,0.05144448245,-0.0002369763508".to_string()
            }],
        };
        let event: SubEvent = quick_xml::de::from_str(txt).unwrap();
        assert_eq!(event, ref_event);
    }

    #[test]
    fn deser_file() {
        let file_txt = r#"<?xml version="1.0" encoding="UTF-8"?>
<Eventrecord nevents="2286" nsubevents="2286" nreweights="2286" as="2" name="Bm">
<!--
File generated with STRIPPER v0.1 for online data base
-->
<e>
<se w="-0.0002369763508" muR="91.16253934" muF="91.16253934">
<p id="1,21"> 5780.608219,0,0,5780.608219 </p>
<p id="1,21"> 334.3891359,0,0,-334.3891359 </p>
<p id="0,6"> 357.9061187,-58.25473457,9.621341818,-307.9843429 </p>
<p id="0,-6"> 5757.091237,58.25473457,-9.621341818,5754.203426 </p>
<rw ch="12"> 0.8893243414,0.05144448245,-0.0002369763508 </rw>
</se>
</e>
<e>
<se w="-0.0004385904665" muR="517.0997809" muF="517.0997809">
<p id="1,1"> 327.0442813,0,0,327.0442813 </p>
<p id="1,-1"> 4344.52245,0,0,-4344.52245 </p>
<p id="0,6"> 3334.580936,-386.757619,943.5205498,-3170.151619 </p>
<p id="0,-6"> 1336.985795,386.757619,-943.5205498,-847.3265495 </p>
<rw ch="1"> 0.05031450481,0.6683880692,-0.0004385904665 </rw>
</se>
</e>
<e>
<se w="-2.098171554e-05" muR="1140.717994" muF="1140.717994">
<p id="1,1"> 1610.067985,0,0,1610.067985 </p>
<p id="1,-1"> 4034.720799,0,0,-4034.720799 </p>
<p id="0,6"> 3362.889308,-2194.656814,598.8951393,-2470.642493 </p>
<p id="0,-6"> 2281.899476,2194.656814,-598.8951393,45.98967904 </p>
<rw ch="1"> 0.2477027669,0.6207262768,-2.098171554e-05 </rw>
</se>
</e>
<e>
<se w="-4.834595231e-05" muR="635.8532498" muF="635.8532498">
<p id="1,1"> 1299.986308,0,0,1299.986308 </p>
<p id="1,-1"> 3291.996472,0,0,-3291.996472 </p>
<p id="0,6"> 1510.408466,-1129.902831,557.4950784,814.9210479 </p>
<p id="0,-6"> 3081.574313,1129.902831,-557.4950784,-2806.931212 </p>
<rw ch="1"> 0.1999978935,0.5064609956,-4.834595231e-05 </rw>
</se>
</e>
</Eventrecord>
"#;

        let record: Eventrecord = quick_xml::de::from_str(file_txt).unwrap();
        assert_eq!(record.nevents, 2286);
        assert_eq!(record.nsubevents, 2286);
        assert_eq!(record.nreweights, 2286);
        assert_eq!(record.alpha_s_power, 2);
        assert_eq!(record.name, "Bm");
    }

}
