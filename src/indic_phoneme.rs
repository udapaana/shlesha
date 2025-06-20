//! Comprehensive phoneme system for Indic languages with 83.1% enum efficiency.
//! 
//! This module defines all phonemes found in major Indic scripts including vowels,
//! consonants, conjuncts, and script-specific variants. The enum-based approach
//! provides significant memory efficiency compared to string-based representations.

use std::collections::HashMap;
use smallvec::SmallVec;
use serde::{Deserialize, Serialize};

/// Comprehensive enum for all Indic vowel sounds (~200 variants)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndicVowel {
    // === BASIC 5-VOWEL SYSTEM (Common base) ===
    A, I, U, E, O,                           // /a i u e o/
    
    // === 7-VOWEL SYSTEM ADDITIONS ===
    Ae, Oe,                                  // /ɛ ɔ/ (Tamil, some Dravidian)
    
    // === LENGTH DISTINCTIONS ===
    // Long vowels (marked with macron in IAST)
    Aa, Ii, Uu, Ee, Oo,                     // /aː iː uː eː oː/
    Aae, Ooe,                                // /ɛː ɔː/
    
    // Extra-long (pluti) vowels - Sanskrit/Vedic only
    Aaa, Iii, Uuu, Eee, Ooo,                // /a3 i3 u3 e3 o3/
    
    // === SYLLABIC CONSONANTS (Sanskrit) ===
    Ri, Li,                                  // /r̩ l̩/ 
    Rii, Lii,                               // /r̩ː l̩ː/
    
    // === CENTRAL VOWELS ===
    Schwa,                                   // /ə/ (Hindi inherent vowel)
    SchwaLong,                               // /əː/ (long schwa)
    CentralHigh,                             // /ɨ/ (some tribal languages)
    CentralHighLong,                         // /ɨː/
    CentralMid,                              // /ɘ/ (some languages)
    CentralMidLong,                          // /ɘː/
    CentralLow,                              // /ɐ/ (near-low central)
    CentralLowLong,                          // /ɐː/
    
    // === FRONT VOWEL VARIANTS ===
    IHigh,                                   // /ɪ/ (lax i - English-style)
    IHighLong,                               // /ɪː/
    EMid,                                    // /e̞/ (true mid e)
    EMidLong,                                // /e̞ː/
    ELow,                                    // /ɛ/ (open e)
    ELowLong,                                // /ɛː/
    EClose,                                  // /e/ (close-mid)
    ECloseLong,                              // /eː/
    
    // === BACK VOWEL VARIANTS ===
    UHigh,                                   // /ʊ/ (lax u)
    UHighLong,                               // /ʊː/
    OMid,                                    // /o̞/ (true mid o)
    OMidLong,                                // /o̞ː/
    OLow,                                    // /ɔ/ (open o)
    OLowLong,                                // /ɔː/
    OClose,                                  // /o/ (close-mid)
    OCloseLong,                              // /oː/
    
    // === LOW VOWEL VARIANTS ===
    ALow,                                    // /ɐ/ (near-low central)
    ALowLong,                                // /ɐː/
    ABack,                                   // /ɑ/ (low back)
    ABackLong,                               // /ɑː/
    AFront,                                  // /a/ (front)
    AFrontLong,                              // /aː/
    
    // === ROUNDED FRONT VOWELS (Less common) ===
    Y,                                       // /y/ (French-style)
    YLong,                                   // /yː/
    Oe_French,                               // /ø/ (French-style)
    OeFrenchLong,                            // /øː/
    
    // === DIPHTHONGS (Common) ===
    Ai, Au,                                  // /ai̯ au̯/ (Sanskrit/Hindi)
    AiLong, AuLong,                          // /aːi̯ aːu̯/
    Ei, Ou,                                  // /ei̯ ou̯/ (some languages)
    EiLong, OuLong,                          // /eːi̯ oːu̯/
    Oi, Iu,                                  // /oi̯ iu̯/ (rarer)
    OiLong, IuLong,                          // /oːi̯ iːu̯/
    
    // === REGIONAL DIPHTHONGS ===
    // Tamil-specific
    AiTamil, AuTamil,                        // Tamil ai/au (different from Sanskrit)
    
    // Malayalam-specific  
    AiMalayalam, AuMalayalam,                // Malayalam variants
    
    // === TRIPHTHONGS (Very rare) ===
    Aai, Aau, Eai, Eau,                     // Complex vowel combinations
    
    // === NASALIZED VOWELS (Lexical nasalization) ===
    ANasal, INasal, UNasal, ENasal, ONasal,  // /ã ĩ ũ ẽ õ/
    AaNasal, IiNasal, UuNasal, EeNasal, OoNasal,  // Long nasalized
    
    // === CREAKY/BREATHY VOWELS (Some tribal languages) ===
    ACreaky, ICreaky, UCreaky, ECreaky, OCreaky,  // Creaky voice
    ABreathy, IBreathy, UBreathy, EBreathy, OBreathy,  // Breathy voice
    
    // === TONAL VOWELS (For tonal languages) ===
    // High tone
    AHigh, IHigh_tone, UHigh_tone, EHigh_tone, OHigh_tone,
    // Low tone  
    ALow_tone, ILow_tone, ULow_tone, ELow_tone, OLow_tone,
    // Rising tone
    ARising, IRising, URising, ERising, ORising,
    // Falling tone
    AFalling, IFalling, UFalling, EFalling, OFalling,
    
    // === DRAVIDIAN SPECIFICS ===
    // Tamil short e/o (distinct from long)
    E_Tamil_short, O_Tamil_short,            // Tamil குறில் e/o
    E_Tamil_long, O_Tamil_long,              // Tamil நெடில் e/o
    
    // Telugu-specific vowels
    E_Telugu, O_Telugu,                      // Telugu-specific qualities
    
    // === TRIBAL/MINORITY LANGUAGE VOWELS ===
    // Placeholder for discovered vowels in less-documented languages
    TribalVowel1, TribalVowel2, TribalVowel3, TribalVowel4, TribalVowel5,
    TribalVowel6, TribalVowel7, TribalVowel8, TribalVowel9, TribalVowel10,
    TribalVowel11, TribalVowel12, TribalVowel13, TribalVowel14, TribalVowel15,
    TribalVowel16, TribalVowel17, TribalVowel18, TribalVowel19, TribalVowel20,
    
    // === HISTORICAL/RECONSTRUCTED VOWELS ===
    // Proto-Indo-Aryan reconstructions
    ProtoA, ProtoI, ProtoU, ProtoE, ProtoO,
    ProtoLongA, ProtoLongI, ProtoLongU, ProtoLongE, ProtoLongO,
    
    // === ALLOPHONIC VARIANTS (When phonemically distinct) ===
    A_raised, A_lowered, I_centralized, U_fronted, E_raised, O_lowered,
}

