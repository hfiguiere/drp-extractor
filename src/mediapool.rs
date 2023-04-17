
use std;

pub const MEDIAPOOL_DIR: &'static str = "MediaPool";
pub const XML_FILE_NAME: &'static str = "MpFolder.xml";

pub struct MpFolder {
    pub db_id: String,
    pub folder: Option<String>,
    pub item_id: String,
}

pub struct MediaPool {
    pub root: MpFolder
}


impl MpFolder {
    pub fn from_folder<R>(reader: R) -> MpFolder
        where R: std::io::Read
    {
        
        MpFolder {}
    }
}
