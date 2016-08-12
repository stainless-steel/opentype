//! The script list.

use truetype::Tag;

table! {
    @position
    #[doc = "A script list."]
    pub Scripts {
        count (u16), // ScriptCount

        headers (Vec<Header>) |this, tape, _| { // ScriptRecord
            tape.take_given(this.count as usize)
        },

        records (Vec<Record>) |this, tape, position| {
            jump_take!(tape, position, this.count, i => this.headers[i].offset)
        },
    }
}

table! {
    #[doc = "A script header."]
    #[derive(Copy)]
    pub Header {
        tag    (Tag), // ScriptTag
        offset (u16), // Script
    }
}

table! {
    @position
    #[doc = "A script record."]
    pub Record {
        default_language_offset (u16), // DefaultLangSys
        language_count          (u16), // LangSysCount

        language_headers (Vec<LanguageHeader>) |this, tape, _| { // LangSysRecord
            tape.take_given(this.language_count as usize)
        },

        default_language (Option<LanguageRecord>) |this, tape, position| {
            if this.default_language_offset != 0 {
                Ok(Some(jump_take!(@unwrap tape, position, this.default_language_offset)))
            } else {
                Ok(None)
            }
        },

        language_records (Vec<LanguageRecord>) |this, tape, position| {
            jump_take!(tape, position, this.language_count, i => this.language_headers[i].offset)
        },
    }
}

table! {
    #[doc = "A language-system header."]
    pub LanguageHeader {
        tag    (Tag), // LangSysTag
        offset (u16), // LangSys
    }
}

table! {
    #[doc = "A language-system record."]
    pub LanguageRecord {
        lookup_order           (u16) = { 0 }, // LookupOrder
        required_feature_index (u16), // ReqFeatureIndex
        feature_count          (u16), // FeatureCount

        feature_indices (Vec<u16>) |this, tape| { // FeatureIndex
            tape.take_given(this.feature_count as usize)
        },
    }
}

macro_rules! implement {
    ($($tag:expr => $name:expr => $token:ident,)*) => (
        /// A script.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Script {
            $(#[doc = $name] $token,)*
        }

        impl Script {
            fn tag(&self) -> &'static str {
                use self::Script::*;
                match *self {
                    $($token => $tag,)*
                }
            }
        }

        impl Scripts {
            /// Return the record of a script if present.
            pub fn get(&self, script: Script) -> Option<&Record> {
                let tag = script.tag().as_bytes();
                for (i, header) in self.headers.iter().enumerate() {
                    if header.tag.0 == tag {
                        return Some(&self.records[i]);
                    }
                }
                None
            }
        }
    );
}