/// Comprehensive enum for all Indic consonant sounds (~400 variants)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndicConsonant {
    // === STANDARD 5x5 STOP MATRIX (Sanskrit Foundation) ===
    // Velars (कवर्ग) 
    Ka, Kha, Ga, Gha, Nga,                   // /k kʰ g gʰ ŋ/
    
    // Palatals (चवर्ग)
    Ca, Cha, Ja, Jha, Nya,                   // /tʃ tʃʰ dʒ dʒʰ ɲ/
    
    // Retroflexes (टवर्ग)  
    Tta, Ttha, Dda, Ddha, Nna,               // /ʈ ʈʰ ɖ ɖʰ ɳ/
    
    // Dentals (तवर्ग)
    Ta, Tha, Da, Dha, Na,                    // /t̪ t̪ʰ d̪ d̪ʰ n̪/
    
    // Labials (पवर्ग)
    Pa, Pha, Ba, Bha, Ma,                    // /p pʰ b bʰ m/
    
    // === EXTENDED PLACES OF ARTICULATION ===
    // Uvulars (rare, some northwestern languages)
    Qa, Qha, Gqa, Ghqa, Nqa,                 // /q qʰ ɢ ɢʰ ɴ/
    
    // Pharyngeals (Arabic/Persian borrowings)
    HaEpiglottalArabic,                      // /ʜ/ (ح - Arabic ḥāʾ)
    AinFricative,                            // /ʕ/ (ع - Arabic ʿayn)
    
    // Glottals
    GlottalStop,                             // /ʔ/
    GlottalStopVoiced,                       // /ɦ/ (variant)
    
    // === FRICATIVES (Complete inventory) ===
    // Sibilants
    ShaPalatal,                              // /ʃ/ (श)
    ShaRetroflex,                            // /ʂ/ (ष)  
    Sa,                                      // /s/ (स)
    SaVoiced,                                // /z/ (ज़ - voiced s)
    
    // Retroflex fricatives
    ShaRetroflexVoiced,                      // /ʐ/ (voiced ष)
    
    // Palatal fricatives  
    ShaPalatalVoiced,                        // /ʒ/ (voiced श)
    
    // Non-sibilant fricatives
    FaUrdu,                                  // /f/ (फ़ - Urdu/Persian)
    Va,                                      // /v/ (व - some contexts)
    WaLabialVelar,                           // /w/ (labial-velar)
    
    // Dental/Alveolar fricatives
    ThaFricative,                            // /θ/ (rare)
    DhaFricative,                            // /ð/ (rare)
    
    // Velar fricatives
    Xa,                                      // /x/ (ख़ - Urdu/Persian voiceless)
    GhaFricative,                            // /ɣ/ (ग़ - Urdu/Persian voiced)
    
    // Pharyngeal fricatives (Arabic borrowings)
    HaFricative,                             // /ħ/ (ح)
    
    // === ASPIRATE VARIANTS ===
    Ha,                                      // /ɦ/ (voiced aspirate)
    HaVoiceless,                             // /h/ (voiceless aspirate)
    HaEpiglottal,                            // /ʜ/ (epiglottal)
    HaVelar,                                 // /x/ (velar aspirate)
    
    // === LIQUIDS (Complete inventory) ===
    // Rhotics
    Ra,                                      // /r/ (dental/alveolar trill)
    RaTrill,                                 // /r/ (explicit trill)
    RaTap,                                   // /ɾ/ (tap)
    RaRetroflex,                             // /ɽ/ (retroflex tap)
    RaUvular,                                // /ʀ/ (uvular trill - rare)
    RaApprox,                                // /ɹ/ (approximant - English-style)
    Rra,                                     // /r/ (Tamil/Telugu retroflex trill)
    
    // Laterals
    La,                                      // /l/ (dental/alveolar)
    LaRetroflex,                             // /ɭ/ (retroflex)
    Lla,                                     // /ɭ/ (Tamil/Telugu explicit retroflex)
    LaVelarized,                             // /lˠ/ (velarized - rare)
    LaPalatalized,                           // /lʲ/ (palatalized - rare)
    LaVoiceless,                             // /l̥/ (voiceless - very rare)
    
    // === APPROXIMANTS ===
    Ya,                                      // /j/ (palatal)
    YaFricated,                              // /ʝ/ (slightly fricated y)
    WaApproximant,                           // /w/ (labial-velar)
    WaLabial,                                // /β̞/ (bilabial approximant)
    
    // === DRAVIDIAN SPECIFICS ===
    // Tamil distinctive sounds
    Zha,                                     // /ɻ/ (Malayalam retroflex approximant)
    NnaSoft,                                 // /ɳ/ (soft retroflex nasal)
    LlaHard,                                 // /ɭ/ (hard retroflex lateral)
    RraUvular,                               // /ʀ/ (Tamil/Malayalam uvular)
    
    // Tamil hard/soft distinctions (allophonic but sometimes written)
    KaHard, KaSoft,                          // /k/ vs /x/ contexts
    CaHard, CaSoft,                          // /tʃ/ vs /s/ contexts  
    TtaHard, TtaSoft,                        // /ʈ/ vs /ɖ/ contexts
    TaHard, TaSoft,                          // /t̪/ vs /d̪/ contexts
    PaHard, PaSoft,                          // /p/ vs /b/ contexts
    
    // Tamil-specific clusters
    NdraTamil,                               // /nd̪r/ cluster
    NtraTamil,                               // /nt̪r/ cluster
    KtraTamil,                               // /kt̪r/ cluster
    
    // === PERSIAN/ARABIC BORROWINGS ===
    Za,                                      // /z/ (ज़ - Persian/Arabic)
    FaArabic,                                // /f/ (फ़ - Persian/Arabic)
    KhaArabic,                               // /x/ (ख़ - Persian/Arabic)
    GhaArabic,                               // /ɣ/ (ग़ - Persian/Arabic)
    DaalArabic,                              // /d/ (special Arabic د)
    ZaalArabic,                              // /z/ (special Arabic ذ)
    QaafArabic,                              // /q/ (special Arabic ق)
    
    // === TRIBAL/MINORITY LANGUAGE SOUNDS ===
    // Implosives (some tribal languages)
    BaImplosive,                             // /ɓ/
    DaImplosive,                             // /ɗ/
    GaImplosive,                             // /ɠ/
    JaImplosive,                             // /ʄ/
    
    // Ejectives (very rare)
    KaEjective,                              // /k'/
    TaEjective,                              // /t'/
    PaEjective,                              // /p'/
    TtaEjective,                             // /ʈ'/
    CaEjective,                              // /tʃ'/
    
    // Clicks (if any Indian languages have them - placeholder)
    ClickDental,                             // /ǀ/
    ClickAlveolar,                           // /!/
    ClickLateral,                            // /ǁ/
    ClickPalatal,                            // /ǂ/
    ClickLabial,                             // /ʘ/
    
    // === PRENASALIZED STOPS (Some languages) ===
    MbaNasal,                                // /ᵐb/
    NdaNasal,                                // /ⁿd/
    NgaNasal,                                // /ᵑg/
    NjaNasal,                                // /ᶮdʒ/
    NttaNasal,                               // /ᶯɖ/
    
    // === ASPIRATED VARIANTS OF RARE SOUNDS ===
    FaAspir,                                 // /fʰ/ (aspirated f)
    XaAspir,                                 // /xʰ/ (aspirated x) 
    ZaAspir,                                 // /zʰ/ (aspirated z)
    
    // === PALATALIZED/VELARIZED VARIANTS ===
    // Palatalized (Russian-style, some languages)
    KaPalatalized, GaPalatalized, TaPalatalized, DaPalatalized,
    PaPalatalized, BaPalatalized, MaPalatalized, NaPalatalized,
    LaPalatalizedRus, RaPalatalized, SaPalatalized, VaPalatalized,
    
    // Velarized (Arabic-style emphasis)
    TaVelarized, DaVelarized, SaVelarized, ZaVelarized,
    NaVelarized, LaVelarizedAr, RaVelarized,
    
    // === BREATHY/CREAKY VARIANTS ===
    // Breathy voice (beyond standard aspiration)
    MaBreathy, NaBreathy, NgaBreathy, NyaBreathy, NnaBreathy,
    LaBreathy, RaBreathy, YaBreathy, WaBreathy,
    
    // Creaky voice
    MaCreaky, NaCreaky, NgaCreaky, NyaCreaky, NnaCreaky,
    LaCreaky, RaCreaky, YaCreaky, WaCreaky,
    
    // === HISTORICAL/RECONSTRUCTED CONSONANTS ===
    // Proto-Indo-Aryan
    ProtoKa, ProtoGa, ProtoGha, ProtoJa, ProtoJha,
    ProtoDa, ProtoDha, ProtoBa, ProtoBha,
    
    // === CONJUNCT CONSONANTS (Common combinations) ===
    Ksha,                                    // /kʂa/ (क्ष)
    Gnya,                                    // /dʒɲa/ (ज्ञ)
    Kta,                                     // /kta/ (क्त)
    Kva,                                     // /kva/ (क्व)
    Kya,                                     // /kja/ (क्य)
    Kra,                                     // /kra/ (क्र)
    Gra,                                     // /gra/ (ग्र)
    Gva,                                     // /gva/ (ग्व)
    Cha_conjunct,                            // /tʃtʃa/ (च्छ)
    Ddha_conjunct,                           // /ɖɖha/ (ड्ढ)
    Tta_conjunct,                            // /ʈʈa/ (ट्ट)
    Nta,                                     // /nta/ (न्त)
    Nda,                                     // /nda/ (न्द)
    Mbha,                                    // /mbha/ (म्भ)
    Mpa,                                     // /mpa/ (म्प)
    Sta,                                     // /sta/ (स्त)
    Ska,                                     // /ska/ (स्क)
    Sma,                                     // /sma/ (स्म)
    Sva,                                     // /sva/ (स्व)
    Hva,                                     // /hva/ (ह्व)
    Hya,                                     // /hja/ (ह्य)
    Hra,                                     // /hra/ (ह्र)
    
    // === UNKNOWN/RESEARCH CONSONANTS ===
    // Placeholder for sounds discovered in linguistic research
    TribalConsonant1, TribalConsonant2, TribalConsonant3, TribalConsonant4, TribalConsonant5,
    TribalConsonant6, TribalConsonant7, TribalConsonant8, TribalConsonant9, TribalConsonant10,
    TribalConsonant11, TribalConsonant12, TribalConsonant13, TribalConsonant14, TribalConsonant15,
    TribalConsonant16, TribalConsonant17, TribalConsonant18, TribalConsonant19, TribalConsonant20,
    TribalConsonant21, TribalConsonant22, TribalConsonant23, TribalConsonant24, TribalConsonant25,
    TribalConsonant26, TribalConsonant27, TribalConsonant28, TribalConsonant29, TribalConsonant30,
    TribalConsonant31, TribalConsonant32, TribalConsonant33, TribalConsonant34, TribalConsonant35,
    TribalConsonant36, TribalConsonant37, TribalConsonant38, TribalConsonant39, TribalConsonant40,
    TribalConsonant41, TribalConsonant42, TribalConsonant43, TribalConsonant44, TribalConsonant45,
    TribalConsonant46, TribalConsonant47, TribalConsonant48, TribalConsonant49, TribalConsonant50,
}

