pub mod movements {
    pub fn all() -> Vec<String> {
        let first_movement = "1030".to_string();
        let second_movement = "P.2003".to_string();
        let third_movement = "L121.1".to_string();
        let fourth_movement = "52010".to_string();
        let fifth_movement = "3120".to_string();
        let sixth_movement  = "3185".to_string();

        vec![
            first_movement,
            second_movement,
            third_movement,
            fourth_movement,
            fifth_movement,
            sixth_movement
        ]
    }
}

pub mod companies {
    pub fn all() -> Vec<(String, i16)> {
        let first_company = ("Panerai".to_string(), 1860i16);
        let second_company = ("Rolex".to_string(), 1905i16);
        let third_company = ("A. Lange & Sohne".to_string(), 1845i16);
        let fourth_company = ("Audemars Piguet".to_string(), 1875i16);
        let fifth_company = ("IWC Schaffhausen".to_string(), 1868i16);

        vec![
            first_company,
            second_company,
            third_company,
            fourth_company,
            fifth_company
        ]
    }
}

pub mod watches {
    pub fn all() -> Vec<(String, i16, String)> {
        // 1030 movement
        let first_watch = ("6541".to_string(), 1958i16, "Milguass".to_string());

        // P.2003
        let second_watch = (
            "PAM00335".to_string(),
            2015,
            "Luminor 1950 10 Days Black Dial Ceramic Black".to_string()
        );

        // L121.1
        let third_watch = (
            "101.021".to_string(),
            2015,
            "Lange 1".to_string()
        );

        // 5007 is the watch 03 is coloring
        // maybe a good watch to test out the waters of splitting on variation
        // 52010
        let fourth_watch = (
            "IW500703".to_string(),
            2015,
            "Portugieser Automatic".to_string()
        );

        // 3120
        let fifth_watch = (
            "15400ST.OO.1220ST.01".to_string(),
            2015,
            "Royal Oak Stainless Steel".to_string()
        );

        let sixth_watch = (
            "16710".to_string(),
            1989,
            "GMT Master II".to_string()
        );

        let seventh_watch = (
            "16570".to_string(),
            1989,
            "Explorer II".to_string()
        );

        // I am still having trouble figuring out how to store watches
        // it seems that this ref should be able to given a year
        //
        //
        // Ref. 16710
        // Production Period: 1989-2007
        // Model Name: Rolex GMT Master II
        // Caliber: 3185 (late models with 3186), 28800A/h, hacking, quickset (24-hour-hand)
        // Pressure proof to 100m/330ft
        // Bracelet: Oyster 78360 and 78790 (Oysterlock), Jubilé 62510
        // Glass: Sapphire crystal
        // Bezel: Anodized aluminum, 120 clicks

        vec![
            first_watch,
            second_watch,
            third_watch,
            fourth_watch,
            fifth_watch,
            sixth_watch,
            seventh_watch
        ]
    }
}
