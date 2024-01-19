#![feature(test)]
#![feature(portable_simd)]
#![feature(iter_array_chunks)]

mod levenstein;
pub use crate::levenstein::*;

extern crate test;

#[cfg(test)]
mod tests {
    use crate::{recursive, dynamic_wasteful, recursive_ascii_case_sens, dynamic_ascii_case_sensitive, dynamic_simd_wasteful};
    use test::{Bencher, black_box};

    const BASIC_ASCII_TEST_CASES: [(&str, &str, usize); 5] = [
        ("kitten", "sitting", 3),
        ("aamu", "aamukahvi", 5),
        ("sello", "kello", 1),
        ("kasvimaa", "maa", 5),
        ("yhdys sana", "yhdyssana", 1)
    ];

    const FINNISH_ALPHABET_TEST_CASES: [(&str, &str, usize); 5] = [
        ("aita", "äiti", 2),
        ("ääää", "aaaa", 4),
        ("märkäpuku", "puku", 5),
        ("älytön", "äly", 3),
        ("vääryys", "käännös", 4)
    ];

    #[test]
    fn recursive_works() {
        let it = BASIC_ASCII_TEST_CASES.iter()
            .chain(FINNISH_ALPHABET_TEST_CASES.iter());
        for (s1, s2, ans) in it {
            assert_eq!(recursive(s1, s2), *ans);
        }
    }

    #[test]
    fn recursive_ascii_case_sens_works() {
        for (s1, s2, ans) in BASIC_ASCII_TEST_CASES {
            assert_eq!(recursive(s1, s2), ans);
        }
    }

    #[test]
    fn dynamic_wasteful_works() {
        let it = BASIC_ASCII_TEST_CASES.iter()
            .chain(FINNISH_ALPHABET_TEST_CASES.iter());
        for (s1, s2, ans) in it {
            assert_eq!(recursive(s1, s2), *ans);
        }
    }

    #[test]
    fn dynamic_ascii_case_sens_works() {
        for (s1, s2, ans) in BASIC_ASCII_TEST_CASES {
            assert_eq!(dynamic_ascii_case_sensitive(s1, s2), ans);
        }
    }

    #[test]
    fn dynamic_simd_wasteful_works() {
        for (s1, s2, ans) in BASIC_ASCII_TEST_CASES {
            assert_eq!(dynamic_simd_wasteful(s1, s2), ans);
        }
        for (s1, s2, ans) in FINNISH_ALPHABET_TEST_CASES {
            assert_eq!(dynamic_simd_wasteful(s1, s2), ans);
        }
    }

    #[bench]
    #[ignore]
    fn bench_recursive(b: &mut Bencher) {
        let s1 = "rasvakeitin";
        let s2 = "reissumies";
        b.iter(|| recursive(black_box(s1), black_box(s2)));
    }

    #[bench]
    #[ignore]
    fn bench_recursive_ascii(b: &mut Bencher) {
        // Test correctness first
        let s1 = "rasvakeitin";
        let s2 = "reissumies";
        assert_eq!(recursive(s1, s2), recursive_ascii_case_sens(s1, s2));
        b.iter(|| recursive_ascii_case_sens(black_box(s1), black_box(s2)));
    }

    #[bench]
    fn bench_dynamic_wasteful(b: &mut Bencher) {
        let s1 = "rasvakeitin";
        let s2 = "reissumies";
        assert_eq!(dynamic_wasteful(s1, s2), recursive(s1, s2));
        b.iter(|| dynamic_wasteful(black_box(s1), black_box(s2)));
    }

    #[bench]
    fn bench_dynamic_simd_wasteful(b: &mut Bencher) {
        let s1 = "rasvakeitin";
        let s2 = "reissumies";
        assert_eq!(dynamic_simd_wasteful(s1, s2), dynamic_wasteful(s1, s2));
        b.iter(|| dynamic_simd_wasteful(black_box(s1), black_box(s2)));
    }