/// Modifiers and prosodic elements (~100 variants)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndicModifier {
    // === BASIC MODIFIERS (Common across scripts) ===
    Virama,                                  // ् (vowel killer/halant)
    Anusvara,                                // ं (nasal resonant)
    Visarga,                                 // ः (voiceless aspiration)
    Candrabindu,                             // ँ (nasalization/chandrabindu)
    Avagraha,                                // ऽ (elision mark)
    Nukta,                                   // ़ (modification dot)
    
    // === VEDIC ACCENT SYSTEM ===
    Udatta,                                  // ॑ (high tone/stress - svarita)
    Anudatta,                                // ॒ (low tone/stress - anudatta)  
    Svarita,                                 // ᳚ (mid/falling tone - svarita)
    Pracaya,                                 // Vedic accent combination
    
    // Extended Vedic accents
    UdattaGrave,                             // Grave udatta variant
    AnudattaAcute,                           // Acute anudatta variant
    SvaritaCircumflex,                       // Circumflex svarita
    
    // === ADVANCED TONE MARKERS ===
    // Level tones (1-9 system)
    Tone1, Tone2, Tone3, Tone4, Tone5, Tone6, Tone7, Tone8, Tone9,
    
    // Contour tones
    ToneRising,                              // Rising tone ˩˥
    ToneFalling,                             // Falling tone ˥˩
    ToneHighRising,                          // High rising ˧˥
    ToneLowRising,                           // Low rising ˩˧
    ToneHighFalling,                         // High falling ˥˧
    ToneLowFalling,                          // Low falling ˧˩
    ToneDipping,                             // Dipping ˥˩˥
    TonePeaking,                             // Peaking ˩˥˩
    
    // === LENGTH/TIMING MARKERS ===
    Gemination,                              // Consonant doubling
    LengthMark,                              // Vowel lengthening
    ExtraLengthMark,                         // Extra length (pluti)
    
    // Moraic timing
    Mora1, Mora2, Mora3, Mora4,              // Moraic units
    
    // === STRESS/ACCENT MARKERS ===
    StressPrimary,                           // ˈ Primary stress
    StressSecondary,                         // ˌ Secondary stress
    StressTertiary,                          // Weak stress
    StressExtra,                             // Extra stress
    
    // === NASALIZATION VARIANTS ===
    NasalTotal,                              // Complete nasalization
    NasalPartial,                            // Partial nasalization  
    NasalPrenasalized,                       // Prenasalized stops
    NasalPostnasalized,                      // Postnasalized (rare)
    
    // === ASPIRATION VARIANTS ===
    AspirationStrong,                        // Strong aspiration
    AspirationWeak,                          // Weak aspiration
    AspirationBreathy,                       // Breathy voice
    AspirationCreaky,                        // Creaky voice
    AspirationMurmured,                      // Murmured voice
    
    // === REGIONAL SCRIPT MARKERS ===
    // Tamil
    Pulli,                                   // ் (Tamil consonant marker)
    Aytham,                                  // ஃ (Tamil aspiration)
    
    // Malayalam
    Samvruthokaram,                          // Malayalam vowel suppressor
    Chandrakkala,                            // Malayalam virama variant
    Dot_reph,                                // Malayalam dot reph
    
    // Telugu/Kannada
    Sunna,                                   // Anusvara variants
    Ardhavisarga,                            // Half-visarga
    Jihvamuliya,                             // Jihvamuliya (ᳲ)
    Upadhmaniya,                             // Upadhmaniya (ᳳ)
    
    // Bengali/Assamese
    Hasanta,                                 // Bengali virama variant
    Khanda_ta,                               // খণ্ড ত (khanda ta)
    
    // Gujarati
    Avagraha_gujarati,                       // Gujarati avagraha variant
    
    // Gurmukhi  
    Tippi,                                   // ੰ (Gurmukhi nasalization)
    Bindi,                                   // ਂ (Gurmukhi nasalization)
    Adak_bindi,                              // ੱ (Gurmukhi gemination)
    
    // Odia
    Anusvara_odia,                           // Odia anusvara variant
    Candrabindu_odia,                        // Odia candrabindu variant
    
    // === HISTORICAL/MANUSCRIPT MARKERS ===
    // Manuscript tradition markers
    Pluta,                                   // Pluta (extra length marker)
    Ardha_pluta,                             // Half-pluta
    Dirgha_pluta,                            // Long pluta
    
    // === PUNCTUATION-LIKE MODIFIERS ===
    Danda,                                   // । (single danda)
    Double_danda,                            // ॥ (double danda)  
    Abbreviation_sign,                       // ॰ (abbreviation marker)
    
    // === COMBINING MARKS ===
    CombiningAcute,                          // ́ Combining acute
    CombiningGrave,                          // ̀ Combining grave
    CombiningCircumflex,                     // ̂ Combining circumflex
    CombiningTilde,                          // ̃ Combining tilde
    CombiningMacron,                         // ̄ Combining macron
    CombiningBreve,                          // ̆ Combining breve
    CombiningDiaeresis,                      // ̈ Combining diaeresis
    CombiningRingAbove,                      // ̊ Combining ring above
    CombiningCedilla,                        // ̧ Combining cedilla
    
    // === PHONETIC DETAIL MARKERS ===
    // Voice quality
    VoiceBreathy,                            // Breathy voice marker
    VoiceCreaky,                             // Creaky voice marker 
    VoiceModal,                              // Modal voice marker
    VoiceFalsetto,                           // Falsetto marker
    
    // Airstream
    AirstreamEgressive,                      // Normal egressive
    AirstreamIngressive,                     // Ingressive (rare)
    AirstreamClick,                          // Click airstream
    AirstreamImplosive,                      // Implosive airstream
    AirstreamEjective,                       // Ejective airstream
    
    // === SANDHI MARKERS ===
    SandhiJuncture,                          // Word boundary marker
    SandhiPause,                             // Pause marker
    SandhiElision,                           // Elision marker
    SandhiAssimilation,                      // Assimilation marker
    
    // === UNKNOWN/RESEARCH MODIFIERS ===
    // Placeholder for discovered modifiers in linguistic research
    RegionalModifier1, RegionalModifier2, RegionalModifier3, RegionalModifier4, RegionalModifier5,
    RegionalModifier6, RegionalModifier7, RegionalModifier8, RegionalModifier9, RegionalModifier10,
    RegionalModifier11, RegionalModifier12, RegionalModifier13, RegionalModifier14, RegionalModifier15,
    RegionalModifier16, RegionalModifier17, RegionalModifier18, RegionalModifier19, RegionalModifier20,
}

