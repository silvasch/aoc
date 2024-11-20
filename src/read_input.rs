#[macro_export]
macro_rules! read_input {
    ($year:literal, $day:literal) => {
        include_str!(concat!($crate::get_day_folder!($year, $day), "/input.txt"))
    };
}

#[macro_export]
macro_rules! read_example_input {
    ($year:literal, $day:literal, $part:literal) => {
        include_str!(concat!(
            $crate::get_day_folder!($year, $day),
            "/part",
            $part,
            "-example-input.txt"
        ))
    };
}

#[macro_export]
macro_rules! read_example_output {
    ($year:literal, $day:literal, $part:literal) => {
        include_str!(concat!(
            $crate::get_day_folder!($year, $day),
            "/part",
            $part,
            "-example-output.txt"
        ))
    };
}

#[macro_export]
macro_rules! get_year_folder {
    ($year:literal) => {
        concat!(location_macros::workspace_dir!(), "/inputs/year", $year)
    };
}

#[macro_export]
macro_rules! get_day_folder {
    ($year:literal, $day:literal) => {
        concat!($crate::get_year_folder!($year), "/day", $day)
    };
}
