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
pub const STRING_TO_INT: i64 = 2;
pub const STRING_TO_FLOAT: i64 = 3;
pub const INT_TO_FLOAT: i64 = 4;
pub const INT_TO_STRING: i64 = 5;
pub const FLOAT_TO_INT: i64 = 6;
pub const FLOAT_TO_STRING: i64 = 7;
pub const GET_STRING_INDEX: i64 = 8;
pub const SET_STRING_INDEX: i64 = 9;
pub const GET_COPY_STRING: i64 = 10;

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
    sl_data.insert("read".to_string(), SlData {
        sl_num: READ,
        types: vec![],
        num_types: 1,
        return_type: ANY,
    });
    sl_data.insert("string_to_int".to_string(), SlData {
        sl_num: STRING_TO_INT,
        types: vec![STRING],
        num_types: 1,
        return_type: INT,
    });
    sl_data.insert("string_to_float".to_string(), SlData {
        sl_num: STRING_TO_FLOAT,
        types: vec![STRING],
        num_types: 1,
        return_type: FLOAT,
    });
    sl_data.insert("int_to_float".to_string(), SlData {
        sl_num: INT_TO_FLOAT,
        types: vec![INT],
        num_types: 1,
        return_type: FLOAT,
    });
    sl_data.insert("int_to_string".to_string(), SlData {
        sl_num: INT_TO_STRING,
        types: vec![INT],
        num_types: 1,
        return_type: STRING,
    });
    sl_data.insert("float_to_int".to_string(), SlData {
        sl_num: FLOAT_TO_INT,
        types: vec![FLOAT],
        num_types: 1,
        return_type: INT,
    });
    sl_data.insert("float_to_string".to_string(), SlData {
        sl_num: FLOAT_TO_STRING,
        types: vec![FLOAT],
        num_types: 1,
        return_type: STRING,
    });
    sl_data.insert("get_string_index".to_string(), SlData {
        sl_num: GET_STRING_INDEX,
        types: vec![STRING, INT],
        num_types: 2,
        return_type: STRING,
    });
    sl_data.insert("set_string_index".to_string(), SlData {
        sl_num: SET_STRING_INDEX,
        types: vec![STRING, INT, STRING],
        num_types: 3,
        return_type: VOID,
    });
    sl_data.insert("get_copy_string".to_string(), SlData {
        sl_num: GET_COPY_STRING,
        types: vec![STRING],
        num_types: 1,
        return_type: STRING,
    });
    sl_data
}