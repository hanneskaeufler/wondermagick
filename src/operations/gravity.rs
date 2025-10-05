use crate::error::MagickError;
use crate::wm_err;

pub enum Gravity {
    Center,
    North,
    South,
    East,
    West,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

impl TryFrom<&String> for Gravity {
    type Error = MagickError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "center" => Ok(Gravity::Center),
            "north" => Ok(Gravity::North),
            "south" => Ok(Gravity::South),
            "east" => Ok(Gravity::East),
            "west" => Ok(Gravity::West),
            "northeast" => Ok(Gravity::Northeast),
            "northwest" => Ok(Gravity::Northwest),
            "southeast" => Ok(Gravity::Southeast),
            "southwest" => Ok(Gravity::Southwest),
            _ => Err(wm_err!("invalid gravity argument")),
        }
    }
}
