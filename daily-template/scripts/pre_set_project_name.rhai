fn set_project_name(year, day) {
    let project_name = "day-" + day;
    let crate_name = "aoc" + year + "_day_" + day;
    variable::set("project-name", project_name);
    variable::set("crate_name", crate_name);
}

let year = variable::get("year");
let day = variable::get("day");
set_project_name(year, day);