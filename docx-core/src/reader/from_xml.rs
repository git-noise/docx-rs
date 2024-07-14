use quick_xml::Reader;

use crate::reader::ReaderError;
use std::io::{BufRead, BufReader, Read};

pub trait FromXML {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError>
    where
        Self: std::marker::Sized;
}
pub trait FromXMLQuickXml {
    /// Parses XML from any type that implements `Read`, using a `BufReader` internally.
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError>
    where
        Self: Sized,
    {
        let mut reader = Reader::from_reader(BufReader::new(reader));

        let mut buf = Vec::new();
        Self::parse_xml(&mut reader, &mut buf)
    }

    /// Actual XML parsing logic specific to each implementor, needs a reader that implements `BufRead`.
    fn parse_xml<R: BufRead>(
        reader: &mut Reader<R>,
        buf: &mut Vec<u8>,
    ) -> Result<Self, ReaderError>
    where
        Self: Sized;
}
