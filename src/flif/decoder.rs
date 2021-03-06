use std::io::Read;
use components::header::{Header, SecondHeader};
use error::*;
use {Flif, Metadata};
use numbers::rac::Rac;

pub struct Decoder<R> {
    reader: R,
}

impl<R: Read> Decoder<R> {
    pub fn new(reader: R) -> Self {
        Decoder { reader }
    }

    pub fn decode(&mut self) -> Result<Flif> {
        // read the first header
        let main_header = Header::from_reader(&mut self.reader)?;

        // read the metadata chunks
        let (metadata, non_optional_byte) = Metadata::all_from_reader(&mut self.reader)?;

        if non_optional_byte != 0 {
            return Err(Error::UnknownRequiredMetadata(non_optional_byte));
        }

        // After this point all values are encoding using the RAC so methods should no longer take
        // the Read object directly.
        let mut rac: Rac<_> = Rac::from_reader(&mut self.reader)?;

        let second_header = SecondHeader::from_rac(&main_header, &mut rac)?;
        Ok(Flif {
            header: main_header,
            metadata,
            second_header,
            _image_data: (),
        })
    }
}
