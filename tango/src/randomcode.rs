use rand::Rng;

struct Choices {
    pub starts: &'static [&'static str],
    pub middles: &'static [&'static str],
    pub ends: &'static [&'static str],
}

lazy_static! {
    static ref CHOICES: std::collections::HashMap<&'static str, Choices> =
        std::collections::HashMap::from([
            (
                "en",
                Choices {
                    starts: &[
                        "adjective",
                        "agameof",
                        "alpha",
                        "anti",
                        "availableto",
                        "awfullyhot",
                        "awhisperof",
                        "bad",
                        "balanced",
                        "beholdthe",
                        "better",
                        "big",
                        "bingussed",
                        "blind",
                        "boneless",
                        "boosted",
                        "bottom",
                        "bubbly",
                        "bugged",
                        "central",
                        "chilly",
                        "chonky",
                        "cobbing",
                        "code",
                        "cold",
                        "compiled",
                        "cool",
                        "corny",
                        "cursed",
                        "dancing",
                        "dang",
                        "dark",
                        "deleted",
                        "electric",
                        "endless",
                        "epic",
                        "error",
                        "famous",
                        "feedthe",
                        "fistfullof",
                        "flying",
                        "free",
                        "ghostly",
                        "giga",
                        "goodluck",
                        "harbingerof",
                        "heavy",
                        "hello",
                        "helpme",
                        "herecomes",
                        "herecomes",
                        "higsbys",
                        "hot",
                        "hyper",
                        "im",
                        "impossible",
                        "infamous",
                        "intense",
                        "invisible",
                        "itsme",
                        "jackin",
                        "lacking",
                        "legsgo",
                        "licenseto",
                        "lookup",
                        "lucky",
                        "lv100",
                        "mega",
                        "moon",
                        "murky",
                        "nebula",
                        "net",
                        "new",
                        "nice",
                        "nicebig",
                        "no",
                        "nonstop",
                        "official",
                        "old",
                        "one",
                        "online",
                        "open",
                        "overwhelming",
                        "player",
                        "pocketfullof",
                        "poggy",
                        "popped",
                        "protect",
                        "rad",
                        "relaxed",
                        "returnof",
                        "revengeofthe",
                        "risen",
                        "risky",
                        "running",
                        "scary",
                        "shining",
                        "shrimpy",
                        "spectacular",
                        "spiffy",
                        "spooky",
                        "starforce",
                        "stolen",
                        "sus",
                        "sword",
                        "team",
                        "techno",
                        "the",
                        "thefinal",
                        "thesearchfor",
                        "thicc",
                        "thick",
                        "throwthe",
                        "top",
                        "totallyaccurate",
                        "unhinged",
                        "unlikely",
                        "uwu",
                        "verbing",
                        "veteran",
                        "vibing",
                        "weird",
                        "whohere",
                        "winking",
                        "yoinky",
                    ],
                    middles: &[
                        "airhockey",
                        "airraid",
                        "airshot",
                        "airspin",
                        "alpaca",
                        "alpha",
                        "antidmg",
                        "antinavi",
                        "antirecov",
                        "antisword",
                        "anubis",
                        "apple",
                        "aquadrgn",
                        "aquasword",
                        "areagrab",
                        "assnsword",
                        "aurahead",
                        "baby",
                        "balance",
                        "balanced",
                        "bambsword",
                        "barrier",
                        "bass",
                        "bassanly",
                        "bcc",
                        "bdt",
                        "bigbomb",
                        "bighook",
                        "billy",
                        "bingus",
                        "blackbomb",
                        "blast",
                        "blues",
                        "bogos",
                        "bomboy",
                        "bombcorn",
                        "boomer",
                        "boyfriends",
                        "browser",
                        "brs",
                        "bubble",
                        "bubbleman",
                        "bugbomb",
                        "bugfix",
                        "bugfrag",
                        "bunny",
                        "burrito",
                        "busterup",
                        "cannon",
                        "canodumb",
                        "cat",
                        "charge",
                        "chicken",
                        "chiptrader",
                        "chonk",
                        "circlegun",
                        "circles",
                        "circus",
                        "cob",
                        "coffeepot",
                        "colarmy",
                        "coldbear",
                        "coldman",
                        "colonel",
                        "colorpoint",
                        "command",
                        "content",
                        "cornshot",
                        "cowboy",
                        "crackshot",
                        "crossdivide",
                        "cucumber",
                        "damage",
                        "damnswrd",
                        "daniel",
                        "digeridoo",
                        "discord",
                        "dive",
                        "dog",
                        "dollthunder",
                        "donut",
                        "doubleshot",
                        "drcossak",
                        "drillarm",
                        "duo",
                        "dust",
                        "eguchi",
                        "eleball",
                        "elec",
                        "elecdrgb",
                        "elecpulse",
                        "elecsword",
                        "element",
                        "elemtrap",
                        "energybomb",
                        "erase",
                        "error",
                        "falzar",
                        "fan",
                        "fanfare",
                        "fastgauge",
                        "firebrn",
                        "firehit",
                        "firesword",
                        "fish",
                        "fishanly",
                        "flashbomb",
                        "friday",
                        "fullcust",
                        "geddon",
                        "golemhit",
                        "grabbanish",
                        "gregar",
                        "ground",
                        "guardian",
                        "gundels",
                        "havefun",
                        "heat",
                        "hiboomer",
                        "holypanel",
                        "hub",
                        "hubbatch",
                        "humor",
                        "ice",
                        "iceball",
                        "iceseed",
                        "imfish",
                        "iminthecode",
                        "invisible",
                        "ironshell",
                        "judge",
                        "justiceone",
                        "lan",
                        "lance",
                        "landing",
                        "lifeaura",
                        "lifesync",
                        "lilboiler",
                        "machgun",
                        "magcoil",
                        "magnum",
                        "man",
                        "meaman",
                        "megaman",
                        "melody",
                        "meteorknuckle",
                        "meteors",
                        "mettaur",
                        "mine",
                        "minibomb",
                        "monday",
                        "moloko",
                        "moon",
                        "moonblade",
                        "mrfamous",
                        "muramasa",
                        "navi",
                        "navicust",
                        "needle",
                        "neovariable",
                        "netbattle",
                        "netbattler",
                        "nightmare",
                        "noun",
                        "one",
                        "operator",
                        "pengi",
                        "permahole",
                        "pet",
                        "pirate",
                        "poutine",
                        "prinkus",
                        "proto",
                        "qbot",
                        "qforce",
                        "quaker",
                        "rabbit",
                        "recovery",
                        "reflector",
                        "riskyhoney",
                        "rock",
                        "rockcube",
                        "roll",
                        "rollinglog",
                        "routine",
                        "rush",
                        "sanctuary",
                        "sand",
                        "sandworm",
                        "sensor",
                        "sequel",
                        "shark",
                        "shield",
                        "shrubby",
                        "silence",
                        "skill",
                        "slash",
                        "slowgauge",
                        "snake",
                        "spout",
                        "spreadr",
                        "starfish",
                        "static",
                        "stepsword",
                        "stevejobs",
                        "suprvulc",
                        "tango",
                        "tankcan",
                        "tengu",
                        "thawk",
                        "thegiantfist",
                        "thunder",
                        "timpani",
                        "toiletmet",
                        "tomahawk",
                        "tornado",
                        "trainarrow",
                        "tripleshot",
                        "uninstall",
                        "uwu",
                        "vdoll",
                        "vulcan",
                        "wavearm",
                        "whitecap",
                        "widesht",
                        "widesword",
                        "win",
                        "wind",
                        "windrack",
                        "wood",
                        "wooddrgn",
                        "www",
                        "yoyo",
                        "zenny",
                        "zero",
                    ],
                    ends: &[
                        "6",
                        "aaaaaa",
                        "alpha",
                        "amogus",
                        "angy",
                        "area",
                        "aura",
                        "banned",
                        "battle",
                        "beastmode",
                        "bimbus",
                        "bingus",
                        "binted",
                        "blessing",
                        "blubblub",
                        "bot",
                        "burrito",
                        "chip",
                        "chonked",
                        "clowntown",
                        "cob",
                        "cobbers",
                        "combo",
                        "congratulations",
                        "cornfusion",
                        "cornout",
                        "crasher",
                        "damn",
                        "data",
                        "denizen",
                        "eguchiwut",
                        "endofstring",
                        "energy",
                        "error",
                        "exe",
                        "execute",
                        "experience",
                        "extra",
                        "faked",
                        "fartspin",
                        "forme",
                        "fortnite",
                        "frenzy",
                        "gauntlet",
                        "ggswp",
                        "grandprix",
                        "greatplay",
                        "hamachi",
                        "heehoo",
                        "helpimtrappedinhere",
                        "hour",
                        "huh",
                        "hype",
                        "impression",
                        "isa",
                        "isbalanced",
                        "issue",
                        "iswinning",
                        "jello",
                        "legabed",
                        "letmeout",
                        "license",
                        "lilguy",
                        "loicense",
                        "longsword",
                        "lovemegaman",
                        "man",
                        "megalegs",
                        "meme",
                        "milk",
                        "minna",
                        "moi",
                        "mojo",
                        "occurroico",
                        "omega",
                        "parttwo",
                        "party",
                        "pause",
                        "power",
                        "powerhour",
                        "progchamp",
                        "programadvance",
                        "ratioed",
                        "reg",
                        "rollback",
                        "rotango",
                        "sfboy",
                        "shmooving",
                        "sickos",
                        "slimetier",
                        "snapped",
                        "solution",
                        "sp",
                        "spam",
                        "sploinky",
                        "sprite",
                        "stevejobs",
                        "strategy",
                        "swag",
                        "swaggums",
                        "swarm",
                        "symeseus",
                        "tag",
                        "technology",
                        "tfc",
                        "thunder",
                        "tier",
                        "tiltcontrols",
                        "time",
                        "toptier",
                        "ultrafiesta",
                        "unchained",
                        "uninstalled",
                        "unlegs",
                        "uwu",
                        "vbalink",
                        "victor",
                        "wavedash",
                        "wswalk",
                        "x2",
                        "yeastmode",
                        "yeet",
                        "youareworthy",
                        "yourewinner",
                        "yum",
                    ]
                }
            ),
            (
                "ja",
                Choices {
                    starts: &[
                        "abunai",
                        "anoo",
                        "bokuno",
                        "dareka",
                        "haroharo",
                        "hazimete",
                        "hontouno",
                        "ii",
                        "intaanetto",
                        "itiban",
                        "mada",
                        "maji",
                        "makenai",
                        "masaka",
                        "meta",
                        "mettya",
                        "metyakuyta",
                        "minna",
                        "muzukasii",
                        "mou",
                        "naisu",
                        "nee",
                        "ohayou",
                        "oreno",
                        "sasuga",
                        "sausage",
                        "singuru",
                        "sugee",
                        "sugoi",
                        "sugu",
                        "tasikani",
                        "tokorode",
                        "toriaezu",
                        "toripuru",
                        "tuyoi",
                        "tuyosugiru",
                        "wagahaiha",
                        "yahari",
                        "yoi",
                        "yorosiku",
                        "zannen",
                        "zenzen",
                        "zibunno",
                        "zituwa",
                    ],
                    middles: &[
                        "arigatou",
                        "basutaa",
                        "batoru",
                        "dekao",
                        "dhuuo",
                        "doyoubi",
                        "eguti",
                        "eguze",
                        "faruzaa",
                        "furusinkuro",
                        "enzan",
                        "hontounotikara",
                        "geemu",
                        "guranpuri",
                        "gureiga",
                        "iiwake",
                        "ikkai",
                        "kasutamu",
                        "kasutamaizaa",
                        "kayoubi",
                        "kimoti",
                        "koonpaathii",
                        "koonsyotto",
                        "mainkurafuto",
                        "masutazukurasu",
                        "meizin",
                        "mondai",
                        "neko",
                        "netto",
                        "nige",
                        "nontan",
                        "onegai",
                        "rokkuman",
                        "siniakurasu",
                        "susi",
                        "sutegapanesuti",
                        "syoubu",
                        "taisen",
                        "tango",
                        "tengu",
                        "tikara",
                        "tisao",
                        "tyouzetu",
                        "waburuwiisuto",
                    ],
                    ends: &[
                        "bakari",
                        "daizyoubu",
                        "dake",
                        "dearu",
                        "desu",
                        "faito",
                        "gogogo",
                        "gozaimasu",
                        "hazimemasite",
                        "hazu",
                        "ikanai",
                        "ikenai",
                        "itadakimasu",
                        "kasutamaizu",
                        "kudasai",
                        "kure",
                        "kusa",
                        "makenai",
                        "maketa",
                        "miseteyaro",
                        "mitetekure",
                        "nai",
                        "onegai",
                        "sikatanai",
                        "simasu",
                        "simasyou",
                        "sitteruno",
                        "sugee",
                        "sugiru",
                        "sukanai",
                        "syouganai",
                        "taisen",
                        "taisensimasyou",
                        "tasukete",
                        "tenkyuu",
                        "tigau",
                        "uwaa",
                        "wakaranai",
                        "warota",
                        "yabe",
                        "yaritai",
                        "yaritakunai",
                        "yaru",
                        "yatta",
                        "ze",
                        "zyanai",
                    ]
                }
            ),
            // (
            //     "es",
            //     Choices {
            //         starts: &[
            //             "bassado",
            //             "hay",
            //             "hombre",
            //             "jugadorzero",
            //             "mi",
            //             "orgullosamente",
            //             "princessa",
            //             "reskek",
            //             "serenito",
            //             "sospechoso",
            //             "usando",
            //         ],
            //         middles: &[
            //             "alguien", "bn3", "es", "estillo", "jueves", "roca", "mandalo", "mi",
            //             "pride", "ribbita",
            //         ],
            //         ends: &[
            //             "adios",
            //             "brazil",
            //             "chiposcuro",
            //             "detras",
            //             "ejecutable",
            //             "ejecucao",
            //             "facha",
            //             "meta",
            //             "vainilla",
            //         ],
            //     }
            // )
        ]);
}

pub fn generate(lang: &unic_langid::LanguageIdentifier) -> String {
    let mut thread_rng = rand::thread_rng();

    let mut lang_code = lang.language.as_str();
    if CHOICES.contains_key(&lang_code) {
        lang_code = "en";
    }

    if thread_rng.gen_range(0..5) <= 0 {
        let mut output = [&""];
        reservoir_sampling::unweighted::core::l(
            CHOICES.keys().filter(|v| **v != lang_code),
            &mut output[..],
            &mut thread_rng,
        );
        lang_code = *output.first().unwrap();
    }

    let choices = CHOICES.get(lang_code).unwrap();
    format!(
        "{}-{}-{}",
        choices.starts[thread_rng.gen_range(0..choices.starts.len())],
        choices.middles[thread_rng.gen_range(0..choices.middles.len())],
        choices.ends[thread_rng.gen_range(0..choices.ends.len())]
    )
}
