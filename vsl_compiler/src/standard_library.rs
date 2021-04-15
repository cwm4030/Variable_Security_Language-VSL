use std::collections::HashMap;

// data types
pub const INT: i64 = 0;
pub const FLOAT: i64 = 1;
pub const STRING: i64 = 2;
pub const VOID: i64 = 3;
pub const VEC_INT: i64 = 4;
pub const VEC_FLOAT: i64 = 5;
pub const VEC_STRING: i64 = 6;
pub const ANY: i64 = 7;

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
pub const VEC_INT_NEW: i64 = 11;
pub const VEC_INT_PUSH: i64 = 12;
pub const VEC_INT_POP: i64 = 13;
pub const VEC_INT_SET_INDEX: i64 = 14;
pub const VEC_INT_GET_INDEX: i64 = 15;
pub const VEC_INT_LEN: i64 = 16;
pub const VEC_FLOAT_NEW: i64 = 17;
pub const VEC_FLOAT_PUSH: i64 = 18;
pub const VEC_FLOAT_POP: i64 = 19;
pub const VEC_FLOAT_SET_INDEX: i64 = 20;
pub const VEC_FLOAT_GET_INDEX: i64 = 21;
pub const VEC_FLOAT_LEN: i64 = 22;
pub const VEC_STRING_NEW: i64 = 23;
pub const VEC_STRING_PUSH: i64 = 24;
pub const VEC_STRING_POP: i64 = 25;
pub const VEC_STRING_SET_INDEX: i64 = 26;
pub const VEC_STRING_GET_INDEX: i64 = 27;
pub const VEC_STRING_LEN: i64 = 28;

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
    sl_data.insert("vec_int_new".to_string(), SlData {
        sl_num: VEC_INT_NEW,
        types: vec![],
        num_types: 0,
        return_type: VEC_INT,
    });
    sl_data.insert("vec_int_push".to_string(), SlData {
        sl_num: VEC_INT_PUSH,
        types: vec![VEC_INT, INT],
        num_types: 2,
        return_type: VOID,
    });
    sl_data.insert("vec_int_pop".to_string(), SlData {
        sl_num: VEC_INT_POP,
        types: vec![INT],
        num_types: 1,
        return_type: VOID,
    });
    sl_data.insert("vec_int_set_index".to_string(), SlData {
        sl_num: VEC_INT_SET_INDEX,
        types: vec![VEC_INT, INT, INT],
        num_types: 3,
        return_type: VOID,
    });
    sl_data.insert("vec_int_get_index".to_string(), SlData {
        sl_num: VEC_INT_GET_INDEX,
        types: vec![VEC_INT, INT],
        num_types: 2,
        return_type: INT,
    });
    sl_data.insert("vec_int_len".to_string(), SlData {
        sl_num: VEC_INT_LEN,
        types: vec![VEC_INT],
        num_types: 1,
        return_type: VOID,
    });
    sl_data.insert("vec_float_new".to_string(), SlData {
        sl_num: VEC_FLOAT_NEW,
        types: vec![],
        num_types: 0,
        return_type: VEC_FLOAT,
    });
    sl_data.insert("vec_float_push".to_string(), SlData {
        sl_num: VEC_FLOAT_PUSH,
        types: vec![VEC_FLOAT, FLOAT],
        num_types: 2,
        return_type: VOID,
    });
    sl_data.insert("vec_float_pop".to_string(), SlData {
        sl_num: VEC_FLOAT_POP,
        types: vec![VEC_FLOAT],
        num_types: 1,
        return_type: VOID,
    });
    sl_data.insert("vec_float_set_index".to_string(), SlData {
        sl_num: VEC_FLOAT_SET_INDEX,
        types: vec![VEC_FLOAT, INT, FLOAT],
        num_types: 3,
        return_type: VOID,
    });
    sl_data.insert("vec_float_get_index".to_string(), SlData {
        sl_num: VEC_FLOAT_GET_INDEX,
        types: vec![VEC_FLOAT, INT],
        num_types: 2,
        return_type: FLOAT,
    });
    sl_data.insert("vec_float_len".to_string(), SlData {
        sl_num: VEC_FLOAT_LEN,
        types: vec![VEC_FLOAT],
        num_types: 1,
        return_type: INT,
    });
    sl_data
}