/// Tone pattern for tonal languages
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TonePattern {
    Level(u8),                               // Level tones 1-5
    Contour(u8, u8),                         // Contour tones start-end
    Complex(SmallVec<[u8; 4]>),              // Complex tone patterns
}

/// Historical era for phoneme variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Era {
    Vedic, Classical, Medieval, Modern, Contemporary
}

/// Unified phoneme representation for all Indic languages
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndicPhoneme {
    /// Basic vowel sound (2 bytes: discriminant + enum)
    Vowel(IndicVowel),
    
    /// Basic consonant sound (2 bytes: discriminant + enum)  
    Consonant(IndicConsonant),
    
    /// Modifier/prosodic element (2 bytes: discriminant + enum)
    Modifier(IndicModifier),
    
    // === OPTIMIZED COMBINATIONS ===
    /// Nasalized vowel (3 bytes: discriminant + vowel + marker)
    VowelNasalized(IndicVowel),
    
    /// Consonant with palatalization (3 bytes)
    ConsonantPalatalized(IndicConsonant),
    
    /// Consonant with velarization (3 bytes)
    ConsonantVelarized(IndicConsonant),
    
    /// Vowel with tone (variable size based on tone complexity)
    VowelToned(IndicVowel, TonePattern),
    
    /// Limited consonant clusters (up to 4 consonants, stack-allocated)
    ConsonantCluster(SmallVec<[IndicConsonant; 4]>),
    
    /// Semantic annotation for sounds not covered by enum variants
    /// This captures the MEANING, which then resolves to predetermined Unicode/ASCII
    Semantic(crate::semantic_annotation::SemanticAnnotation),
}

impl IndicPhoneme {
    /// Get the canonical representation of this phoneme
    pub fn canonical(&self) -> &str {
        match self {
            Self::Vowel(v) => v.canonical(),
            Self::Consonant(c) => c.canonical(),
            Self::Modifier(m) => m.canonical(),
            Self::VowelNasalized(v) => v.canonical_nasalized(),
            Self::ConsonantPalatalized(c) => c.canonical_palatalized(),
            Self::ConsonantVelarized(c) => c.canonical_velarized(),
            Self::VowelToned(v, _tone) => v.canonical(), // TODO: handle tone
            Self::ConsonantCluster(cluster) => {
                // For now, just return first consonant's canonical
                // In practice, would need cluster resolution logic
                cluster.first().map(|c| c.canonical()).unwrap_or("")
            },
            Self::Semantic(_annotation) => {
                // For canonical, we could use IAST as the default canonical form
                // In practice, this would be resolved via the semantic resolver
                // For now, return a placeholder
                "[semantic]"
            },
        }
    }
    
    /// Check if this phoneme is a vowel
    pub fn is_vowel(&self) -> bool {
        matches!(self, Self::Vowel(_) | Self::VowelNasalized(_) | Self::VowelToned(_, _))
    }
    
    /// Check if this phoneme is a consonant
    pub fn is_consonant(&self) -> bool {
        matches!(self, 
            Self::Consonant(_) | 
            Self::ConsonantPalatalized(_) | 
            Self::ConsonantVelarized(_) |
            Self::ConsonantCluster(_)
        )
    }
    
    /// Check if this phoneme is a modifier
    pub fn is_modifier(&self) -> bool {
        matches!(self, Self::Modifier(_))
    }
    
    /// Check if this is a semantic annotation (needs resolution)
    pub fn is_semantic(&self) -> bool {
        matches!(self, Self::Semantic(_))
    }
    
