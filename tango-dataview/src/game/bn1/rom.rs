use byteorder::ByteOrder;

use crate::{rom, text};

pub struct Offsets {
    chip_data: u32,
    chip_names_pointer: u32,
    chip_descriptions_pointer: u32,
    chip_icon_palette_pointer: u32,
    element_icon_palette_pointer: u32,
    element_icons_pointer: u32,
}

#[rustfmt::skip]
pub static AREE_00: Offsets = Offsets {
    chip_data:                      0x08007d70,
    chip_names_pointer:             0x080145f4,
    chip_descriptions_pointer:      0x08016104,
    element_icons_pointer:          0x0801a688,
    element_icon_palette_pointer:   0x08005a1c,
    chip_icon_palette_pointer:      0x08015ebc,
};

#[rustfmt::skip]
pub static AREJ_00: Offsets = Offsets {
    chip_data:                      0x08007d3c,
    chip_names_pointer:             0x08014578,
    chip_descriptions_pointer:      0x08016088,
    element_icons_pointer:          0x0801a5a4,
    element_icon_palette_pointer:   0x08005a0c,
    chip_icon_palette_pointer:      0x08015e40,
};

pub struct Assets {
    offsets: &'static Offsets,
    text_parse_options: text::ParseOptions,
    mapper: rom::MemoryMapper,
    chip_icon_palette: [image::Rgba<u8>; 16],
    element_icon_palette: [image::Rgba<u8>; 16],
}

struct Chip<'a> {
    id: usize,
    assets: &'a Assets,
}

impl<'a> Chip<'a> {
    fn raw_info(&'a self) -> [u8; 0x1c] {
        self.assets.mapper.get(self.assets.offsets.chip_data)[self.id * 0x1c..(self.id + 1) * 0x1c]
            .try_into()
            .unwrap()
    }
}

impl<'a> rom::Chip for Chip<'a> {
    fn name(&self) -> String {
        if let Ok(parts) = text::parse_entry(
            &self.assets.mapper.get(byteorder::LittleEndian::read_u32(
                &self.assets.mapper.get(self.assets.offsets.chip_names_pointer)[..4],
            )),
            self.id,
            &self.assets.text_parse_options,
        ) {
            parts
                .into_iter()
                .flat_map(|part| {
                    match part {
                        text::Part::String(s) => s,
                        _ => "".to_string(),
                    }
                    .chars()
                    .collect::<Vec<_>>()
                })
                .collect::<String>()
        } else {
            "???".to_string()
        }
    }

    fn description(&self) -> String {
        if let Ok(parts) = text::parse_entry(
            &self.assets.mapper.get(byteorder::LittleEndian::read_u32(
                &self.assets.mapper.get(self.assets.offsets.chip_descriptions_pointer)[..4],
            )),
            self.id,
            &self.assets.text_parse_options,
        ) {
            parts
                .into_iter()
                .flat_map(|part| {
                    match part {
                        text::Part::String(s) => s,
                        _ => "".to_string(),
                    }
                    .chars()
                    .collect::<Vec<_>>()
                })
                .collect::<String>()
        } else {
            "???".to_string()
        }
    }

    fn icon(&self) -> image::RgbaImage {
        let raw = self.raw_info();
        rom::apply_palette(
            rom::read_merged_tiles(
                &self
                    .assets
                    .mapper
                    .get(byteorder::LittleEndian::read_u32(&raw[0x10..0x10 + 4]))[..rom::TILE_BYTES * 4],
                2,
            )
            .unwrap(),
            &self.assets.chip_icon_palette,
        )
    }

    fn image(&self) -> image::RgbaImage {
        let raw = self.raw_info();
        rom::apply_palette(
            rom::read_merged_tiles(
                &self
                    .assets
                    .mapper
                    .get(byteorder::LittleEndian::read_u32(&raw[0x14..0x14 + 4]))[..rom::TILE_BYTES * 8 * 7],
                8,
            )
            .unwrap(),
            &rom::read_palette(
                &self
                    .assets
                    .mapper
                    .get(byteorder::LittleEndian::read_u32(&raw[0x18..0x18 + 4]))[..32],
            ),
        )
    }