implement! {
    "adlm" => "Adlam" => Adlam,
    "ahom" => "Ahom" => Ahom,
    "hluw" => "Anatolian Hieroglyphs" => AnatolianHieroglyphs,
    "arab" => "Arabic" => Arabic,
    "armn" => "Armenian" => Armenian,
    "avst" => "Avestan" => Avestan,
    "bali" => "Balinese" => Balinese,
    "bamu" => "Bamum" => Bamum,
    "bass" => "Bassa Vah" => BassaVah,
    "batk" => "Batak" => Batak,
    "beng" => "Bengali" => Bengali,
    "bng2" => "Bengali v2" => BengaliV2,
    "bhks" => "Bhaiksuki" => Bhaiksuki,
    "bopo" => "Bopomofo" => Bopomofo,
    "brah" => "Brahmi" => Brahmi,
    "brai" => "Braille" => Braille,
    "bugi" => "Buginese" => Buginese,
    "buhd" => "Buhid" => Buhid,
    "byzm" => "Byzantine Music" => ByzantineMusic,
    "cans" => "Canadian Syllabics" => CanadianSyllabics,
    "cari" => "Carian" => Carian,
    "aghb" => "Caucasian Albanian" => CaucasianAlbanian,
    "cakm" => "Chakma" => Chakma,
    "cham" => "Cham" => Cham,
    "cher" => "Cherokee" => Cherokee,
    "hani" => "CJK Ideographic" => CJKIdeographic,
    "copt" => "Coptic" => Coptic,
    "cprt" => "Cypriot Syllabary" => CypriotSyllabary,
    "cyrl" => "Cyrillic" => Cyrillic,
    "DFLT" => "Default" => Default,
    "dsrt" => "Deseret" => Deseret,
    "deva" => "Devanagari" => Devanagari,
    "dev2" => "Devanagari v2" => DevanagariV2,
    "dupl" => "Duployan" => Duployan,
    "egyp" => "Egyptian Hieroglyphs" => EgyptianHieroglyphs,
    "elba" => "Elbasan" => Elbasan,
    "ethi" => "Ethiopic" => Ethiopic,
    "geor" => "Georgian" => Georgian,
    "glag" => "Glagolitic" => Glagolitic,
    "goth" => "Gothic" => Gothic,
    "gran" => "Grantha" => Grantha,
    "grek" => "Greek" => Greek,
    "gujr" => "Gujarati" => Gujarati,
    "gjr2" => "Gujarati v2" => GujaratiV2,
    "guru" => "Gurmukhi" => Gurmukhi,
    "gur2" => "Gurmukhi v2" => GurmukhiV2,
    "hang" => "Hangul" => Hangul,
    "jamo" => "Hangul Jamo" => HangulJamo,
    "hano" => "Hanunoo" => Hanunoo,
    "hatr" => "Hatran" => Hatran,
    "hebr" => "Hebrew" => Hebrew,
    "kana" => "Hiragana" => Hiragana,
    "armi" => "Imperial Aramaic" => ImperialAramaic,
    "phli" => "Inscriptional Pahlavi" => InscriptionalPahlavi,
    "prti" => "Inscriptional Parthian" => InscriptionalParthian,
    "java" => "Javanese" => Javanese,
    "kthi" => "Kaithi" => Kaithi,
    "knda" => "Kannada" => Kannada,
    "knd2" => "Kannada v2" => KannadaV2,
    "kana" => "Katakana" => Katakana,
    "kali" => "Kayah Li" => KayahLi,
    "khar" => "Kharosthi" => Kharosthi,
    "khmr" => "Khmer" => Khmer,
    "khoj" => "Khojki" => Khojki,
    "sind" => "Khudawadi" => Khudawadi,
    "lao " => "Lao" => Lao,
    "latn" => "Latin" => Latin,
    "lepc" => "Lepcha" => Lepcha,
    "limb" => "Limbu" => Limbu,
    "lina" => "Linear A" => LinearA,
    "linb" => "Linear B" => LinearB,
    "lisu" => "Lisu (Fraser)" => Lisu,
    "lyci" => "Lycian" => Lycian,
    "lydi" => "Lydian" => Lydian,
    "mahj" => "Mahajani" => Mahajani,
    "mlym" => "Malayalam" => Malayalam,
    "mlm2" => "Malayalam v2" => MalayalamV2,
    "mand" => "Mandaic, Mandaean" => Mandaic,
    "mani" => "Manichaean" => Manichaean,
    "marc" => "Marchen" => Marchen,
    "math" => "Mathematical Alphanumeric Symbols" => MathematicalAlphanumericSymbols,
    "mtei" => "Meitei Mayek (Meithei, Meetei)" => MeiteiMayek,
    "mend" => "Mende Kikakui" => MendeKikakui,
    "merc" => "Meroitic Cursive" => MeroiticCursive,
    "mero" => "Meroitic Hieroglyphs" => MeroiticHieroglyphs,
    "plrd" => "Miao" => Miao,
    "modi" => "Modi" => Modi,
    "mong" => "Mongolian" => Mongolian,
    "mroo" => "Mro" => Mro,
    "mult" => "Multani" => Multani,
    "musc" => "Musical Symbols" => MusicalSymbols,
    "mymr" => "Myanmar" => Myanmar,
    "mym2" => "Myanmar v2" => MyanmarV2,
    "nbat" => "Nabataean" => Nabataean,
    "newa" => "Newa" => Newa,
    "talu" => "New Tai Lue" => NewTaiLue,
    "nko " => "N’Ko" => NKo,
    "orya" => "Odia (formerly Oriya)" => Odia,
    "ory2" => "Odia v2 (formerly Oriya v2)" => OdiaV2,
    "ogam" => "Ogham" => Ogham,
    "olck" => "Ol Chiki" => OlChiki,
    "ital" => "Old Italic" => OldItalic,
    "hung" => "Old Hungarian" => OldHungarian,
    "narb" => "Old North Arabian" => OldNorthArabian,
    "perm" => "Old Permic" => OldPermic,
    "xpeo" => "Old Persian Cuneiform" => OldPersianCuneiform,
    "sarb" => "Old South Arabian" => OldSouthArabian,
    "orkh" => "Old Turkic, Orkhon Runic" => OldTurkic,
    "osge" => "Osage" => Osage,
    "osma" => "Osmanya" => Osmanya,
    "hmng" => "Pahawh Hmong" => PahawhHmong,
    "palm" => "Palmyrene" => Palmyrene,
    "pauc" => "Pau Cin Hau" => PauCinHau,
    "phag" => "Phags-pa" => Phagspa,
    "phnx" => "Phoenician" => Phoenician,
    "phlp" => "Psalter Pahlavi" => PsalterPahlavi,
    "rjng" => "Rejang" => Rejang,
    "runr" => "Runic" => Runic,
    "samr" => "Samaritan" => Samaritan,
    "saur" => "Saurashtra" => Saurashtra,
    "shrd" => "Sharada" => Sharada,
    "shaw" => "Shavian" => Shavian,
    "sidd" => "Siddham" => Siddham,
    "sgnw" => "Sign Writing" => SignWriting,
    "sinh" => "Sinhala" => Sinhala,
    "sora" => "Sora Sompeng" => SoraSompeng,
    "xsux" => "Sumero-Akkadian Cuneiform" => SumeroAkkadianCuneiform,
    "sund" => "Sundanese" => Sundanese,
    "sylo" => "Syloti Nagri" => SylotiNagri,
    "syrc" => "Syriac" => Syriac,
    "tglg" => "Tagalog" => Tagalog,
    "tagb" => "Tagbanwa" => Tagbanwa,
    "tale" => "Tai Le" => TaiLe,
    "lana" => "Tai Tham (Lanna)" => TaiTham,
    "tavt" => "Tai Viet" => TaiViet,
    "takr" => "Takri" => Takri,
    "taml" => "Tamil" => Tamil,
    "tml2" => "Tamil v2" => TamilV2,
    "tang" => "Tangut" => Tangut,
    "telu" => "Telugu" => Telugu,
    "tel2" => "Telugu v2" => TeluguV2,
    "thaa" => "Thaana" => Thaana,
    "thai" => "Thai" => Thai,
    "tibt" => "Tibetan" => Tibetan,
    "tfng" => "Tifinagh" => Tifinagh,
    "tirh" => "Tirhuta" => Tirhuta,
    "ugar" => "Ugaritic Cuneiform" => UgariticCuneiform,
    "vai " => "Vai" => Vai,
    "wara" => "Warang Citi" => WarangCiti,
    "yi  " => "Yi" => Yi,
}

macro_rules! implement {
    ($($tag:expr => $name:expr => $token:ident => $code:expr,)*) => (
        /// A language system.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Language {
            $(#[doc = $name] $token,)*
        }

        impl Language {
            fn tag(&self) -> &'static str {
                use self::Language::*;
                match *self {
                    $($token => $tag,)*
                }
            }
        }

        impl Record {
            /// Return the record of a language system if present.
            pub fn get(&self, language: Language) -> Option<&LanguageRecord> {
                let tag = language.tag().as_bytes();
                for (i, header) in self.language_headers.iter().enumerate() {
                    if header.tag.0 == tag {
                        return Some(&self.language_records[i]);
                    }
                }
                None
            }
        }
    );
}