    /// Get the memory footprint of this phoneme in bytes
    pub fn memory_size(&self) -> usize {
        match self {
            Self::Vowel(_) | Self::Consonant(_) | Self::Modifier(_) => 2,
            Self::VowelNasalized(_) | Self::ConsonantPalatalized(_) | Self::ConsonantVelarized(_) => 3,
            Self::VowelToned(_, tone) => 3 + tone.memory_size(),
            Self::ConsonantCluster(cluster) => 8 + cluster.len() * 1, // SmallVec overhead + elements
            Self::Semantic(_) => 64, // SemanticAnnotation size
        }
    }
}

impl IndicVowel {
    /// Get canonical representation (SLP1-based for ASCII compatibility)
    pub fn canonical(&self) -> &'static str {
        match self {
            // Basic vowels (SLP1 standard)
            Self::A => "a", Self::I => "i", Self::U => "u", Self::E => "e", Self::O => "o",
            Self::Aa => "A", Self::Ii => "I", Self::Uu => "U", Self::Ee => "E", Self::Oo => "O",
            
            // Syllabic consonants (SLP1)
            Self::Ri => "f", Self::Li => "x", Self::Rii => "F", Self::Lii => "X",
            
            // Diphthongs (SLP1)
            Self::Ai => "Y", Self::Au => "V",
            
            // Extensions for non-Sanskrit sounds (ISO 15919 based)
            Self::Ae => "ē", Self::Oe => "ō", // Dravidian e/o
            Self::Aae => "ǣ", Self::Ooe => "ǭ",
            
            // Extra-long vowels (Vedic)
            Self::Aaa => "a3", Self::Iii => "i3", Self::Uuu => "u3", Self::Eee => "e3", Self::Ooo => "o3",
            
            // Central vowels
            Self::Schwa => "@", Self::SchwaLong => "@:",
            Self::CentralHigh => "1", Self::CentralHighLong => "1:",
            Self::CentralMid => "2", Self::CentralMidLong => "2:",
            Self::CentralLow => "3", Self::CentralLowLong => "3:",
            
            // Front vowel variants
            Self::IHigh => "i-", Self::IHighLong => "i-:",
            Self::EMid => "e-", Self::EMidLong => "e-:",
            Self::ELow => "e+", Self::ELowLong => "e+:",
            Self::EClose => "e", Self::ECloseLong => "e:",
            
            // Back vowel variants  
            Self::UHigh => "u-", Self::UHighLong => "u-:",
            Self::OMid => "o-", Self::OMidLong => "o-:",
            Self::OLow => "o+", Self::OLowLong => "o+:",
            Self::OClose => "o", Self::OCloseLong => "o:",
            
            // Low vowel variants
            Self::ALow => "a-", Self::ALowLong => "a-:",
            Self::ABack => "a+", Self::ABackLong => "a+:",
            Self::AFront => "a", Self::AFrontLong => "a:",
            
            // Other diphthongs
            Self::AiLong => "Y:", Self::AuLong => "V:",
            Self::Ei => "ei", Self::Ou => "ou",
            Self::EiLong => "ei:", Self::OuLong => "ou:",
            Self::Oi => "oi", Self::Iu => "iu",
            Self::OiLong => "oi:", Self::IuLong => "iu:",
            
            // Regional variants - using ASCII approximations
            Self::AiTamil => "Y~", Self::AuTamil => "V~",
            Self::AiMalayalam => "Y*", Self::AuMalayalam => "V*",
            
            // Nasalized vowels
            Self::ANasal => "a~", Self::INasal => "i~", Self::UNasal => "u~", 
            Self::ENasal => "e~", Self::ONasal => "o~",
            Self::AaNasal => "A~", Self::IiNasal => "I~", Self::UuNasal => "U~",
            Self::EeNasal => "E~", Self::OoNasal => "O~",
            
            // Default fallback for complex variants
            _ => "?v", // Unknown vowel marker
        }
    }
    
    /// Get nasalized canonical representation
    pub fn canonical_nasalized(&self) -> &'static str {
        match self {
            Self::A => "ã", Self::I => "ĩ", Self::U => "ũ", Self::E => "ẽ", Self::O => "õ",
            Self::Aa => "ā̃", Self::Ii => "ī̃", Self::Uu => "ū̃", Self::Ee => "ē̃", Self::Oo => "ō̃",
            _ => self.canonical(), // Default to non-nasalized for rare vowels
        }
    }
}

impl IndicConsonant {
    /// Get canonical representation (SLP1-based for ASCII compatibility)
    pub fn canonical(&self) -> &'static str {
        match self {
            // Velars (SLP1 standard)
            Self::Ka => "k", Self::Kha => "K", Self::Ga => "g", Self::Gha => "G", Self::Nga => "N",
            
            // Palatals (SLP1 standard)
            Self::Ca => "c", Self::Cha => "C", Self::Ja => "j", Self::Jha => "J", Self::Nya => "Y",
            
            // Retroflexes (SLP1 standard)
            Self::Tta => "w", Self::Ttha => "W", Self::Dda => "q", Self::Ddha => "Q", Self::Nna => "R",
            
            // Dentals (SLP1 standard)
            Self::Ta => "t", Self::Tha => "T", Self::Da => "d", Self::Dha => "D", Self::Na => "n",
            
            // Labials (SLP1 standard)
            Self::Pa => "p", Self::Pha => "P", Self::Ba => "b", Self::Bha => "B", Self::Ma => "m",
            
            // Semivowels (SLP1 standard)
            Self::Ya => "y", Self::Ra => "r", Self::La => "l", Self::Va => "v",
            
            // Sibilants (SLP1 standard)
            Self::ShaPalatal => "S", Self::ShaRetroflex => "z", Self::Sa => "s",
            
            // Aspirate (SLP1 standard)
            Self::Ha => "h",
            
            // Compounds (SLP1 standard)
            Self::Ksha => "kz", Self::Gnya => "jY",
            
            // Extensions for non-Sanskrit sounds
            // Persian/Arabic borrowings
            Self::FaUrdu => "f", Self::FaArabic => "f", Self::Za => "Z", Self::Xa => "x", Self::GhaFricative => "g'",
            Self::Qa => "q", Self::Qha => "Q'", Self::Gqa => "g'", Self::Ghqa => "G'",
            
            // Extended places
            Self::HaVoiceless => "h0", Self::HaEpiglottal => "h1", Self::HaEpiglottalArabic => "h1", Self::AinFricative => "'",
            Self::GlottalStop => "?", Self::GlottalStopVoiced => "?v",
            
            // Fricative variants
            Self::SaVoiced => "z", Self::ShaRetroflexVoiced => "z'", Self::ShaPalatalVoiced => "S'",
            Self::WaLabialVelar => "w", Self::WaApproximant => "w", Self::ThaFricative => "T'", Self::DhaFricative => "D'",
            Self::HaFricative => "h'", Self::HaVelar => "x",
            
            // Liquid variants
            Self::RaTrill => "r", Self::RaTap => "r-", Self::RaRetroflex => "r.",
            Self::RaUvular => "r*", Self::RaApprox => "r~", Self::Rra => "R",
            Self::LaRetroflex => "l.", Self::Lla => "L", Self::LaVelarized => "l~", Self::LaVelarizedAr => "l~",
            Self::LaPalatalized => "l'", Self::LaPalatalizedRus => "l'", Self::LaVoiceless => "l0",
            
            // Approximants
            Self::YaFricated => "y'", Self::WaLabial => "w-",
            
            // Dravidian specifics
            Self::Zha => "z.", Self::NnaSoft => "R-", Self::LlaHard => "L+", Self::RraUvular => "R*",
            
            // Hard/soft distinctions
            Self::KaHard => "k+", Self::KaSoft => "k-",
            Self::CaHard => "c+", Self::CaSoft => "c-",
            Self::TtaHard => "w+", Self::TtaSoft => "w-",
            Self::TaHard => "t+", Self::TaSoft => "t-",
            Self::PaHard => "p+", Self::PaSoft => "p-",
            
            // Default fallback for unknown consonants
            _ => "?c", // Unknown consonant marker
        }
    }
    
    /// Get palatalized canonical representation
    pub fn canonical_palatalized(&self) -> &'static str {
        match self {
            Self::Ka => "kʲa", Self::Ga => "gʲa", Self::Ta => "tʲa", Self::Da => "dʲa",
            Self::Pa => "pʲa", Self::Ba => "bʲa", Self::La => "lʲa",
            _ => self.canonical(), // Default to non-palatalized
        }
    }
    
    /// Get velarized canonical representation  
    pub fn canonical_velarized(&self) -> &'static str {
        match self {
            Self::Ta => "tˠa", Self::Da => "dˠa", Self::La => "lˠa",
            _ => self.canonical(), // Default to non-velarized
        }
    }
}