    fn codes(&self) -> Vec<char> {
        let raw = self.raw_info();
        raw[0x00..0x05]
            .iter()
            .cloned()
            .filter(|code| *code != 0xff)
            .map(|code| b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"[code as usize] as char)
            .collect()
    }

    fn element(&self) -> usize {
        let raw = self.raw_info();
        raw[0x05] as usize
    }

    fn class(&self) -> rom::ChipClass {
        rom::ChipClass::Standard
    }

    fn dark(&self) -> bool {
        false
    }

    fn mb(&self) -> u8 {
        0
    }

    fn damage(&self) -> u32 {
        let raw = self.raw_info();
        byteorder::LittleEndian::read_u16(&raw[0x0c..0x0c + 2]) as u32
    }

    fn library_sort_order(&self) -> Option<usize> {
        Some(self.id)
    }
}

impl Assets {
    pub fn new(offsets: &'static Offsets, charset: Vec<String>, rom: Vec<u8>, wram: Vec<u8>) -> Self {
        let mapper = rom::MemoryMapper::new(rom, wram);
        let chip_icon_palette = rom::read_palette(
            &mapper.get(byteorder::LittleEndian::read_u32(
                &mapper.get(offsets.chip_icon_palette_pointer)[..4],
            ))[..32],
        );
        let element_icon_palette = rom::read_palette(
            &mapper.get(byteorder::LittleEndian::read_u32(
                &mapper.get(offsets.element_icon_palette_pointer)[..4],
            ))[..32],
        );

        Self {
            offsets,
            text_parse_options: text::ParseOptions {
                charset,
                extension_ops: 0xe5..=0xe6,
                eof_op: 0xe7,
                newline_op: 0xe8,
                commands: std::collections::HashMap::new(),
            },
            mapper,
            chip_icon_palette,
            element_icon_palette,
        }
    }
}

impl rom::Assets for Assets {
    fn chip<'a>(&'a self, id: usize) -> Option<Box<dyn rom::Chip + 'a>> {
        if id >= self.num_chips() {
            return None;
        }
        Some(Box::new(Chip { id, assets: self }))
    }

    fn num_chips(&self) -> usize {
        super::NUM_CHIPS
    }

    fn chips_have_mb(&self) -> bool {
        false
    }

    fn element_icon(&self, id: usize) -> Option<image::RgbaImage> {
        if id >= 5 {
            return None;
        }

        let buf = self.mapper.get(byteorder::LittleEndian::read_u32(
            &self.mapper.get(self.offsets.element_icons_pointer)[..4],
        ));
        Some(rom::apply_palette(
            rom::read_merged_tiles(&buf[id * rom::TILE_BYTES * 4..(id + 1) * rom::TILE_BYTES * 4], 2).unwrap(),
            &self.element_icon_palette,
        ))
    }
}

#[rustfmt::skip]
pub const EN_CHARSET: &[&str] = &[" ", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "[Lv.]", "[11]", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "-", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "[.]", "[×]", "[=]", "[:]", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "!", "‼", "?", "“", "„", "#", "♭", "$", "%", "&", "'", "(", ")", "~", "^", "\"", "∧", "∨", "<", ">", ",", "。", ".", "・", "/", "\\\\", "_", "「", "」", "\\[", "\\]", "[bracket1]", "[bracket2]", "⊂", "⊃", "∩", "[raindrop]", "↑", "→", "↓", "←", "∀", "α", "β", "@", "★", "♥", "♪", "℃", "♂", "♀", "＿", "｜", "￣", ":", ";", "…", "¥", "+", "×", "÷", "=", "※", "*", "○", "●", "◎", "□", "■", "◇", "◆", "△", "▲", "▽", "▼", "▶", "◀", "☛", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "�", "¼", "[infinity1]", "[infinity2]"];

