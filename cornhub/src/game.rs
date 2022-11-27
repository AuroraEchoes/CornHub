pub mod game {
    use std::{sync::RwLock, time::Instant};

    use serenity::model::user::User;

    
    #[derive(PartialEq, Debug)]
    pub struct Farm {
        pub owner: u64,
        pub name: String,
        pub last_interaction: Instant
    }

    pub struct Shard {
        pub cached_farms: RwLock<Vec<Box<Farm>>>
    }

    pub struct Upgrade;

    // TODO: Remove me later. I am temporary
    struct Embed;

    impl Shard {

        pub fn info(&self, user: &u64) -> Option<Farm> {
            for farm_iter in self.cached_farms.read().unwrap().iter() {
                if &farm_iter.owner == user {
                    return Some(farm_iter.farm_clone());
                }
            }
            return None;
            // TODO: If farm is null, check database. Offline calc.
            // TODO: This is passing around a farm by cloning. Is there a better approach?
            
        }

        pub fn create_farm(&self, user: &User) {
            // This sets the default farm stats. Put this in it's own file?
            self.cached_farms.write().unwrap().push( Box::new(Farm { 
                owner: user.id.as_u64().to_owned(), 
                name: format!("{}'s farm", user.name), 
                last_interaction: Instant::now() 
            }))
            // TODO: send to database
        }
        
        pub fn upgrades(user: &u64) -> Vec<Upgrade> {
            /*
                TODO:
                    - Check farm upgrades
                    - List available farm upgrades
            */

            vec![Upgrade { } ]
        }

        pub fn refresh(user: &u64) {
            /*
                TODO:
                    - Check if user is cached
                    - Check time since last interaction
                    - Do a calculation
            */
        }
    }

    impl Farm {
        fn farm_clone(&self) -> Farm {
            return Farm { owner: self.owner, name: self.name.clone(), last_interaction: self.last_interaction }
        }
    }

}