implement! {
    "ABA " => "Abaza" => Abaza => "abq",
    "ABK " => "Abkhazian" => Abkhazian => "abk",
    "ACH " => "Acholi" => Acholi => "ach",
    "ACR " => "Achi" => Achi => "acr",
    "ADY " => "Adyghe" => Adyghe => "ady",
    "AFK " => "Afrikaans" => Afrikaans => "afr",
    "AFR " => "Afar" => Afar => "aar",
    "AGW " => "Agaw" => Agaw => "ahg",
    "AIO " => "Aiton" => Aiton => "aio",
    "AKA " => "Akan" => Akan => "aka",
    "ALS " => "Alsatian" => Alsatian => "gsw",
    "ALT " => "Altai" => Altai => "atv, alt",
    "AMH " => "Amharic" => Amharic => "amh",
    "ANG " => "Anglo-Saxon" => AngloSaxon => "ang",
    "APPH" => "Phonetic transcription, Americanist" => AmericanistPhoneticNotation => "",
    "ARA " => "Arabic" => Arabic => "ara",
    "ARG " => "Aragonese" => Aragonese => "arg",
    "ARI " => "Aari" => Aari => "aiw",
    "ARK " => "Rakhine" => Rakhine => "mhv, rmz, rki",
    "ASM " => "Assamese" => Assamese => "asm",
    "AST " => "Asturian" => Asturian => "ast",
    "ATH " => "Athapaskan" => Athapaskan => "apk, apj, apl, apm, apw, nav, bea, sek, bcr, caf, \
                                             crx, clc, gwi, haa, chp, dgr, scs, xsl, srs, ing, \
                                             hoi, koy, hup, ktw, mvb, wlk, coq, ctc, gce, tol, \
                                             tuu, kkz, tgx, tht, aht, tfn, taa, tau, tcb, kuu, \
                                             tce, ttm, txc",
    "AVR " => "Avar" => Avar => "ava",
    "AWA " => "Awadhi" => Awadhi => "awa",
    "AYM " => "Aymara" => Aymara => "aym",
    "AZB " => "Torki" => Torki => "azb",
    "AZE " => "Azerbaijani" => Azerbaijani => "aze",
    "BAD " => "Badaga" => Badaga => "bfq",
    "BAD0" => "Banda" => Banda => "bad",
    "BAG " => "Baghelkhandi" => Baghelkhandi => "bfy",
    "BAL " => "Balkar" => Balkar => "krc",
    "BAN " => "Balinese" => Balinese => "ban",
    "BAR " => "Bavarian" => Bavarian => "bar",
    "BAU " => "Baulé" => Baule => "bci",
    "BBC " => "Batak Toba" => BatakToba => "bbc",
    "BBR " => "Berber" => Berber => "",
    "BCH " => "Bench" => Bench => "bcq",
    "BCR " => "Bible Cree" => BibleCree => "",
    "BDY " => "Bandjalang" => Bandjalang => "bdy",
    "BEL " => "Belarussian" => Belarussian => "bel",
    "BEM " => "Bemba" => Bemba => "bem",
    "BEN " => "Bengali" => Bengali => "ben",
    "BGC " => "Haryanvi" => Haryanvi => "bgc",
    "BGQ " => "Bagri" => Bagri => "bgq",
    "BGR " => "Bulgarian" => Bulgarian => "bul",
    "BHI " => "Bhili" => Bhili => "bhi, bhb",
    "BHO " => "Bhojpuri" => Bhojpuri => "bho",
    "BIK " => "Bikol" => Bikol => "bik, bhk, bcl, bto, cts, bln",
    "BIL " => "Bilen" => Bilen => "byn",
    "BIS " => "Bislama" => Bislama => "bis",
    "BJJ " => "Kanauji" => Kanauji => "bjj",
    "BKF " => "Blackfoot" => Blackfoot => "bla",
    "BLI " => "Baluchi" => Baluchi => "bal",
    "BLK " => "Pa’o Karen" => PaoKaren => "blk",
    "BLN " => "Balante" => Balante => "bjt, ble",
    "BLT " => "Balti" => Balti => "bft",
    "BMB " => "Bambara (Bamanankan)" => Bambara => "bam",
    "BML " => "Bamileke" => Bamileke => "",
    "BOS " => "Bosnian" => Bosnian => "bos",
    "BPY " => "Bishnupriya Manipuri" => BishnupriyaManipuri => "bpy",
    "BRE " => "Breton" => Breton => "bre",
    "BRH " => "Brahui" => Brahui => "brh",
    "BRI " => "Braj Bhasha" => BrajBhasha => "bra",
    "BRM " => "Burmese" => Burmese => "mya",
    "BRX " => "Bodo" => Bodo => "brx",
    "BSH " => "Bashkir" => Bashkir => "bak",
    "BSK " => "Burushaski" => Burushaski => "bsk",
    "BTI " => "Beti" => Beti => "btb",
    "BTS " => "Batak Simalungun" => BatakSimalungun => "bts",
    "BUG " => "Bugis" => Bugis => "bug",
    "BYV " => "Medumba" => Medumba => "byv",
    "CAK " => "Kaqchikel" => Kaqchikel => "cak",
    "CAT " => "Catalan" => Catalan => "cat",
    "CBK " => "Zamboanga Chavacano" => ZamboangaChavacano => "cbk",
    "CEB " => "Cebuano" => Cebuano => "ceb",
    "CHE " => "Chechen" => Chechen => "che",
    "CHG " => "Chaha Gurage" => ChahaGurage => "sgw",
    "CHH " => "Chattisgarhi" => Chattisgarhi => "hne",
    "CHI " => "Chichewa (Chewa, Nyanja)" => Chichewa => "nya",
    "CHK " => "Chukchi" => Chukchi => "ckt",
    "CHK0" => "Chuukese" => Chuukese => "chk",
    "CHO " => "Choctaw" => Choctaw => "cho",
    "CHP " => "Chipewyan" => Chipewyan => "chp",
    "CHR " => "Cherokee" => Cherokee => "chr",
    "CHA " => "Chamorro" => Chamorro => "cha",
    "CHU " => "Chuvash" => Chuvash => "chv",
    "CHY " => "Cheyenne" => Cheyenne => "chy",
    "CGG " => "Chiga" => Chiga => "cgg",
    "CMR " => "Comorian" => Comorian => "swb, wlc, wni, zdj",
    "COP " => "Coptic" => Coptic => "cop",
    "COR " => "Cornish" => Cornish => "cor",
    "COS " => "Corsican" => Corsican => "cos",
    "CPP " => "Creoles" => Creoles => "cpp",
    "CRE " => "Cree" => Cree => "cre",
    "CRR " => "Carrier" => Carrier => "crx, caf",
    "CRT " => "Crimean Tatar" => CrimeanTatar => "crh",
    "CSB " => "Kashubian" => Kashubian => "csb",
    "CSL " => "Church Slavonic" => ChurchSlavonic => "chu",
    "CSY " => "Czech" => Czech => "ces",
    "CTG " => "Chittagonian" => Chittagonian => "ctg",
    "CUK " => "San Blas Kuna" => SanBlasKuna => "cuk",
    "DAN " => "Danish" => Danish => "dan",
    "DAR " => "Dargwa" => Dargwa => "dar",
    "DAX " => "Dayi" => Dayi => "dax",
    "DCR " => "Woods Cree" => WoodsCree => "cwd",
    "DEU " => "German" => German => "deu",
    "DGO " => "Dogri" => Dogri => "dgo",
    "DGR " => "Dogri" => DogriMacrolanguage => "doi",
    "DHG " => "Dhangu" => Dhangu => "dhg",
    "DHV " => "Divehi (Dhivehi, Maldivian)" => DivehiDeprecated => "div",
    "DIQ " => "Dimli" => Dimli => "diq",
    "DIV " => "Divehi (Dhivehi, Maldivian)" => Divehi => "div",
    "DJR " => "Zarma" => Zarma => "dje",
    "DJR0" => "Djambarrpuyngu" => Djambarrpuyngu => "djr",
    "DNG " => "Dangme" => Dangme => "ada",
    "DNJ " => "Dan" => Dan => "dnj",
    "DNK " => "Dinka" => Dinka => "din",
    "DRI " => "Dari" => Dari => "prs",
    "DUJ " => "Dhuwal" => Dhuwal => "duj",
    "DUN " => "Dungan" => Dungan => "dng",
    "DZN " => "Dzongkha" => Dzongkha => "dzo",
    "EBI " => "Ebira" => Ebira => "igb",
    "ECR " => "Eastern Cree" => EasternCree => "crj, crl",
    "EDO " => "Edo" => Edo => "bin",
    "EFI " => "Efik" => Efik => "efi",
    "ELL " => "Greek" => Greek => "ell",
    "EMK " => "Eastern Maninkakan" => EasternManinkakan => "emk",
    "ENG " => "English" => English => "eng",
    "ERZ " => "Erzya" => Erzya => "myv",
    "ESP " => "Spanish" => Spanish => "spa",
    "ESU " => "Central Yupik" => CentralYupik => "esu",
    "ETI " => "Estonian" => Estonian => "est",
    "EUQ " => "Basque" => Basque => "eus",
    "EVK " => "Evenki" => Evenki => "evn",
    "EVN " => "Even" => Even => "eve",
    "EWE " => "Ewe" => Ewe => "ewe",
    "FAN " => "French Antillean" => FrenchAntillean => "acf",
    "FAN0" => "Fang" => Fang => "fan",
    "FAR " => "Persian" => Persian => "fas",
    "FAT " => "Fanti" => Fanti => "fat",
    "FIN " => "Finnish" => Finnish => "fin",
    "FJI " => "Fijian" => Fijian => "fij",
    "FLE " => "Dutch (Flemish)" => DutchFlemish => "vls",
    "FMP " => "Fe’fe’" => Fefe => "fmp",
    "FNE " => "Forest Nenets" => ForestNenets => "enf",
    "FON " => "Fon" => Fon => "fon",
    "FOS " => "Faroese" => Faroese => "fao",
    "FRA " => "French" => French => "fra",
    "FRC " => "Cajun French" => CajunFrench => "frc",
    "FRI " => "Frisian" => Frisian => "fry",
    "FRL " => "Friulian" => Friulian => "fur",
    "FRP " => "Arpitan" => Arpitan => "frp",
    "FTA " => "Futa" => Futa => "fuf",
    "FUL " => "Fulah" => Fulah => "ful",
    "FUV " => "Nigerian Fulfulde" => NigerianFulfulde => "fuv",
    "GAD " => "Ga" => Ga => "gaa",
    "GAE " => "Scottish Gaelic (Gaelic)" => ScottishGaelic => "gla",
    "GAG " => "Gagauz" => Gagauz => "gag",
    "GAL " => "Galician" => Galician => "glg",
    "GAR " => "Garshuni" => Garshuni => "",
    "GAW " => "Garhwali" => Garhwali => "gbm",
    "GEZ " => "Ge’ez" => Geez => "gez",
    "GIH " => "Githabul" => Githabul => "gih",
    "GIL " => "Gilyak" => Gilyak => "niv",
    "GIL0" => "Kiribati (Gilbertese)" => Kiribati => "gil",
    "GKP " => "Kpelle (Guinea)" => KpelleGuinea => "gkp",
    "GLK " => "Gilaki" => Gilaki => "glk",
    "GMZ " => "Gumuz" => Gumuz => "guk",
    "GNN " => "Gumatj" => Gumatj => "gnn",
    "GOG " => "Gogo" => Gogo => "gog",
    "GON " => "Gondi" => Gondi => "gon",
    "GRN " => "Greenlandic" => Greenlandic => "kal",
    "GRO " => "Garo" => Garo => "grt",
    "GUA " => "Guarani" => Guarani => "grn",
    "GUC " => "Wayuu" => Wayuu => "guc",
    "GUF " => "Gupapuyngu" => Gupapuyngu => "guf",
    "GUJ " => "Gujarati" => Gujarati => "guj",
    "GUZ " => "Gusii" => Gusii => "guz",
    "HAI " => "Haitian (Haitian Creole)" => Haitian => "hat",
    "HAL " => "Halam" => Halam => "flm",
    "HAR " => "Harauti" => Harauti => "hoj",
    "HAU " => "Hausa" => Hausa => "hau",
    "HAW " => "Hawaiian" => Hawaiian => "haw",
    "HAY " => "Haya" => Haya => "hay",
    "HAZ " => "Hazaragi" => Hazaragi => "haz",
    "HBN " => "Hammer-Banna" => HammerBanna => "amf",
    "HER " => "Herero" => Herero => "her",
    "HIL " => "Hiligaynon" => Hiligaynon => "hil",
    "HIN " => "Hindi" => Hindi => "hin",
    "HMA " => "High Mari" => HighMari => "mrj",
    "HMN " => "Hmong" => Hmong => "hmn",
    "HMO " => "Hiri Motu" => HiriMotu => "hmo",
    "HND " => "Hindko" => Hindko => "hno, hnd",
    "HO  " => "Ho" => Ho => "hoc",
    "HRI " => "Harari" => Harari => "har",
    "HRV " => "Croatian" => Croatian => "hrv",
    "HUN " => "Hungarian" => Hungarian => "hun",
    "HYE " => "Armenian" => Armenian => "hye",
    "HYE0" => "Armenian East" => ArmenianEast => "hye",
    "IBA " => "Iban" => Iban => "iba",
    "IBB " => "Ibibio" => Ibibio => "ibb",
    "IBO " => "Igbo" => Igbo => "ibo",
    "IJO " => "Ijo languages" => Ijolanguages => "ijc",
    "IDO " => "Ido" => Ido => "ido",
    "ILE " => "Interlingue" => Interlingue => "ile",
    "ILO " => "Ilokano" => Ilokano => "ilo",
    "INA " => "Interlingua" => Interlingua => "ina",
    "IND " => "Indonesian" => Indonesian => "ind",
    "ING " => "Ingush" => Ingush => "inh",
    "INU " => "Inuktitut" => Inuktitut => "iku",
    "IPK " => "Inupiat" => Inupiat => "ipk",
    "IPPH" => "Phonetic transcription, IPA" => InternationalPhoneticAlphabet => "",
    "IRI " => "Irish" => Irish => "gle",
    "IRT " => "Irish Traditional" => IrishTraditional => "gle",
    "ISL " => "Icelandic" => Icelandic => "isl",
    "ISM " => "Inari Sami" => InariSami => "smn",
    "ITA " => "Italian" => Italian => "ita",
    "IWR " => "Hebrew" => Hebrew => "heb",
    "JAM " => "Jamaican Creole" => JamaicanCreole => "jam",
    "JAN " => "Japanese" => Japanese => "jpn",
    "JAV " => "Javanese" => Javanese => "jav",
    "JBO " => "Lojban" => Lojban => "jbo",
    "JCT " => "Krymchak" => Krymchak => "jct",
    "JII " => "Yiddish" => Yiddish => "yid",
    "JUD " => "Ladino" => Ladino => "lad",
    "JUL " => "Jula" => Jula => "dyu",
    "KAB " => "Kabardian" => Kabardian => "kbd",
    "KAB0" => "Kabyle" => Kabyle => "kab",
    "KAC " => "Kachchi" => Kachchi => "kfr",
    "KAL " => "Kalenjin" => Kalenjin => "kln",
    "KAN " => "Kannada" => Kannada => "kan",
    "KAR " => "Karachay" => Karachay => "krc",
    "KAT " => "Georgian" => Georgian => "kat",
    "KAZ " => "Kazakh" => Kazakh => "kaz",
    "KDE " => "Makonde" => Makonde => "kde",
    "KEA " => "Kabuverdianu (Crioulo)" => Kabuverdianu => "kea",
    "KEB " => "Kebena" => Kebena => "ktb",
    "KEK " => "Kekchi" => Kekchi => "kek",
    "KGE " => "Khutsuri Georgian" => KhutsuriGeorgian => "kat",
    "KHA " => "Khakass" => Khakass => "kjh",
    "KHK " => "Khanty-Kazim" => KhantyKazim => "kca",
    "KHM " => "Khmer" => Khmer => "khm",
    "KHS " => "Khanty-Shurishkar" => KhantyShurishkar => "kca",
    "KHT " => "Khamti Shan" => KhamtiShan => "kht",
    "KHV " => "Khanty-Vakhi" => KhantyVakhi => "kca",
    "KHW " => "Khowar" => Khowar => "khw",
    "KIK " => "Kikuyu (Gikuyu)" => Kikuyu => "kik",
    "KIR " => "Kirghiz (Kyrgyz)" => Kirghiz => "kir",
    "KIS " => "Kisii" => Kisii => "kqs, kss",
    "KIU " => "Kirmanjki" => Kirmanjki => "kiu",
    "KJD " => "Southern Kiwai" => SouthernKiwai => "kjd",
    "KJP " => "Eastern Pwo Karen" => EasternPwoKaren => "kjp",
    "KKN " => "Kokni" => Kokni => "kex",
    "KLM " => "Kalmyk" => Kalmyk => "xal",
    "KMB " => "Kamba" => Kamba => "kam",
    "KMN " => "Kumaoni" => Kumaoni => "kfy",
    "KMO " => "Komo" => Komo => "kmw",
    "KMS " => "Komso" => Komso => "kxc",
    "KMZ " => "Khorasani Turkic" => KhorasaniTurkic => "kmz",
    "KNR " => "Kanuri" => Kanuri => "kau",
    "KOD " => "Kodagu" => Kodagu => "kfa",
    "KOH " => "Korean Old Hangul" => KoreanOldHangul => "okm",
    "KOK " => "Konkani" => Konkani => "kok",
    "KON " => "Kikongo" => Kikongo => "ktu",
    "KOM " => "Komi" => Komi => "kom",
    "KON0" => "Kongo" => Kongo => "kon",
    "KOP " => "Komi-Permyak" => KomiPermyak => "koi",
    "KOR " => "Korean" => Korean => "kor",
    "KOS " => "Kosraean" => Kosraean => "kos",
    "KOZ " => "Komi-Zyrian" => KomiZyrian => "kpv",
    "KPL " => "Kpelle" => Kpelle => "kpe",
    "KRI " => "Krio" => Krio => "kri",
    "KRK " => "Karakalpak" => Karakalpak => "kaa",
    "KRL " => "Karelian" => Karelian => "krl",
    "KRM " => "Karaim" => Karaim => "kdr",
    "KRN " => "Karen" => Karen => "kar",
    "KRT " => "Koorete" => Koorete => "kqy",
    "KSH " => "Kashmiri" => Kashmiri => "kas",
    "KSH0" => "Ripuarian" => Ripuarian => "ksh",
    "KSI " => "Khasi" => Khasi => "kha",
    "KSM " => "Kildin Sami" => KildinSami => "sjd",
    "KSW " => "S’gaw Karen" => SgawKaren => "ksw",
    "KUA " => "Kuanyama" => Kuanyama => "kua",
    "KUI " => "Kui" => Kui => "kxu",
    "KUL " => "Kulvi" => Kulvi => "kfx",
    "KUM " => "Kumyk" => Kumyk => "kum",
    "KUR " => "Kurdish" => Kurdish => "kur",
    "KUU " => "Kurukh" => Kurukh => "kru",
    "KUY " => "Kuy" => Kuy => "kdt",
    "KYK " => "Koryak" => Koryak => "kpy",
    "KYU " => "Western Kayah" => WesternKayah => "kyu",
    "LAD " => "Ladin" => Ladin => "lld",
    "LAH " => "Lahuli" => Lahuli => "bfu",
    "LAK " => "Lak" => Lak => "lbe",
    "LAM " => "Lambani" => Lambani => "lmn",
    "LAO " => "Lao" => Lao => "lao",
    "LAT " => "Latin" => Latin => "lat",
    "LAZ " => "Laz" => Laz => "lzz",
    "LCR " => "L-Cree" => LCree => "crm",
    "LDK " => "Ladakhi" => Ladakhi => "lbj",
    "LEZ " => "Lezgi" => Lezgi => "lez",
    "LIJ " => "Ligurian" => Ligurian => "lij",
    "LIM " => "Limburgish" => Limburgish => "lim",
    "LIN " => "Lingala" => Lingala => "lin",
    "LIS " => "Lisu" => Lisu => "lis",
    "LJP " => "Lampung" => Lampung => "ljp",
    "LKI " => "Laki" => Laki => "lki",
    "LMA " => "Low Mari" => LowMari => "mhr",
    "LMB " => "Limbu" => Limbu => "lif",
    "LMO " => "Lombard" => Lombard => "lmo",
    "LMW " => "Lomwe" => Lomwe => "ngl",
    "LOM " => "Loma" => Loma => "lom",
    "LRC " => "Luri" => Luri => "lrc, luz, bqi, zum",
    "LSB " => "Lower Sorbian" => LowerSorbian => "dsb",
    "LSM " => "Lule Sami" => LuleSami => "smj",
    "LTH " => "Lithuanian" => Lithuanian => "lit",
    "LTZ " => "Luxembourgish" => Luxembourgish => "ltz",
    "LUA " => "Luba-Lulua" => LubaLulua => "lua",
    "LUB " => "Luba-Katanga" => LubaKatanga => "lub",
    "LUG " => "Ganda" => Ganda => "lug",
    "LUH " => "Luyia" => Luyia => "luy",
    "LUO " => "Luo" => Luo => "luo",
    "LVI " => "Latvian" => Latvian => "lav",
    "MAD " => "Madura" => Madura => "mad",
    "MAG " => "Magahi" => Magahi => "mag",
    "MAH " => "Marshallese" => Marshallese => "mah",
    "MAJ " => "Majang" => Majang => "mpe",
    "MAK " => "Makhuwa" => Makhuwa => "vmw",
    "MAL " => "Malayalam" => Malayalam => "mal",
    "MAM " => "Mam" => Mam => "mam",
    "MAN " => "Mansi" => Mansi => "mns",
    "MAP " => "Mapudungun" => Mapudungun => "arn",
    "MAR " => "Marathi" => Marathi => "mar",
    "MAW " => "Marwari" => Marwari => "mwr, dhd, rwr, mve, wry, mtr, swv",
    "MBN " => "Mbundu" => Mbundu => "kmb",
    "MBO " => "Mbo" => Mbo => "mbo",
    "MCH " => "Manchu" => Manchu => "mnc",
    "MCR " => "Moose Cree" => MooseCree => "crm",
    "MDE " => "Mende" => Mende => "men",
    "MDR " => "Mandar" => Mandar => "mdr",
    "MEN " => "Me’en" => Meen => "mym",
    "MER " => "Meru" => Meru => "mer",
    "MFE " => "Morisyen" => Morisyen => "mfe",
    "MIN " => "Minangkabau" => Minangkabau => "min",
    "MIZ " => "Mizo" => Mizo => "lus",
    "MKD " => "Macedonian" => Macedonian => "mkd",
    "MKR " => "Makasar" => Makasar => "mak",
    "MKW " => "Kituba" => Kituba => "mkw",
    "MLE " => "Male" => Male => "mdy",
    "MLG " => "Malagasy" => Malagasy => "mlg",
    "MLN " => "Malinke" => Malinke => "mlq",
    "MLR " => "Malayalam Reformed" => MalayalamReformed => "mal",
    "MLY " => "Malay" => Malay => "msa",
    "MND " => "Mandinka" => Mandinka => "mnk",
    "MNG " => "Mongolian" => Mongolian => "mon",
    "MNI " => "Manipuri" => Manipuri => "mni",
    "MNK " => "Maninka" => Maninka => "man, mnk, myq, mku, msc, emk, mwk, mlq",
    "MNX " => "Manx" => Manx => "glv",
    "MOH " => "Mohawk" => Mohawk => "moh",
    "MOK " => "Moksha" => Moksha => "mdf",
    "MOL " => "Moldavian" => Moldavian => "mol",
    "MON " => "Mon" => Mon => "mnw",
    "MOR " => "Moroccan" => Moroccan => "",
    "MOS " => "Mossi" => Mossi => "mos",
    "MRI " => "Maori" => Maori => "mri",
    "MTH " => "Maithili" => Maithili => "mai",
    "MTS " => "Maltese" => Maltese => "mlt",
    "MUN " => "Mundari" => Mundari => "unr",
    "MUS " => "Muscogee" => Muscogee => "mus",
    "MWL " => "Mirandese" => Mirandese => "mwl",
    "MWW " => "Hmong Daw" => HmongDaw => "mww",
    "MYN " => "Mayan" => Mayan => "myn",
    "MZN " => "Mazanderani" => Mazanderani => "mzn",
    "NAG " => "Naga-Assamese" => NagaAssamese => "nag",
    "NAH " => "Nahuatl" => Nahuatl => "nah",
    "NAN " => "Nanai" => Nanai => "gld",
    "NAP " => "Neapolitan" => Neapolitan => "nap",
    "NAS " => "Naskapi" => Naskapi => "nsk",
    "NAU " => "Nauruan" => Nauruan => "nau",
    "NAV " => "Navajo" => Navajo => "nav",
    "NCR " => "N-Cree" => NCree => "csw",
    "NDB " => "Ndebele" => Ndebele => "nbl, nde",
    "NDC " => "Ndau" => Ndau => "ndc",
    "NDG " => "Ndonga" => Ndonga => "ndo",
    "NDS " => "Low Saxon" => LowSaxon => "nds",
    "NEP " => "Nepali" => Nepali => "nep",
    "NEW " => "Newari" => Newari => "new",
    "NGA " => "Ngbaka" => Ngbaka => "nga",
    "NGR " => "Nagari" => Nagari => "",
    "NHC " => "Norway House Cree" => NorwayHouseCree => "csw",
    "NIS " => "Nisi" => Nisi => "dap",
    "NIU " => "Niuean" => Niuean => "niu",
    "NKL " => "Nyankole" => Nyankole => "nyn",
    "NKO " => "N’Ko" => NKo => "nqo",
    "NLD " => "Dutch" => Dutch => "nld",
    "NOE " => "Nimadi" => Nimadi => "noe",
    "NOG " => "Nogai" => Nogai => "nog",
    "NOR " => "Norwegian" => Norwegian => "nob",
    "NOV " => "Novial" => Novial => "nov",
    "NSM " => "Northern Sami" => NorthernSami => "sme",
    "NSO " => "Sotho, Northern" => NorthernSotho => "nso",
    "NTA " => "Northern Tai" => NorthernTai => "nod",
    "NTO " => "Esperanto" => Esperanto => "epo",
    "NYM " => "Nyamwezi" => Nyamwezi => "nym",
    "NYN " => "Norwegian Nynorsk (Nynorsk, Norwegian)" => NorwegianNynorsk => "nno",
    "NZA " => "Mbembe Tigon" => MbembeTigon => "nza",
    "OCI " => "Occitan" => Occitan => "oci",
    "OCR " => "Oji-Cree" => OjiCree => "ojs",
    "OJB " => "Ojibway" => Ojibway => "oji",
    "ORI " => "Odia (formerly Oriya)" => Odia => "ori",
    "ORO " => "Oromo" => Oromo => "orm",
    "OSS " => "Ossetian" => Ossetian => "oss",
    "PAA " => "Palestinian Aramaic" => PalestinianAramaic => "sam",
    "PAG " => "Pangasinan" => Pangasinan => "pag",
    "PAL " => "Pali" => Pali => "pli",
    "PAM " => "Pampangan" => Pampangan => "pam",
    "PAN " => "Punjabi" => Punjabi => "pan",
    "PAP " => "Palpa" => Palpa => "plp",
    "PAP0" => "Papiamentu" => Papiamentu => "pap",
    "PAS " => "Pashto" => Pashto => "pus",
    "PAU " => "Palauan" => Palauan => "pau",
    "PCC " => "Bouyei" => Bouyei => "pcc",
    "PCD " => "Picard" => Picard => "pcd",
    "PDC " => "Pennsylvania German" => PennsylvaniaGerman => "pdc",
    "PGR " => "Polytonic Greek" => PolytonicGreek => "ell",
    "PHK " => "Phake" => Phake => "phk",
    "PIH " => "Norfolk" => Norfolk => "pih",
    "PIL " => "Filipino" => Filipino => "fil",
    "PLG " => "Palaung" => Palaung => "pce, rbb, pll",
    "PLK " => "Polish" => Polish => "pol",
    "PMS " => "Piemontese" => Piemontese => "pms",
    "PNB " => "Western Panjabi" => WesternPanjabi => "pnb",
    "POH " => "Pocomchi" => Pocomchi => "poh",
    "PON " => "Pohnpeian" => Pohnpeian => "pon",
    "PRO " => "Provencal" => Provencal => "pro",
    "PTG " => "Portuguese" => Portuguese => "por",
    "PWO " => "Western Pwo Karen" => WesternPwoKaren => "pwo",
    "QIN " => "Chin" => Chin => "bgr, cnh, cnw, czt, sez, tcp, csy, ctd, flm, pck, tcz, zom, cmr, \
                                 dao, hlt, cka, cnk, mrh, mwg, cbl, cnb, csh",
    "QUC " => "K’iche’" => Kiche => "quc",
    "QUH " => "Quechua (Bolivia)" => QuechuaBolivia => "quh",
    "QUZ " => "Quechua" => Quechua => "quz",
    "QVI " => "Quechua (Ecuador)" => QuechuaEcuador => "qvi",
    "QWH " => "Quechua (Peru)" => QuechuaPeru => "qwh",
    "RAJ " => "Rajasthani" => Rajasthani => "raj",
    "RAR " => "Rarotongan" => Rarotongan => "rar",
    "RBU " => "Russian Buriat" => RussianBuriat => "bxr",
    "RCR " => "R-Cree" => RCree => "atj",
    "REJ " => "Rejang" => Rejang => "rej",
    "RIA " => "Riang" => Riang => "ria",
    "RIF " => "Tarifit" => Tarifit => "rif",
    "RIT " => "Ritarungo" => Ritarungo => "rit",
    "RKW " => "Arakwal" => Arakwal => "rkw",
    "RMS " => "Romansh" => Romansh => "roh",
    "RMY " => "Vlax Romani" => VlaxRomani => "rmy",
    "ROM " => "Romanian" => Romanian => "ron",
    "ROY " => "Romany" => Romany => "rom",
    "RSY " => "Rusyn" => Rusyn => "rue",
    "RTM " => "Rotuman" => Rotuman => "rtm",
    "RUA " => "Kinyarwanda" => Kinyarwanda => "kin",
    "RUN " => "Rundi" => Rundi => "run",
    "RUP " => "Aromanian" => Aromanian => "rup",
    "RUS " => "Russian" => Russian => "rus",
    "SAD " => "Sadri" => Sadri => "sck",
    "SAN " => "Sanskrit" => Sanskrit => "san",
    "SAS " => "Sasak" => Sasak => "sas",
    "SAT " => "Santali" => Santali => "sat",
    "SAY " => "Sayisi" => Sayisi => "chp",
    "SCN " => "Sicilian" => Sicilian => "scn",
    "SCO " => "Scots" => Scots => "sco",
    "SEK " => "Sekota" => Sekota => "xan",
    "SEL " => "Selkup" => Selkup => "sel",
    "SGA " => "Old Irish" => OldIrish => "sga",
    "SGO " => "Sango" => Sango => "sag",
    "SGS " => "Samogitian" => Samogitian => "sgs",
    "SHI " => "Tachelhit" => Tachelhit => "shi",
    "SHN " => "Shan" => Shan => "shn",
    "SIB " => "Sibe" => Sibe => "sjo",
    "SID " => "Sidamo" => Sidamo => "sid",
    "SIG " => "Silte Gurage" => SilteGurage => "xst",
    "SKS " => "Skolt Sami" => SkoltSami => "sms",
    "SKY " => "Slovak" => Slovak => "slk",
    "SCS " => "North Slavey" => NorthSlavey => "scs",
    "SLA " => "Slavey" => Slavey => "scs, xsl",
    "SLV " => "Slovenian" => Slovenian => "slv",
    "SML " => "Somali" => Somali => "som",
    "SMO " => "Samoan" => Samoan => "smo",
    "SNA " => "Sena" => Sena => "seh",
    "SNA0" => "Shona" => Shona => "sna",
    "SND " => "Sindhi" => Sindhi => "snd",
    "SNH " => "Sinhala (Sinhalese)" => SinhalaSinhalese => "sin",
    "SNK " => "Soninke" => Soninke => "snk",
    "SOG " => "Sodo Gurage" => SodoGurage => "gru",
    "SOP " => "Songe" => Songe => "sop",
    "SOT " => "Sotho, Southern" => SouthernSotho => "sot",
    "SQI " => "Albanian" => Albanian => "sqi",
    "SRB " => "Serbian" => Serbian => "srp",
    "SRD " => "Sardinian" => Sardinian => "srd",
    "SRK " => "Saraiki" => Saraiki => "skr",
    "SRR " => "Serer" => Serer => "srr",
    "SSL " => "South Slavey" => SouthSlavey => "xsl",
    "SSM " => "Southern Sami" => SouthernSami => "sma",
    "STQ " => "Saterland Frisian" => SaterlandFrisian => "stq",
    "SUK " => "Sukuma" => Sukuma => "suk",
    "SUN " => "Sundanese" => Sundanese => "sun",
    "SUR " => "Suri" => Suri => "suq",
    "SVA " => "Svan" => Svan => "sva",
    "SVE " => "Swedish" => Swedish => "swe",
    "SWA " => "Swadaya Aramaic" => SwadayaAramaic => "aii",
    "SWK " => "Swahili" => Swahili => "swa",
    "SWZ " => "Swati" => Swati => "ssw",
    "SXT " => "Sutu" => Sutu => "ngo",
    "SXU " => "Upper Saxon" => UpperSaxon => "sxu",
    "SYL " => "Sylheti" => Sylheti => "syl",
    "SYR " => "Syriac" => Syriac => "syr",
    "SZL " => "Silesian" => Silesian => "szl",
    "TAB " => "Tabasaran" => Tabasaran => "tab",
    "TAJ " => "Tajiki" => Tajiki => "tgk",
    "TAM " => "Tamil" => Tamil => "tam",
    "TAT " => "Tatar" => Tatar => "tat",
    "TCR " => "TH-Cree" => THCree => "cwd",
    "TDD " => "Dehong Dai" => DehongDai => "tdd",
    "TEL " => "Telugu" => Telugu => "tel",
    "TET " => "Tetum" => Tetum => "tet",
    "TGL " => "Tagalog" => Tagalog => "tgl",
    "TGN " => "Tongan" => Tongan => "ton",
    "TGR " => "Tigre" => Tigre => "tig",
    "TGY " => "Tigrinya" => Tigrinya => "tir",
    "THA " => "Thai" => Thai => "tha",
    "THT " => "Tahitian" => Tahitian => "tah",
    "TIB " => "Tibetan" => Tibetan => "bod",
    "TIV " => "Tiv" => Tiv => "tiv",
    "TKM " => "Turkmen" => Turkmen => "tuk",
    "TMH " => "Tamashek" => Tamashek => "tmh",
    "TMN " => "Temne" => Temne => "tem",
    "TNA " => "Tswana" => Tswana => "tsn",
    "TNE " => "Tundra Nenets" => TundraNenets => "enh",
    "TNG " => "Tonga" => Tonga => "toi",
    "TOD " => "Todo" => Todo => "xal",
    "TOD0" => "Toma" => Toma => "tod",
    "TPI " => "Tok Pisin" => TokPisin => "tpi",
    "TRK " => "Turkish" => Turkish => "tur",
    "TSG " => "Tsonga" => Tsonga => "tso",
    "TUA " => "Turoyo Aramaic" => TuroyoAramaic => "tru",
    "TUM " => "Tulu" => Tulu => "tum",
    "TUL " => "Tumbuka" => Tumbuka => "tcy",
    "TUV " => "Tuvin" => Tuvin => "tyv",
    "TVL " => "Tuvalu" => Tuvalu => "tvl",
    "TWI " => "Twi" => Twi => "aka",
    "TYZ " => "Tày" => Tay => "tyz",
    "TZM " => "Tamazight" => Tamazight => "tzm",
    "TZO " => "Tzotzil" => Tzotzil => "tzo",
    "UDM " => "Udmurt" => Udmurt => "udm",
    "UKR " => "Ukrainian" => Ukrainian => "ukr",
    "UMB " => "Umbundu" => Umbundu => "umb",
    "URD " => "Urdu" => Urdu => "urd",
    "USB " => "Upper Sorbian" => UpperSorbian => "hsb",
    "UYG " => "Uyghur" => Uyghur => "uig",
    "UZB " => "Uzbek" => Uzbek => "uzb",
    "VEC " => "Venetian" => Venetian => "vec",
    "VEN " => "Venda" => Venda => "ven",
    "VIT " => "Vietnamese" => Vietnamese => "vie",
    "VOL " => "Volapük" => Volapuk => "vol",
    "VRO " => "Võro" => Voro => "vro",
    "WA  " => "Wa" => Wa => "wbm",
    "WAG " => "Wagdi" => Wagdi => "wbr",
    "WAR " => "Waray-Waray" => WarayWaray => "war",
    "WCR " => "West-Cree" => WestCree => "crk",
    "WEL " => "Welsh" => Welsh => "cym",
    "WLN " => "Walloon" => Walloon => "wln",
    "WLF " => "Wolof" => Wolof => "wol",
    "WTM " => "Mewati" => Mewati => "wtm",
    "XBD " => "Lü" => Lu => "khb",
    "XHS " => "Xhosa" => Xhosa => "xho",
    "XJB " => "Minjangbal" => Minjangbal => "xjb",
    "XOG " => "Soga" => Soga => "xog",
    "XPE " => "Kpelle (Liberia)" => KpelleLiberia => "xpe",
    "YAK " => "Sakha" => Sakha => "sah",
    "YAO " => "Yao" => Yao => "yao",
    "YAP " => "Yapese" => Yapese => "yap",
    "YBA " => "Yoruba" => Yoruba => "yor",
    "YCR " => "Y-Cree" => YCree => "cre",
    "YIC " => "Yi Classic" => YiClassic => "",
    "YIM " => "Yi Modern" => YiModern => "iii",
    "ZEA " => "Zealandic" => Zealandic => "zea",
    "ZGH " => "Standard Morrocan Tamazigh" => StandardMorrocanTamazigh => "zgh",
    "ZHA " => "Zhuang" => Zhuang => "zha",
    "ZHH " => "Chinese, Hong Kong SAR" => Chinese => "zho",
    "ZHP " => "Chinese Phonetic" => ChinesePhonetic => "zho",
    "ZHS " => "Chinese Simplified" => ChineseSimplified => "zho",
    "ZHT " => "Chinese Traditional" => ChineseTraditional => "zho",
    "ZND " => "Zande" => Zande => "zne",
    "ZUL " => "Zulu" => Zulu => "zul",
    "ZZA " => "Zazaki" => Zazaki => "zza",
}
