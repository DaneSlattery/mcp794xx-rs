extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
mod common;
use common::{destroy_mcp7940n, new_mcp7940n, Register, DEVICE_ADDRESS as DEV_ADDR};
extern crate mcp794xx;
use mcp794xx::{Error, Hours, Rtcc};

macro_rules! set_invalid_param_test {
    ($name:ident, $method:ident, $value:expr) => {
        mod $name {
            use super::*;
            for_all_ics!(cannot_set_invalid, set_invalid_test, $method, $value);
        }
    };
}

macro_rules! set_invalid_param_range_test {
    ($name:ident, $method:ident, $too_small_value:expr, $too_big_value:expr) => {
        mod $name {
            use super::*;
            for_all_ics!(too_small, set_invalid_test, $method, $too_small_value);
            for_all_ics!(too_big, set_invalid_test, $method, $too_big_value);
        }
    };
}

mod seconds {
    use super::*;
    get_param_test!(get, get_seconds, SECONDS, 12, [18]);
    set_param_test!(set, set_seconds, SECONDS, 12, [18]);
    set_invalid_param_test!(invalid, set_seconds, 60);
}

mod minutes {
    use super::*;
    get_param_test!(get, get_minutes, MINUTES, 13, [19]);
    set_param_test!(set, set_minutes, MINUTES, 13, [19]);
    set_invalid_param_test!(invalid, set_minutes, 60);
}

mod hours_24h {
    use super::*;
    get_param_test!(get, get_hours, HOURS, Hours::H24(21), [0b0010_0001]);
    set_param_test!(set, set_hours, HOURS, Hours::H24(21), [0b0010_0001]);
    set_invalid_param_test!(invalid, set_hours, Hours::H24(24));
}

mod hours_12h_am {
    use super::*;
    get_param_test!(get, get_hours, HOURS, Hours::AM(12), [0b0101_0010]);
    set_param_test!(set, set_hours, HOURS, Hours::AM(12), [0b0101_0010]);
    set_invalid_param_range_test!(invalid, set_hours, Hours::AM(0), Hours::AM(13));
}

mod hours_12h_pm {
    use super::*;
    get_param_test!(get, get_hours, HOURS, Hours::PM(12), [0b0111_0010]);
    set_param_test!(set, set_hours, HOURS, Hours::PM(12), [0b0111_0010]);
    set_invalid_param_range_test!(invalid, set_hours, Hours::PM(0), Hours::PM(13));
}

mod weekday {
    use super::*;
    get_param_test!(get, get_weekday, WEEKDAY, 5, [5]);
    set_param_test!(set, set_weekday, WEEKDAY, 7, [7]);
    set_invalid_param_range_test!(invalid, set_weekday, 0, 8);
}

mod day {
    use super::*;
    get_param_test!(get, get_day, DAY, 23, [0b0010_0011]);
    set_param_test!(set, set_day, DAY, 31, [0b0011_0001]);
    set_invalid_param_range_test!(invalid, set_day, 0, 32);
}

mod month {
    use super::*;
    get_param_test!(get, get_month, MONTH, 12, [0b0001_0010]);
    set_param_test!(set, set_month, MONTH, 9, [0b0000_1001]);
    set_invalid_param_range_test!(invalid, set_month, 0, 13);
}