#[rustfmt::skip]
pub const JA_CHARSET: &[&str] = &[" ", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "ア", "イ", "ウ", "エ", "オ", "カ", "キ", "ク", "ケ", "コ", "サ", "シ", "ス", "セ", "ソ", "タ", "チ", "ツ", "テ", "ト", "ナ", "ニ", "ヌ", "ネ", "ノ", "ハ", "ヒ", "フ", "ヘ", "ホ", "マ", "ミ", "ム", "メ", "モ", "ヤ", "ユ", "ヨ", "ラ", "リ", "ル", "レ", "ロ", "ワ", "ヰ", "ヱ", "ヲ", "ン", "ガ", "ギ", "グ", "ゲ", "ゴ", "ザ", "ジ", "ズ", "ゼ", "ゾ", "ダ", "ヂ", "ヅ", "デ", "ド", "バ", "ビ", "ブ", "ベ", "ボ", "パ", "ピ", "プ", "ペ", "ポ", "ァ", "ィ", "ゥ", "ェ", "ォ", "ッ", "ャ", "ュ", "ョ", "ヴ", "ー", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "あ", "い", "う", "え", "お", "か", "き", "く", "け", "こ", "さ", "し", "す", "せ", "そ", "た", "ち", "つ", "て", "と", "な", "に", "ぬ", "ね", "の", "は", "ひ", "ふ", "へ", "ほ", "ま", "み", "む", "め", "も", "や", "ゆ", "よ", "ら", "り", "る", "れ", "ろ", "わ", "ゐ", "ゑ", "を", "ん", "が", "ぎ", "ぐ", "げ", "ご", "ざ", "じ", "ず", "ぜ", "ぞ", "だ", "ぢ", "づ", "で", "ど", "ば", "び", "ぶ", "べ", "ぼ", "ぱ", "ぴ", "ぷ", "ぺ", "ぽ", "ぁ", "ぃ", "ぅ", "ぇ", "ぉ", "っ", "ゃ", "ゅ", "ょ", "!", "‼", "?", "“", "„", "#", "♭", "$", "%", "&", "'", "(", ")", "~", "^", "\"", "∧", "∨", "<", ">", "、", "。", ".", "・", "/", "\\\\", "_", "「", "」", "\\[", "\\]", "[bracket1]", "[bracket2]", "⊂", "⊃", "∩", "[raindrop]", "↑", "→", "↓", "←", "∀", "α", "β", "@", "★", "♥", "♪", "℃", "♂", "♀", "＿", "｜", "￣", ":", ";", "…", "¥", "+", "×", "÷", "=", "※", "*", "○", "●", "◎", "□", "■", "◇", "◆", "△", "▲", "▽", "▼", "▶", "◀", "☛", "止", "彩", "起", "父", "博", "士", "一", "二", "三", "四", "五", "六", "七", "八", "九", "十", "百", "千", "万", "億", "上", "下", "左", "右", "手", "足", "日", "目", "月", "顔", "頭", "人", "入", "出", "山", "口", "光", "電", "気", "話", "広", "雨", "名", "前", "学", "校", "保", "健", "室", "世", "界", "体", "育", "館", "信", "号", "機", "器", "大", "小", "中", "自", "分", "間", "開", "閉", "問", "聞", "門", "熱", "斗", "要", "住", "道", "行", "街", "屋", "水", "見", "家", "教", "走", "先", "生", "長", "今", "事", "点", "女", "子", "言", "会", "来", "¼", "[infinity1]", "[infinity2]", "思", "時", "円", "知", "毎", "年", "火", "朝", "計", "画", "休", "曜", "帰", "回", "外", "多", "考", "正", "死", "値", "合", "戦", "争", "秋", "原", "町", "天", "用", "金", "男", "作", "数", "方", "社", "攻", "撃", "力", "同", "武", "何", "発", "少", "度", "以", "楽", "早", "暮", "面", "組", "後", "文", "字", "本", "階", "岩", "才", "者", "立", "官", "庁", "ヶ", "連", "射", "国", "局", "耳", "土", "炎", "伊", "集", "院", "各", "科", "省", "祐", "朗", "枚", "永", "川", "花", "兄", "茶", "音", "属", "性", "持", "勝", "赤", "充", "池", "停", "丁", "舎", "地", "所", "明", "切", "急", "木", "無", "高", "駅", "店", "闘", "絵", "球", "研", "究", "香"];