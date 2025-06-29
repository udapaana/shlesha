use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use shlesha::Shlesha;

// Test data for profiling
const SMALL_IAST: &str = "dharma";
const MEDIUM_IAST: &str = "dharma yoga bhārata saṃskṛta veda upaniṣad gītā rāmāyaṇa mahābhārata";
const LARGE_IAST: &str = "dharma yoga bhārata saṃskṛta veda upaniṣad gītā rāmāyaṇa mahābhārata purāṇa śāstra darśana āyurveda jyotiṣa vyākaraṇa chanda nirukta kalpa śikṣā smṛti śruti ācāra vicāra saṃskāra paramparā satya ahiṃsā karuṇā dayā prema śānti ānanda mokṣa nirvāṇa samādhi dhyāna prāṇāyāma āsana mantra yantra tantra";

const SMALL_ITRANS: &str = "dharma";
const MEDIUM_ITRANS: &str =
    "dharma yoga bhArata sa~nskR^ita veda upaniShad gItA rAmAyaNa mahAbhArata";
const LARGE_ITRANS: &str = "dharma yoga bhArata sa~nskR^ita veda upaniShad gItA rAmAyaNa mahAbhArata purANa shAstra darshana Ayurveda jyotiSha vyAkaraNa chanda nirukta kalpa shikShA smR^iti shruti AchAra vichAra sa~nskAra paramparA satya ahi~nsA karuNA dayA prema shAnti Ananda mokSha nirvANa samAdhi dhyAna prANAyAma Asana mantra yantra tantra";

const SMALL_SLP1: &str = "Darma";
const MEDIUM_SLP1: &str = "Darma yoga BArata saMskfta veda upanizad gItA rAmAyaNa mahABArata";
const LARGE_SLP1: &str = "Darma yoga BArata saMskfta veda upanizad gItA rAmAyaNa mahABArata purANa SAstra darSana Ayurveda jyotiza vyAkaraNa Canda nirukta kalpa SikzA smfti Sruti AcAra vicAra saMskAra paramparA satya ahiMsA karuNA dayA prema SAnti Ananda mokza nirvANa samADi DyAna prANAyAma Asana mantra yantra tantra";

const SMALL_DEVA: &str = "धर्म";
const MEDIUM_DEVA: &str = "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत";
const LARGE_DEVA: &str = "धर्म योग भारत संस्कृत वेद उपनिषद् गीता रामायण महाभारत पुराण शास्त्र दर्शन आयुर्वेद ज्योतिष व्याकरण छन्द निरुक्त कल्प शिक्षा स्मृति श्रुति आचार विचार संस्कार परम्परा सत्य अहिंसा करुणा दया प्रेम शान्ति आनन्द मोक्ष निर्वाण समाधि ध्यान प्राणायाम आसन मन्त्र यन्त्र तन्त्र";

fn bench_roman_to_indic(c: &mut Criterion) {
    let transliterator = Shlesha::new();

    let mut group = c.benchmark_group("roman_to_indic");

    // IAST to Devanagari
    for (size, text) in [
        ("small", SMALL_IAST),
        ("medium", MEDIUM_IAST),
        ("large", LARGE_IAST),
    ] {
        group.bench_with_input(
            BenchmarkId::new("iast_to_devanagari", size),
            &text,
            |b, &text| {
                b.iter(|| {
                    transliterator
                        .transliterate(text, "iast", "devanagari")
                        .unwrap()
                })
            },
        );
    }

    // ITRANS to Devanagari
    for (size, text) in [
        ("small", SMALL_ITRANS),
        ("medium", MEDIUM_ITRANS),
        ("large", LARGE_ITRANS),
    ] {
        group.bench_with_input(
            BenchmarkId::new("itrans_to_devanagari", size),
            &text,
            |b, &text| {
                b.iter(|| {
                    transliterator
                        .transliterate(text, "itrans", "devanagari")
                        .unwrap()
                })
            },
        );
    }

    // SLP1 to Devanagari
    for (size, text) in [
        ("small", SMALL_SLP1),
        ("medium", MEDIUM_SLP1),
        ("large", LARGE_SLP1),
    ] {
        group.bench_with_input(
            BenchmarkId::new("slp1_to_devanagari", size),
            &text,
            |b, &text| {
                b.iter(|| {
                    transliterator
                        .transliterate(text, "slp1", "devanagari")
                        .unwrap()
                })
            },
        );
    }

    group.finish();
}

fn bench_roman_to_roman(c: &mut Criterion) {
    let transliterator = Shlesha::new();

    let mut group = c.benchmark_group("roman_to_roman");

    // IAST to ITRANS
    for (size, text) in [
        ("small", SMALL_IAST),
        ("medium", MEDIUM_IAST),
        ("large", LARGE_IAST),
    ] {
        group.bench_with_input(
            BenchmarkId::new("iast_to_itrans", size),
            &text,
            |b, &text| {
                b.iter(|| {
                    transliterator
                        .transliterate(text, "iast", "itrans")
                        .unwrap()
                })
            },
        );
    }

    // IAST to SLP1
    for (size, text) in [
        ("small", SMALL_IAST),
        ("medium", MEDIUM_IAST),
        ("large", LARGE_IAST),
    ] {
        group.bench_with_input(BenchmarkId::new("iast_to_slp1", size), &text, |b, &text| {
            b.iter(|| transliterator.transliterate(text, "iast", "slp1").unwrap())
        });
    }

    // ITRANS to SLP1
    for (size, text) in [
        ("small", SMALL_ITRANS),
        ("medium", MEDIUM_ITRANS),
        ("large", LARGE_ITRANS),
    ] {
        group.bench_with_input(
            BenchmarkId::new("itrans_to_slp1", size),
            &text,
            |b, &text| {
                b.iter(|| {
                    transliterator
                        .transliterate(text, "itrans", "slp1")
                        .unwrap()
                })
            },
        );
    }

    group.finish();
}

fn bench_devanagari_to_roman(c: &mut Criterion) {
    let transliterator = Shlesha::new();

    let mut group = c.benchmark_group("devanagari_to_roman");

    // Devanagari to IAST
    for (size, text) in [
        ("small", SMALL_DEVA),
        ("medium", MEDIUM_DEVA),
        ("large", LARGE_DEVA),
    ] {
        group.bench_with_input(
            BenchmarkId::new("devanagari_to_iast", size),
            &text,
            |b, &text| {
                b.iter(|| {
                    transliterator
                        .transliterate(text, "devanagari", "iast")
                        .unwrap()
                })
            },
        );
    }

    // Devanagari to ITRANS
    for (size, text) in [
        ("small", SMALL_DEVA),
        ("medium", MEDIUM_DEVA),
        ("large", LARGE_DEVA),
    ] {
        group.bench_with_input(
            BenchmarkId::new("devanagari_to_itrans", size),
            &text,
            |b, &text| {
                b.iter(|| {
                    transliterator
                        .transliterate(text, "devanagari", "itrans")
                        .unwrap()
                })
            },
        );
    }

    // Devanagari to SLP1
    for (size, text) in [
        ("small", SMALL_DEVA),
        ("medium", MEDIUM_DEVA),
        ("large", LARGE_DEVA),
    ] {
        group.bench_with_input(
            BenchmarkId::new("devanagari_to_slp1", size),
            &text,
            |b, &text| {
                b.iter(|| {
                    transliterator
                        .transliterate(text, "devanagari", "slp1")
                        .unwrap()
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_roman_to_indic,
    bench_roman_to_roman,
    bench_devanagari_to_roman
);
criterion_main!(benches);
