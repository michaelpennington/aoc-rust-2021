use std::str::FromStr;

use strum_macros::*;

use anyhow::anyhow;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Sub {
    pub hpos: i32,
    pub depth: i32,
}

#[derive(Debug, PartialEq, Eq, AsRefStr, EnumString, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum CommandType {
    Forward,
    Down,
    Up,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ctype, value) = s.split_once(' ').ok_or_else(|| {
            anyhow!(
                r"Command string must be in the format `[command] [value]`.
                Instead got `{s}`"
            )
        })?;
        let ctype = ctype.parse()?;
        let value = value.parse()?;
        Ok(Self { ctype, value })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Command {
    ctype: CommandType,
    value: i32,
}

impl Sub {
    pub fn command(&mut self, cmd: Command) {
        match cmd.ctype {
            CommandType::Forward => self.hpos += cmd.value,
            CommandType::Down => self.depth += cmd.value,
            CommandType::Up => self.depth -= cmd.value,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct SubV2 {
    pub hpos: i32,
    pub depth: i32,
    aim: i32,
}

impl SubV2 {
    pub fn command(&mut self, cmd: Command) {
        match cmd.ctype {
            CommandType::Forward => {
                self.hpos += cmd.value;
                self.depth += cmd.value * self.aim;
            }
            CommandType::Down => self.aim += cmd.value,
            CommandType::Up => self.aim -= cmd.value,
        }
    }
}
