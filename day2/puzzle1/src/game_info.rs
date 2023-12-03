use regex::Regex;

#[derive(Debug)]
pub struct GameInfo {
    pub id: usize,
    pub red: Vec<u16>,
    pub blue: Vec<u16>,
    pub green: Vec<u16>,
}

impl GameInfo {
    pub fn new(data: &str) -> Option<GameInfo> {
        let game_re = Regex::new(r"Game\t| [0-9]+:").unwrap();
        let mut game_id: i64 = 0;

        match game_re.captures(data) {
            Some(val) => {
                game_id = val.iter().next()
                             .unwrap()
                             .as_ref()
                             .unwrap()
                             .as_str()
                             .trim()
                             .strip_suffix(':')
                             .unwrap_or_else(|| val.iter().next().unwrap().as_ref().unwrap().as_str().trim())
                             .parse::<i64>()
                             .unwrap_or(-1);
                },
            None => (),
        }

        if game_id == -1 {
            return None;
        }

        // get the information for all R/G/B
        let red_re = Regex::new(r"[0-9]+\sred").unwrap();
        let red_res: Vec<_> = red_re.find_iter(data)
                                    .map(|digits| digits.as_str()
                                                        .split_ascii_whitespace()
                                                        .next()
                                                        .as_deref()
                                                        .unwrap()
                                                        .parse::<u16>()
                                                        .unwrap()
                                    )
                                    .collect();


        let blue_re = Regex::new(r"[0-9]+\sblue").unwrap();
        let blu_res: Vec<_> = blue_re.find_iter(data)
                                        .map(|digits| digits.as_str()
                                                            .split_ascii_whitespace()
                                                            .next()
                                                            .as_deref()
                                                            .unwrap()
                                                            .parse::<u16>()
                                                            .unwrap()
                                        )
                                        .collect();

        let green_re = Regex::new(r"[0-9]+\sgreen").unwrap();
        let gre_res: Vec<_> = green_re.find_iter(data)
                                      .map(|digits| digits.as_str()
                                                          .split_ascii_whitespace()
                                                          .next()
                                                          .as_deref()
                                                          .unwrap()
                                                          .parse::<u16>()
                                                          .unwrap()
                                    )
                                    .collect();

        // we have a valid game number and we MAY have valid
        // data, we have no guarantee that the data won't be 
        // 0s though
        Some(GameInfo { 
            id: game_id as usize, 
            red: red_res, 
            blue: blu_res, 
            green: gre_res 
        })
    }
}