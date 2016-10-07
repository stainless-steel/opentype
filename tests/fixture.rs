use std::path::PathBuf;

pub enum Fixture {
    CFF,
    TTF,
    MATH,
}

impl Fixture {
    pub fn path(&self) -> PathBuf {
        match *self {
            Fixture::CFF => "tests/fixtures/SourceSerifPro-Regular.otf",
            Fixture::TTF => "tests/fixtures/OpenSans-Italic.ttf",
            Fixture::MATH => "tests/fixtures/latinmodern-math.otf",
        }.into()
    }

    pub fn offset(&self, table: &str) -> u64 {
        match *self {
            Fixture::CFF => match table {
                "GPOS" => 60412,
                "GSUB" => 57648,
                _ => unreachable!(),
            },
            Fixture::TTF => match table {
                "GDEF" => 206348,
                _ => unreachable!(),
            },
            Fixture::MATH => match table {
                "MATH" => 689248,
                _ => unreachable!(),
            }
        }
    }
}