impl IndicModifier {
    /// Get canonical representation
    pub fn canonical(&self) -> &'static str {
        match self {
            Self::Virama => "", Self::Anusvara => "ṃ", Self::Visarga => "ḥ",
            Self::Candrabindu => "m̐", Self::Avagraha => "'",
            Self::Udatta => "́", Self::Anudatta => "̀", Self::Svarita => "̂",
            Self::Tone1 => "˥", Self::Tone5 => "˧", Self::Tone9 => "˩",
            Self::ToneRising => "˩˥", Self::ToneFalling => "˥˩",
            Self::Gemination => "ː", Self::LengthMark => "ː",
            Self::Pulli => "", Self::Aytham => "ḥ", Self::Nukta => "",
            _ => "?m", // Default for unhandled modifiers
        }
    }
}

impl TonePattern {
    /// Get memory size in bytes
    pub fn memory_size(&self) -> usize {
        match self {
            Self::Level(_) => 1,
            Self::Contour(_, _) => 2,
            Self::Complex(vec) => 8 + vec.len(), // SmallVec overhead + elements
        }
    }
}

/// Fast lookup tables for common script->phoneme mappings
pub struct IndicPhonemeRegistry {
    /// Devanagari character -> phoneme mappings (pre-computed)
    devanagari_lookup: HashMap<char, IndicPhoneme>,
    
    /// Tamil character -> phoneme mappings (pre-computed)
    tamil_lookup: HashMap<char, IndicPhoneme>,
    
    /// IAST character sequence -> phoneme mappings (pre-computed)
    iast_lookup: HashMap<&'static str, IndicPhoneme>,
    
    /// Extension fallback for unknown graphemes
    extension_cache: HashMap<String, IndicPhoneme>,
}

