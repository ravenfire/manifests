
         
            #[allow(non_upper_case_globals)]
            pub const examples__slash__games__slash__spellbinder__slash__game__dot__lock__dot__toml: &str = include_str!("../examples/games/spellbinder/game.lock.toml");
            
 
            #[allow(non_upper_case_globals)]
            pub const examples__slash__peripherals__slash__watertribe__dot__card_reader__slash__watertribe__dot__card_reader__dot__lock__dot__toml: &str = include_str!("../examples/peripherals/watertribe.card_reader/watertribe.card_reader.lock.toml");
            
 
            #[allow(non_upper_case_globals)]
            pub const examples__slash__peripherals__slash__spellbinder__dot__world_builder__slash__spellbinder__dot__world_builder__dot__lock__dot__toml: &str = include_str!("../examples/peripherals/spellbinder.world_builder/spellbinder.world_builder.lock.toml");
            
 
            #[allow(non_upper_case_globals)]
            pub const examples__slash__peripherals__slash__rf__dot__screen__slash__rf__dot__screen__dot__lock__dot__toml: &str = include_str!("../examples/peripherals/rf.screen/rf.screen.lock.toml");
            
 
            #[allow(non_upper_case_globals)]
            pub const examples__slash__peripherals__slash__rf__dot__dice_pad__slash__rf__dot__dice_pad__dot__lock__dot__toml: &str = include_str!("../examples/peripherals/rf.dice_pad/rf.dice_pad.lock.toml");
            


        pub fn load(file_name: &str) -> Option<&'static str> {
            match file_name {
            "examples/games/spellbinder/game.lock.toml" => Some(examples__slash__games__slash__spellbinder__slash__game__dot__lock__dot__toml),
"examples/peripherals/watertribe.card_reader/watertribe.card_reader.lock.toml" => Some(examples__slash__peripherals__slash__watertribe__dot__card_reader__slash__watertribe__dot__card_reader__dot__lock__dot__toml),
"examples/peripherals/spellbinder.world_builder/spellbinder.world_builder.lock.toml" => Some(examples__slash__peripherals__slash__spellbinder__dot__world_builder__slash__spellbinder__dot__world_builder__dot__lock__dot__toml),
"examples/peripherals/rf.screen/rf.screen.lock.toml" => Some(examples__slash__peripherals__slash__rf__dot__screen__slash__rf__dot__screen__dot__lock__dot__toml),
"examples/peripherals/rf.dice_pad/rf.dice_pad.lock.toml" => Some(examples__slash__peripherals__slash__rf__dot__dice_pad__slash__rf__dot__dice_pad__dot__lock__dot__toml),

            _ => None,
        }
        }