    const BENCH_LONG: (&str, &str) = (
        "Finland (Finnish: Suomi; Swedish: Finland), officially the Republic of Finland (Finnish: Suomen tasavalta; Swedish: Republiken Finland (listen to all)),[note 2] is a Nordic country in Northern Europe. It borders Sweden to the northwest, Norway to the north, and Russia to the east, with the Gulf of Bothnia to the west and the Gulf of Finland to the south, across from Estonia. Finland covers an area of 338,145 square kilometres (130,559 sq mi)[5] with a population of 5.6 million. Helsinki is the capital and largest city. The vast majority of the population are ethnic Finns. Finnish and Swedish are the official languages, Swedish being the native language of 5.2% of the population.[12] Finland's climate varies from humid continental in the south to boreal in the north. The land cover is primarily a boreal forest biome, with more than 180,000 recorded lakes.[13] Finland was first inhabited around 9000 BC after the Last Glacial Period.[14] The Stone Age introduced several different ceramic styles and cultures. The Bronze Age and Iron Age were characterized by contacts with other cultures in Fennoscandia and the Baltic region.[15] From the late 13th century, Finland became a part of Sweden as a consequence of the Northern Crusades. In 1809, as a result of the Finnish War, Finland became part of the Russian Empire as the autonomous Grand Duchy of Finland, during which Finnish art flourished and the idea of independence began to take hold. In 1906, Finland became the first European state to grant universal suffrage, and the first in the world to give all adult citizens the right to run for public office.[16][note 3] After the 1917 Russian Revolution, Finland declared independence from Russia. In 1918, the fledgling state was divided by the Finnish Civil War. During World War II, Finland fought the Soviet Union in the Winter War and the Continuation War, and Nazi Germany in the Lapland War. It subsequently lost parts of its territory, but maintained its independence. Finland largely remained an agrarian country until the 1950s. After World War II, it rapidly industrialized and developed an advanced economy, while building an extensive welfare state based on the Nordic model; the country soon enjoyed widespread prosperity and a high per capita income.[17] During the Cold War, Finland adopted an official policy of neutrality. Finland joined the European Union in 1995, the Eurozone at its inception in 1999 and NATO in 2023. It is also a member of the United Nations, the Nordic Council, the Schengen Area, the Council of Europe, the World Trade Organization and the Organisation for Economic Co-operation and Development (OECD). Finland performs highly in metrics of national performance, including education, economic competitiveness, civil liberties, quality of life and human development.[18][19][20][21]",
        "Sweden,[f] formally the Kingdom of Sweden,[18][g] is a Nordic country located on the Scandinavian Peninsula in Northern Europe. It borders Norway to the west and north, Finland to the east, and is connected to Denmark in the southwest by a bridge-tunnel across the Oresund. At 447,425 square kilometres (172,752 sq mi), Sweden is the largest Nordic country, the third-largest country in the European Union, and the fifth-largest country in Europe. The capital and largest city is Stockholm. Sweden has a total population of 10.5 million,[13] and a low population density of 25.5 inhabitants per square kilometre (66/sq mi), with around 87% of Swedes residing in urban areas, which cover 1.5% of the entire land area, in the central and southern half of the country. Nature in Sweden is dominated by forests and many lakes, including some of the largest in Europe. Many long rivers run from the Scandes range through the landscape, primarily emptying into the northern tributaries of the Baltic Sea. It has an extensive coastline and most of the population lives near a major body of water. With the country ranging from 55N to 69N, the climate of Sweden is diverse due to the length of the country. The usual conditions are mild for the latitudes with a maritime south, continental centre and subarctic north. Snow cover is variable in the densely populated south, but reliable in higher latitudes. Furthermore, the rain shadow of the Scandes results in quite dry winters and sunny summers in much of the country. Germanic peoples have inhabited Sweden since prehistoric times, emerging into history as the Geats (Swedish: Gotar) and Swedes (Svear) and constituting the sea peoples known as the Norsemen. An independent Swedish state emerged during the early 12th century. After the Black Death in the middle of the 14th century killed about a third of the Scandinavian population,[19][20] the dominance of the Hanseatic League in Northern Europe threatened Scandinavia economically and politically. This led to the formation of the Scandinavian Kalmar Union in 1397,[21] which Sweden left in 1523. When Sweden became involved in the Thirty Years' War on the Protestant side, an expansion of its territories began, forming the Swedish Empire, which remained one of the great powers of Europe until the early 18th century. Swedish territories outside the Scandinavian Peninsula were gradually lost during the 18th and 19th centuries, ending with the annexation of present-day Finland by Russia in 1809. The last war in which Sweden was directly involved was in 1814 when Norway was militarily forced into a personal union, which peacefully dissolved in 1905. In 2014, Sweden celebrated 200 years of peace, a longer span of peacetime than even Switzerland.[22] Sweden maintained an official policy of neutrality during wartime and non-participation in military alliances during peacetime, although Sweden secretly relied on U.S. nuclear submarines during the Cold War.[23] Sweden has since 2008 joined EU battlegroups, provided intelligence to NATO[24] and since 2009 openly moved towards cooperation with NATO. In 2022, following the Russian invasion of Ukraine, Sweden announced its intent to join NATO. Sweden is a highly developed country ranked seventh in the Human Development Index,[25] it is a constitutional monarchy and a parliamentary democracy, with legislative power vested in the 349-member unicameral Riksdag. It is a unitary state, currently divided into 21 counties and 290 municipalities. Sweden maintains a Nordic social welfare system that provides universal health care and tertiary education for its citizens. It has the world's 14th highest GDP per capita and ranks very highly in quality of life, health, education, protection of civil liberties, economic competitiveness, income equality, gender equality and prosperity.[26][27] Sweden joined the European Union on 1 January 1995 but rejected Eurozone membership following a referendum. It is also a member of the United Nations, the Nordic Council, the Schengen Area, the Council of Europe, the World Trade Organization and the Organisation for Economic Co-operation and Development (OECD)."
    );

    #[bench]
    fn bench_long_dynamic_wasteful(b: &mut Bencher) {
        let (s1, s2) = BENCH_LONG;
        b.iter(|| dynamic_wasteful(black_box(s1), black_box(s2)));
    }

    #[bench]
    fn bench_long_dynamic_ascii_case_sens(b: &mut Bencher) {
        let (s1, s2) = BENCH_LONG;
        b.iter(|| dynamic_ascii_case_sensitive(black_box(s1), black_box(s2)));
    }

    #[bench]
    fn bench_long_dynamic_simd_wasteful(b: &mut Bencher) {
        let (s1, s2) = BENCH_LONG;
        b.iter(|| dynamic_simd_wasteful(black_box(s1), black_box(s2)));
    }
}