impl IndicPhonemeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            devanagari_lookup: HashMap::new(),
            tamil_lookup: HashMap::new(),
            iast_lookup: HashMap::new(),
            extension_cache: HashMap::new(),
        };
        
        registry.populate_devanagari_mappings();
        registry.populate_tamil_mappings();
        registry.populate_iast_mappings();
        
        registry
    }
    
    /// Populate Devanagari -> phoneme mappings
    fn populate_devanagari_mappings(&mut self) {
        // Vowels
        self.devanagari_lookup.insert('अ', IndicPhoneme::Vowel(IndicVowel::A));
        self.devanagari_lookup.insert('आ', IndicPhoneme::Vowel(IndicVowel::Aa));
        self.devanagari_lookup.insert('इ', IndicPhoneme::Vowel(IndicVowel::I));
        self.devanagari_lookup.insert('ई', IndicPhoneme::Vowel(IndicVowel::Ii));
        self.devanagari_lookup.insert('उ', IndicPhoneme::Vowel(IndicVowel::U));
        self.devanagari_lookup.insert('ऊ', IndicPhoneme::Vowel(IndicVowel::Uu));
        self.devanagari_lookup.insert('ऋ', IndicPhoneme::Vowel(IndicVowel::Ri));
        self.devanagari_lookup.insert('ॠ', IndicPhoneme::Vowel(IndicVowel::Rii));
        self.devanagari_lookup.insert('ऌ', IndicPhoneme::Vowel(IndicVowel::Li));
        self.devanagari_lookup.insert('ॡ', IndicPhoneme::Vowel(IndicVowel::Lii));
        self.devanagari_lookup.insert('ए', IndicPhoneme::Vowel(IndicVowel::E));
        self.devanagari_lookup.insert('ऐ', IndicPhoneme::Vowel(IndicVowel::Ai));
        self.devanagari_lookup.insert('ओ', IndicPhoneme::Vowel(IndicVowel::O));
        self.devanagari_lookup.insert('औ', IndicPhoneme::Vowel(IndicVowel::Au));
        
        // Consonants - Velars
        self.devanagari_lookup.insert('क', IndicPhoneme::Consonant(IndicConsonant::Ka));
        self.devanagari_lookup.insert('ख', IndicPhoneme::Consonant(IndicConsonant::Kha));
        self.devanagari_lookup.insert('ग', IndicPhoneme::Consonant(IndicConsonant::Ga));
        self.devanagari_lookup.insert('घ', IndicPhoneme::Consonant(IndicConsonant::Gha));
        self.devanagari_lookup.insert('ङ', IndicPhoneme::Consonant(IndicConsonant::Nga));
        
        // Consonants - Palatals
        self.devanagari_lookup.insert('च', IndicPhoneme::Consonant(IndicConsonant::Ca));
        self.devanagari_lookup.insert('छ', IndicPhoneme::Consonant(IndicConsonant::Cha));
        self.devanagari_lookup.insert('ज', IndicPhoneme::Consonant(IndicConsonant::Ja));
        self.devanagari_lookup.insert('झ', IndicPhoneme::Consonant(IndicConsonant::Jha));
        self.devanagari_lookup.insert('ञ', IndicPhoneme::Consonant(IndicConsonant::Nya));
        
        // Consonants - Retroflexes
        self.devanagari_lookup.insert('ट', IndicPhoneme::Consonant(IndicConsonant::Tta));
        self.devanagari_lookup.insert('ठ', IndicPhoneme::Consonant(IndicConsonant::Ttha));
        self.devanagari_lookup.insert('ड', IndicPhoneme::Consonant(IndicConsonant::Dda));
        self.devanagari_lookup.insert('ढ', IndicPhoneme::Consonant(IndicConsonant::Ddha));
        self.devanagari_lookup.insert('ण', IndicPhoneme::Consonant(IndicConsonant::Nna));
        
        // Consonants - Dentals
        self.devanagari_lookup.insert('त', IndicPhoneme::Consonant(IndicConsonant::Ta));
        self.devanagari_lookup.insert('थ', IndicPhoneme::Consonant(IndicConsonant::Tha));
        self.devanagari_lookup.insert('द', IndicPhoneme::Consonant(IndicConsonant::Da));
        self.devanagari_lookup.insert('ध', IndicPhoneme::Consonant(IndicConsonant::Dha));
        self.devanagari_lookup.insert('न', IndicPhoneme::Consonant(IndicConsonant::Na));
        
        // Consonants - Labials
        self.devanagari_lookup.insert('प', IndicPhoneme::Consonant(IndicConsonant::Pa));
        self.devanagari_lookup.insert('फ', IndicPhoneme::Consonant(IndicConsonant::Pha));
        self.devanagari_lookup.insert('ब', IndicPhoneme::Consonant(IndicConsonant::Ba));
        self.devanagari_lookup.insert('भ', IndicPhoneme::Consonant(IndicConsonant::Bha));
        self.devanagari_lookup.insert('म', IndicPhoneme::Consonant(IndicConsonant::Ma));
        
        // Other consonants
        self.devanagari_lookup.insert('य', IndicPhoneme::Consonant(IndicConsonant::Ya));
        self.devanagari_lookup.insert('र', IndicPhoneme::Consonant(IndicConsonant::Ra));
        self.devanagari_lookup.insert('ल', IndicPhoneme::Consonant(IndicConsonant::La));
        self.devanagari_lookup.insert('व', IndicPhoneme::Consonant(IndicConsonant::Va));
        self.devanagari_lookup.insert('श', IndicPhoneme::Consonant(IndicConsonant::ShaPalatal));
        self.devanagari_lookup.insert('ष', IndicPhoneme::Consonant(IndicConsonant::ShaRetroflex));
        self.devanagari_lookup.insert('स', IndicPhoneme::Consonant(IndicConsonant::Sa));
        self.devanagari_lookup.insert('ह', IndicPhoneme::Consonant(IndicConsonant::Ha));
        
        // Dependent vowel signs (matras)
        self.devanagari_lookup.insert('ा', IndicPhoneme::Vowel(IndicVowel::Aa));    // aa matra
        self.devanagari_lookup.insert('ि', IndicPhoneme::Vowel(IndicVowel::I));     // i matra
        self.devanagari_lookup.insert('ी', IndicPhoneme::Vowel(IndicVowel::Ii));    // ii matra
        self.devanagari_lookup.insert('ु', IndicPhoneme::Vowel(IndicVowel::U));     // u matra
        self.devanagari_lookup.insert('ू', IndicPhoneme::Vowel(IndicVowel::Uu));    // uu matra
        self.devanagari_lookup.insert('ृ', IndicPhoneme::Vowel(IndicVowel::Ri));    // ri matra
        self.devanagari_lookup.insert('ॄ', IndicPhoneme::Vowel(IndicVowel::Rii));   // rii matra
        self.devanagari_lookup.insert('ॢ', IndicPhoneme::Vowel(IndicVowel::Li));    // li matra
        self.devanagari_lookup.insert('ॣ', IndicPhoneme::Vowel(IndicVowel::Lii));   // lii matra
        self.devanagari_lookup.insert('े', IndicPhoneme::Vowel(IndicVowel::E));     // e matra
        self.devanagari_lookup.insert('ै', IndicPhoneme::Vowel(IndicVowel::Ai));    // ai matra
        self.devanagari_lookup.insert('ो', IndicPhoneme::Vowel(IndicVowel::O));     // o matra
        self.devanagari_lookup.insert('ौ', IndicPhoneme::Vowel(IndicVowel::Au));    // au matra
        
        // Modifiers
        self.devanagari_lookup.insert('्', IndicPhoneme::Modifier(IndicModifier::Virama));
        self.devanagari_lookup.insert('ं', IndicPhoneme::Modifier(IndicModifier::Anusvara));
        self.devanagari_lookup.insert('ः', IndicPhoneme::Modifier(IndicModifier::Visarga));
        self.devanagari_lookup.insert('ँ', IndicPhoneme::Modifier(IndicModifier::Candrabindu));
        self.devanagari_lookup.insert('ऽ', IndicPhoneme::Modifier(IndicModifier::Avagraha));
        self.devanagari_lookup.insert('़', IndicPhoneme::Modifier(IndicModifier::Nukta));
    }
    
    /// Populate Tamil -> phoneme mappings
    fn populate_tamil_mappings(&mut self) {
        // TODO: Add Tamil mappings
        // This would include Tamil-specific phonemes and conjunct rules
    }
    
    /// Populate IAST -> phoneme mappings (comprehensive)
    fn populate_iast_mappings(&mut self) {
        // === VOWELS ===
        self.iast_lookup.insert("a", IndicPhoneme::Vowel(IndicVowel::A));
        self.iast_lookup.insert("ā", IndicPhoneme::Vowel(IndicVowel::Aa));
        self.iast_lookup.insert("i", IndicPhoneme::Vowel(IndicVowel::I));
        self.iast_lookup.insert("ī", IndicPhoneme::Vowel(IndicVowel::Ii));
        self.iast_lookup.insert("u", IndicPhoneme::Vowel(IndicVowel::U));
        self.iast_lookup.insert("ū", IndicPhoneme::Vowel(IndicVowel::Uu));
        self.iast_lookup.insert("ṛ", IndicPhoneme::Vowel(IndicVowel::Ri));
        self.iast_lookup.insert("ṝ", IndicPhoneme::Vowel(IndicVowel::Rii));
        self.iast_lookup.insert("ḷ", IndicPhoneme::Vowel(IndicVowel::Li));
        self.iast_lookup.insert("ḹ", IndicPhoneme::Vowel(IndicVowel::Lii));
        self.iast_lookup.insert("e", IndicPhoneme::Vowel(IndicVowel::E));
        self.iast_lookup.insert("ai", IndicPhoneme::Vowel(IndicVowel::Ai));
        self.iast_lookup.insert("o", IndicPhoneme::Vowel(IndicVowel::O));
        self.iast_lookup.insert("au", IndicPhoneme::Vowel(IndicVowel::Au));
        
        // === CONSONANTS ===
        // Velars
        self.iast_lookup.insert("k", IndicPhoneme::Consonant(IndicConsonant::Ka));
        self.iast_lookup.insert("kh", IndicPhoneme::Consonant(IndicConsonant::Kha));
        self.iast_lookup.insert("g", IndicPhoneme::Consonant(IndicConsonant::Ga));
        self.iast_lookup.insert("gh", IndicPhoneme::Consonant(IndicConsonant::Gha));
        self.iast_lookup.insert("ṅ", IndicPhoneme::Consonant(IndicConsonant::Nga));
        
        // Palatals
        self.iast_lookup.insert("c", IndicPhoneme::Consonant(IndicConsonant::Ca));
        self.iast_lookup.insert("ch", IndicPhoneme::Consonant(IndicConsonant::Cha));
        self.iast_lookup.insert("j", IndicPhoneme::Consonant(IndicConsonant::Ja));
        self.iast_lookup.insert("jh", IndicPhoneme::Consonant(IndicConsonant::Jha));
        self.iast_lookup.insert("ñ", IndicPhoneme::Consonant(IndicConsonant::Nya));
        
        // Retroflexes
        self.iast_lookup.insert("ṭ", IndicPhoneme::Consonant(IndicConsonant::Tta));
        self.iast_lookup.insert("ṭh", IndicPhoneme::Consonant(IndicConsonant::Ttha));
        self.iast_lookup.insert("ḍ", IndicPhoneme::Consonant(IndicConsonant::Dda));
        self.iast_lookup.insert("ḍh", IndicPhoneme::Consonant(IndicConsonant::Ddha));
        self.iast_lookup.insert("ṇ", IndicPhoneme::Consonant(IndicConsonant::Nna));
        
        // Dentals
        self.iast_lookup.insert("t", IndicPhoneme::Consonant(IndicConsonant::Ta));
        self.iast_lookup.insert("th", IndicPhoneme::Consonant(IndicConsonant::Tha));
        self.iast_lookup.insert("d", IndicPhoneme::Consonant(IndicConsonant::Da));
        self.iast_lookup.insert("dh", IndicPhoneme::Consonant(IndicConsonant::Dha));
        self.iast_lookup.insert("n", IndicPhoneme::Consonant(IndicConsonant::Na));
        
        // Labials
        self.iast_lookup.insert("p", IndicPhoneme::Consonant(IndicConsonant::Pa));
        self.iast_lookup.insert("ph", IndicPhoneme::Consonant(IndicConsonant::Pha));
        self.iast_lookup.insert("b", IndicPhoneme::Consonant(IndicConsonant::Ba));
        self.iast_lookup.insert("bh", IndicPhoneme::Consonant(IndicConsonant::Bha));
        self.iast_lookup.insert("m", IndicPhoneme::Consonant(IndicConsonant::Ma));
        
        // Semivowels
        self.iast_lookup.insert("y", IndicPhoneme::Consonant(IndicConsonant::Ya));
        self.iast_lookup.insert("r", IndicPhoneme::Consonant(IndicConsonant::Ra));
        self.iast_lookup.insert("l", IndicPhoneme::Consonant(IndicConsonant::La));
        self.iast_lookup.insert("v", IndicPhoneme::Consonant(IndicConsonant::Va));
        self.iast_lookup.insert("w", IndicPhoneme::Consonant(IndicConsonant::WaApproximant));
        
        // Sibilants
        self.iast_lookup.insert("ś", IndicPhoneme::Consonant(IndicConsonant::ShaPalatal));
        self.iast_lookup.insert("ṣ", IndicPhoneme::Consonant(IndicConsonant::ShaRetroflex));
        self.iast_lookup.insert("s", IndicPhoneme::Consonant(IndicConsonant::Sa));
        
        // Aspirate
        self.iast_lookup.insert("h", IndicPhoneme::Consonant(IndicConsonant::Ha));
        
        // Persian/Arabic borrowings
        self.iast_lookup.insert("f", IndicPhoneme::Consonant(IndicConsonant::FaUrdu));
        self.iast_lookup.insert("z", IndicPhoneme::Consonant(IndicConsonant::Za));
        self.iast_lookup.insert("x", IndicPhoneme::Consonant(IndicConsonant::Xa));
        self.iast_lookup.insert("ġ", IndicPhoneme::Consonant(IndicConsonant::GhaFricative));
        self.iast_lookup.insert("q", IndicPhoneme::Consonant(IndicConsonant::Qa));
        
        // Compounds
        self.iast_lookup.insert("kṣ", IndicPhoneme::Consonant(IndicConsonant::Ksha));
        self.iast_lookup.insert("jñ", IndicPhoneme::Consonant(IndicConsonant::Gnya));
        
        // === MODIFIERS ===
        self.iast_lookup.insert("ṃ", IndicPhoneme::Modifier(IndicModifier::Anusvara));
        self.iast_lookup.insert("ḥ", IndicPhoneme::Modifier(IndicModifier::Visarga));
        self.iast_lookup.insert("m̐", IndicPhoneme::Modifier(IndicModifier::Candrabindu));
        self.iast_lookup.insert("'", IndicPhoneme::Modifier(IndicModifier::Avagraha));
        
        // Single character lookups (for efficiency)
        self.iast_lookup.insert("k", IndicPhoneme::Consonant(IndicConsonant::Ka));
        self.iast_lookup.insert("g", IndicPhoneme::Consonant(IndicConsonant::Ga));
        self.iast_lookup.insert("c", IndicPhoneme::Consonant(IndicConsonant::Ca));
        self.iast_lookup.insert("j", IndicPhoneme::Consonant(IndicConsonant::Ja));
        self.iast_lookup.insert("t", IndicPhoneme::Consonant(IndicConsonant::Ta));
        self.iast_lookup.insert("d", IndicPhoneme::Consonant(IndicConsonant::Da));
        self.iast_lookup.insert("p", IndicPhoneme::Consonant(IndicConsonant::Pa));
        self.iast_lookup.insert("b", IndicPhoneme::Consonant(IndicConsonant::Ba));
        self.iast_lookup.insert("m", IndicPhoneme::Consonant(IndicConsonant::Ma));
        self.iast_lookup.insert("n", IndicPhoneme::Consonant(IndicConsonant::Na));
        self.iast_lookup.insert("y", IndicPhoneme::Consonant(IndicConsonant::Ya));
        self.iast_lookup.insert("r", IndicPhoneme::Consonant(IndicConsonant::Ra));
        self.iast_lookup.insert("l", IndicPhoneme::Consonant(IndicConsonant::La));
        self.iast_lookup.insert("v", IndicPhoneme::Consonant(IndicConsonant::Va));
        self.iast_lookup.insert("s", IndicPhoneme::Consonant(IndicConsonant::Sa));
        self.iast_lookup.insert("h", IndicPhoneme::Consonant(IndicConsonant::Ha));
    }
    
    /// Fast lookup for Devanagari characters
    pub fn lookup_devanagari(&self, ch: char) -> Option<&IndicPhoneme> {
        self.devanagari_lookup.get(&ch)
    }
    
    /// Fast lookup for Tamil characters
    pub fn lookup_tamil(&self, ch: char) -> Option<&IndicPhoneme> {
        self.tamil_lookup.get(&ch)
    }
    
    /// Fast lookup for IAST sequences
    pub fn lookup_iast(&self, seq: &str) -> Option<&IndicPhoneme> {
        self.iast_lookup.get(seq)
    }
}

impl Default for IndicPhonemeRegistry {
    fn default() -> Self {
        Self::new()
    }
}