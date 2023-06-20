use particle_id::ParticleID;
use serde::{Deserialize, Serialize, Serializer};
use serde_repr::*;
use strum::EnumString;
use thiserror::Error;

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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[serde(rename = "p")]
pub struct Particle {
    #[serde(rename = "@id")]
    pub id: Id,
    #[serde(rename = "$text")]
    pub momentum: Momentum,
}

#[derive(Deserialize, Serialize)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
#[serde(rename = "rw")]
pub struct Reweight {
    #[serde(rename = "@ch")]
    pub channel: u32,
    #[serde(rename = "$text")]
    pub reweights: Reweights,
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Reweights {
    pub x1: f64,
    pub x2: f64,
    pub log_coeff: Vec<f64>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Id {
    pub status: Status,
    pub pdg_id: ParticleID,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(EnumString)]
#[repr(u8)]
pub enum Status {
    #[strum(serialize = "0")]
    Outgoing = 0,
    #[strum(serialize = "1")]
    Incoming = 1,
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Momentum (pub [f64; 4]);

impl<'de> Deserialize<'de> for Momentum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let momentum_str = String::deserialize(deserializer)?;
        let mut entries = momentum_str.split(',');
        let mut momentum = [0.; 4];
        for i in 0..momentum.len() {
            let Some(p) = entries.next() else {
                return Err(serde::de::Error::custom(
                    ParseErr::NumEntries(momentum_str, 4)
                ));
            };
            momentum[i] = p.parse().map_err(serde::de::Error::custom)?;
        }
        if entries.next().is_some() {
            return Err(serde::de::Error::custom(
                ParseErr::NumEntries(momentum_str, 4)
            ));
        }
        Ok(Self(momentum))
    }
}
impl Serialize for Momentum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let p = self.0;
        serializer.serialize_str(
            &format!("{},{},{},{}", p[0], p[1], p[2], p[3])
        )
    }
}

impl<'de> Deserialize<'de> for Reweights {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        use ParseErr::NumEntries;
        use serde::de::Error;
        let reweights_str = String::deserialize(deserializer)?;
        let mut reweights = reweights_str.split(',');
        let Some(x1) = reweights.next() else {
            return Err(Error::custom(
                NumEntries(reweights_str, 2)
            ));
        };
        let x1 = x1.parse().map_err(Error::custom)?;
        let Some(x2) = reweights.next() else {
            return Err(Error::custom(
                NumEntries(reweights_str, 2)
            ));
        };
        let x2 = x2.parse().map_err(Error::custom)?;
        let mut log_coeff = Vec::new();
        for val in reweights {
            log_coeff.push(val.parse().map_err(Error::custom)?);
        }
        Ok(Self {
            x1,
            x2,
            log_coeff,
        })
    }
}

impl Serialize for Reweights {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut res = format!("{},{}", self.x1, self.x2);
        for log_coeff in &self.log_coeff {
            res.push(',');
            res += &log_coeff.to_string();
        }
        serializer.serialize_str(&res)
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let id_str = String::deserialize(deserializer)?;
        let mut entries = id_str.split(',');
        let Some(status) = entries.next() else {
            return Err(serde::de::Error::custom(
                ParseErr::NumEntries(id_str, 2)
            ));
        };
        let status = status.parse().map_err(
            serde::de::Error::custom
        )?;

        let Some(pdg_id) = entries.next() else {
            return Err(serde::de::Error::custom(
                ParseErr::NumEntries(id_str, 2)
            ));
        };
        let pdg_id: i32 = pdg_id.parse().map_err(
            serde::de::Error::custom
        )?;
        let pdg_id = ParticleID::new(pdg_id);

        if entries.next().is_some() {
            return Err(serde::de::Error::custom(
                ParseErr::NumEntries(id_str, 2)
            ));
        }

        Ok(Self{status, pdg_id})
    }
}
impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(
            &format!("{},{}", self.status as u8, self.pdg_id.id())
        )
    }
}

// serialize as XML
//
// we could use serde or quick_xml::se::Serializer, but we want custom spaces
pub trait WriteXML {
    type Error;

    fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<(), Self::Error>;
}

impl WriteXML for Eventrecord {
    type Error = std::io::Error;

    fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<(), Self::Error> {
        writeln!(
            writer,
            "<Eventrecord nevents=\"{}\" nsubevents=\"{}\" nreweights=\"{}\" as=\"{}\" name=\"{}\">",
            self.nevents, self.nsubevents, self.nreweights, self.alpha_s_power, self.name
        )?;
        writeln!(
            writer,
            "<!--\nRecord generated with {} {}\n-->",
            env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")
        )?;
        for event in &self.events {
            event.write(writer)?;
        }
        writer.write_all(b"</Eventrecord>\n")
    }
}

impl WriteXML for Event {
    type Error = std::io::Error;

    fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<(), Self::Error> {
        writer.write_all(b"<e>\n")?;
        for subevent in &self.subevents {
            subevent.write(writer)?;
        }
        writer.write_all(b"</e>\n")
    }
}

impl WriteXML for SubEvent {
    type Error = std::io::Error;

    fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<(), Self::Error> {
        use std::fmt::Write;

        writeln!(
            writer,
            "<se w=\"{}\" muR=\"{}\" muF=\"{}\">",
            self.weight, self.mu_r, self.mu_f
        )?;
        let mut out = String::new();
        for p in &self.particles {
            let ser = quick_xml::se::Serializer::new(&mut out);
            p.serialize(ser).unwrap();
            out.write_char('\n').unwrap();
        }
        for rw in &self.reweight {
            let ser = quick_xml::se::Serializer::new(&mut out);
            rw.serialize(ser).unwrap();
            out.write_char('\n').unwrap();
        }
        writer.write_all(out.as_bytes())?;
        writer.write_all(b"</se>\n")
    }
}


#[derive(Debug, Error)]
pub enum ParseErr {
    #[error("'{0}' is not a comma-separated list with {1} float values")]
    NumEntries(String, usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deser_subevent() {
        let txt = r#"<se w="-0.0002369763508" muR="91.16253934" muF="91.16253934">
<p id="1,21"> 5780.608219,0,0,5780.608219 </p>
<p id="1,21"> 334.3891359,0,0,-334.3891359 </p>
<p id="0,6"> 357.9061187,-58.25473457,9.621341818,-307.9843429 </p>
<p id="0,-6"> 5757.091237,58.25473457,-9.621341818,5754.203426 </p>
<rw ch="12"> 0.8893243414,0.05144448245,-0.0002369763508 </rw>
</se>"#;
        use Status::*;
        use particle_id::sm_elementary_particles::*;
        let ref_event = SubEvent {
            weight: -0.0002369763508,
            mu_r: 91.16253934,
            mu_f: 91.16253934,
            particles: vec![
                Particle{
                    id: Id{
                        status: Incoming,
                        pdg_id: gluon,
                    },
                    momentum: Momentum([5780.608219,0.,0.,5780.608219])
                },
                Particle{
                    id: Id{
                        status: Incoming,
                        pdg_id: gluon,
                    },
                    momentum: Momentum([334.3891359,0.,0.,-334.3891359])
                },
                Particle{
                    id: Id{
                        status: Outgoing,
                        pdg_id: top,
                    },
                    momentum: Momentum([357.9061187,-58.25473457,9.621341818,-307.9843429])
                },
                Particle{
                    id: Id{
                        status: Outgoing,
                        pdg_id: anti_top,
                    },
                    momentum: Momentum([5757.091237,58.25473457,-9.621341818,5754.203426])
                }
            ],
            reweight: vec![ Reweight {
                channel: 12,
                reweights: Reweights{
                    x1: 0.8893243414,
                    x2: 0.05144448245,
                    log_coeff: vec![-0.0002369763508]
                }
            }],
        };
        let event: SubEvent = quick_xml::de::from_str(txt).unwrap();
        assert_eq!(event, ref_event);
    }

    const REF_RECORD: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
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
    #[test]
    fn deser_file() {

        let record: Eventrecord = quick_xml::de::from_str(REF_RECORD).unwrap();
        assert_eq!(record.nevents, 2286);
        assert_eq!(record.nsubevents, 2286);
        assert_eq!(record.nreweights, 2286);
        assert_eq!(record.alpha_s_power, 2);
        assert_eq!(record.name, "Bm");
        assert_eq!(record.events.len(), 4);
    }

    #[test]
    fn ser_file() {

        let record: Eventrecord = quick_xml::de::from_str(REF_RECORD).unwrap();
        let mut tmp = br#"<?xml version="1.0" encoding="UTF-8"?>
"#.to_vec();
        record.write(&mut tmp).unwrap();
        let record_2: Eventrecord = quick_xml::de::from_reader(tmp.as_slice()).unwrap();
        assert_eq!(record, record_2);
    }
}
