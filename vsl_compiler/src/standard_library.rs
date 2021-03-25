use std::collections::HashMap;

// data types
pub const INT: i64 = 0;
pub const FLOAT: i64 = 1;
pub const STRING: i64 = 2;
pub const VOID: i64 = 3;
pub const ANY: i64 = 4;

// functions
pub const PRINT: i64 = 0;
pub const READ: i64 = 1;

pub struct SlData {
    pub sl_num: i64,
    pub types: Vec<i64>,
    pub num_types: i64,
    pub return_type: i64,
}

pub fn get_sl_data() -> HashMap<String, SlData> {
    let mut sl_data: HashMap<String, SlData> = HashMap::new();
    sl_data.insert("print".to_string(), SlData {
        sl_num: PRINT,
        types: vec![ANY],
        num_types: 1,
        return_type: VOID,
    });
    sl_data.insert("read".to_string(), SlData{
        sl_num: READ,
        types: vec![],
        num_types: 1,
        return_type: ANY,
    });
    sl_data
}