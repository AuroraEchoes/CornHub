pub mod messages{
    pub fn get_string(query: &str) -> String {
        match query {
            "ch.create_farm.farm_exists" => String::from("You already have a farm! Use `/ch` interact with it.")
        }
    }
}