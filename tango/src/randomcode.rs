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
                        "648",
                        "abd",
                        "adjective",
                        "agameof",
                        "alpha",
                        "anti",
                        "availableto",
                        "awfullyhot",
                        "awhisperof",
                        "bad",
                        "balanced",
                        "baolongji",
                        "beholdthe",
                        "better",
                        "beyondard",
                        "big",
                        "bingussed",
                        "blind",
                        "boneless",
                        "boosted",
                        "bottom",
                        "bubbly",
                        "bugged",
                        "caidou",
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
                        "dianboren",
                        "diannaoren",
                        "dianqiuji",
                        "dunpaishenti",
                        "electric",
                        "endless",
                        "epic",
                        "error",
                        "famous",
                        "feedthe",
                        "feifacundang",
                        "fistfullof",
                        "flying",
                        "free",
                        "gege",
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
                        "hikari",
                        "hot",
                        "hyper",
                        "illegalsave",
                        "im",
                        "impossible",
                        "infamous",
                        "intense",
                        "invisible",
                        "itsme",
                        "jackin",
                        "jiangkou",
                        "jinzhi",
                        "juda",
                        "kuajie",
                        "lacking",
                        "legsgo",
                        "lets",
                        "licenseto",
                        "lookup",
                        "lucky",
                        "lv100",
                        "mega",
                        "moon",
                        "murkland",
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
                        "pomabu",
                        "popped",
                        "protect",
                        "quanmianfangyu",
                        "rad",
                        "relaxed",
                        "returnof",
                        "revengeofthe",
                        "risen",
                        "risky",
                        "running",
                        "scary",
                        "shashouren",
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
                        "tiantianduizhan",
                        "tongbu",
                        "top",
                        "totallyaccurate",
                        "tuolier",
                        "unhinged",
                        "unlikely",
                        "uwu",
                        "verbing",
                        "veteran",
                        "vibing",
                        "weili",
                        "weird",
                        "whohere",
                        "winking",
                        "xinghe",
                        "yitiantiande",
                        "yoinky",
                        "zhuyi",
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
                        "bombcorn",
                        "bomboy",
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
                        "cannot",
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
                    ],
                    ends: &[
                        "6",
                        "aaaaaa",
                        "alpha",
                        "amogus",
                        "angy",
                        "applm",
                        "area",
                        "aura",
                        "banned",
                        "battle",
                        "beastmode",
                        "beedited",
                        "bianji",
                        "bimbus",
                        "bingus",
                        "binted",
                        "blessing",
                        "blubblub",
                        "bot",
                        "burrito",
                        "bushenghuole",
                        "chip",
                        "chonked",
                        "chulong",
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
                        "delete",
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
                        "fangyu",
                        "fartspin",
                        "forme",
                        "fortnite",
                        "frenzy",
                        "gauntlet",
                        "ggswp",
                        "goutoupao",
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
                        "miekun",
                        "milk",
                        "minna",
                        "moi",
                        "mojo",
                        "nebulajoy",
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
                        "ronghe",
                        "rotango",
                        "sfboy",
                        "shanchu",
                        "shmooving",
                        "shuibuzhao",
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
                        "tech",
                        "technology",
                        "tfc",
                        "thunder",
                        "tier",
                        "tiltcontrols",
                        "time",
                        "tongbu",
                        "toptier",
                        "ultrafiesta",
                        "unchained",
                        "uninstalled",
                        "unlegs",
                        "uwu",
                        "vbalink",
                        "victor",
                        "wansui",
                        "wavedash",
                        "wswalk",
                        "wuguandongxi",
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
                        "biyondaaru",
                        "bokuno",
                        "dareka",
                        "daakuchippu",
                        "haroharo",
                        "hazimete",
                        "hikari",
                        "hontouno",
                        "hoshikawa",
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
                        "sakurai",
                        "sasuga",
                        "sausage",
                        "shinkurochippu",
                        "singuru",
                        "sugee",
                        "sugoi",
                        "sugu",
                        "tasikani",
                        "tokorode",
                        "toriaezu",
                        "toripuru",
                        "toriru",
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
                        "adokore",
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
                        "meiru",
                        "meizin",
                        "mondai",
                        "neko",
                        "netto",
                        "nige",
                        "nontan",
                        "onegai",
                        "rokkuman",
                        "saito",
                        "siniakurasu",
                        "subaru",
                        "surottoin",
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
                        "wairi",
                        "wotsukainasai",
                    ],
                    ends: &[
                        "bakari",
                        "daizyoubu",
                        "dake",
                        "dearu",
                        "derito",
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
                        "kurosufushyuujon",
                        "kusa",
                        "makenai",
                        "maketa",
                        "miseteyaro",
                        "mitetekure",
                        "nai",
                        "onegai",
                        "oomaigaa",
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
                        "tsuyokunaritaindarou",
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
            )
        ]);
}

pub fn generate(lang: &unic_langid::LanguageIdentifier) -> String {
    let mut thread_rng = rand::thread_rng();

    let mut lang_code = lang.language.as_str();
    if !CHOICES.contains_key(&lang_code) {
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
