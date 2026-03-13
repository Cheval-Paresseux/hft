use crate::logging::Log;
use uuid::Uuid;
use chrono::{Utc, TimeZone};

// ── Sink Trait ────────────────────────────────────────────────────────────────

pub trait Sink<const STR: usize>: Send + 'static {
    fn write(&mut self, log: &Log<STR>);
}

// ── Formatting ────────────────────────────────────────────────────────────────

pub fn format_log<const STR: usize>(log: &Log<STR>, with_date: bool, with_goofy_name: bool) -> String {
    let timestamp = if with_date { timestamp_to_date(log.timestamp) } else { log.timestamp.to_string() };
    let id = if with_goofy_name { goofy_name(log.recorder_id) } else { log.recorder_id.to_string() };

    let parent_id = log.parent_recorder_id.map_or_else(
        || "None".to_string(), |id| if with_goofy_name { goofy_name(id) } else { id.to_string() }
    );

    format!(
        "{} [{:?}] - {:?} --- {}, Son of {}",
        timestamp, log.level, log.event, id, parent_id
    )
}

fn timestamp_to_date(timestamp_ns: u64) -> String {
    let secs = (timestamp_ns / 1_000_000_000) as i64;
    let nanos = (timestamp_ns % 1_000_000_000) as u32;

    Utc.timestamp_opt(secs, nanos)
        .single()
        .unwrap_or_default()
        .format("%Y-%m-%d %H:%M:%S%.3f UTC")
        .to_string()
}

fn goofy_name(id: Uuid) -> String {
    let b = id.as_bytes();
    format!(
        "{} the {}",
        NAMES[b[0] as usize % NAMES.len()],
        ADJECTIVES[b[1] as usize % ADJECTIVES.len()]
    )
}

// ── Word lists ────────────────────────────────────────────────────────────────

const NAMES: &[&str] = &[
    // Serious Business
    "Bachelier", "Cox", "Ross", "Black", "Scholes", "Merton", "Sharpe", "Fama", "French", "Markowitz", "Modigliani", 
    "Keynes", "Buffett", "Soros", "Taleb", "Pythagoras", "Euclid", "Gauss", "Euler", "Fermat", "Riemann", "Poincare",
    "Turing", "Godel", "Cantor", "Hilbert", "Noether", "Ramanujan", "Dijkstra", "Linus", "Knuth", "Lovelace", "Hopper", 
    "McCarthy", "Ritchie", "Thompson", "Kernighan", "Lamport", "Stallman", "Liskov", "Wozniak", "Newton", "Einstein", 
    "Feynman", "Bohr", "Curie", "Heisenberg", "Dirac", "Hawking",

    // Not so serious
    "Yoda", "Vader", "Obi-Wan", "Palpatine", "Grievous", "Maul", "Windu", "Anakin", "Leia", "Ahsoka", "Dooku", "Jarjar",
    "Tyrion", "Cersei", "Daenerys", "Jon", "Arya", "Sansa", "Joffrey", "Tywin", "Jaime", "Ned", "Robb", "Stannis", "Littlefinger", 
    "Varys", "Goku", "Vegeta", "Piccolo", "Gohan", "Frieza", "Cell", "Buu", "Trunks", "Krillin", "Broly", "Beerus", "Whis",
    "Luffy", "Zoro", "Nami", "Sanji", "Chopper", "Robin", "Franky", "Shanks", "Blackbeard", "Whitebeard", "Kaido", "Boa", "Crocodile",
    "Naruto", "Sasuke", "Sakura", "Kakashi", "Itachi", "Madara", "Obito", "Minato", "Tsunade", "Jiraiya", "Orochimaru", "Gaara", 
    "Hinata", "Gandalf", "Aragorn", "Legolas", "Gimli", "Frodo", "Sauron", "Saruman", "Gollum", "Boromir", "Elrond", "Galadriel",
    "Thanos", "Stark", "Strange", "Banner", "Rogers", "Loki", "Ultron", "Geralt", "Kratos", "Arthas", "Thrall", "Illidan", "Link", 
    "Ganon", "Sephiroth", "Cloud", "Tidus", "Solid", "Kazuya", "Ryu",
];

const ADJECTIVES: &[&str] = &[
    "Adamant", "Adroit", "Amatory", "Animistic", "Antic", "Arcadian","Baleful", "Bellicose", "Bilious", "Boorish",
    "Calamitous", "Caustic", "Cerulean", "Comely", "Concomitant", "Contumacious", "Corpulent", "Crapulous", "Defamatory", 
    "Didactic", "Dilatory", "Dowdy", "Efficacious", "Effulgent", "Egregious", "Endemic", "Equanimous", "Execrable",
    "Fastidious", "Feckless", "Fecund", "Friable", "Fulsome", "Garrulous", "Guileless", "Gustatory", "Heuristic", "Histrionic", 
    "Hubristic", "Incendiary", "Insidious", "Insolent", "Intransigent", "Inveterate", "Invidious", "Irksome", "Jejune", 
    "Jocular", "Judicious", "Lachrymose", "Limpid", "Loquacious", "Luminous", "Mannered", "Mendacious", "Meretricious", 
    "Minatory", "Mordant", "Munificent", "Nefarious", "Noxious", "Obtuse", "Parsimonious", "Pendulous", "Pernicious", "Pervasive", 
    "Petulant", "Platitudinous", "Precipitate", "Propitious", "Puckish", "Querulous", "Quiescent", "Rebarbative", "Recalcitrant", 
    "Redolent", "Rhadamanthine", "Risible", "Ruminative", "Sagacious", "Salubrious", "Sartorial", "Sclerotic", "Serpentine", 
    "Spasmodic", "Strident", "Taciturn", "Tenacious", "Tremulous", "Trenchant", "Turbulent", "Turgid", "Ubiquitous", "Uxorious",
    "Verdant", "Voluble", "Voracious", "Wheedling", "Withering", "Zealous",
];

// ── Unit Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logging::{Log, LogLevel, LogEvent};
    use uuid::Uuid;

    fn make_log<const STR: usize>(level: LogLevel, msg: &str) -> Log<STR> {
        Log::new(level, LogEvent::message(msg), Uuid::new_v4(), None)
    }

    fn make_log_with_parent<const STR: usize>(level: LogLevel, msg: &str) -> Log<STR> {
        Log::new(level, LogEvent::message(msg), Uuid::new_v4(), Some(Uuid::new_v4()))
    }

    #[test]
    fn test_format_log_contains_level_and_event() {
        let log = make_log::<32>(LogLevel::Warn, "something went wrong");
        let output = format_log(&log, false, false);

        assert!(output.contains("Warn"), "expected log level in output, got: {output}");
        assert!(output.contains("something went wrong"), "expected message in output, got: {output}");
    }

    #[test]
    fn test_format_log_with_date_produces_utc_timestamp() {
        let log = make_log::<32>(LogLevel::Info, "dated");
        let output = format_log(&log, true, false);

        assert!(output.contains("UTC"), "expected UTC timestamp in output, got: {output}");
    }

    #[test]
    fn test_format_log_with_goofy_name_replaces_uuid() {
        let log = make_log::<32>(LogLevel::Info, "named");
        let uuid_str = log.recorder_id.to_string();

        let without_goofy = format_log(&log, false, false);
        let with_goofy = format_log(&log, false, true);

        assert!(without_goofy.contains(&uuid_str), "plain output should contain UUID");
        assert!(!with_goofy.contains(&uuid_str), "goofy output should not contain UUID");
        assert!(with_goofy.contains(" the "), "goofy name should follow 'X the Y' pattern, got: {with_goofy}");
    }

    #[test]
    fn test_format_log_parent_id_shown_when_present() {
        let log = make_log_with_parent::<32>(LogLevel::Info, "child log");
        let output = format_log(&log, false, false);

        let parent_id = log.parent_recorder_id.unwrap().to_string();
        assert!(output.contains(&parent_id), "expected parent UUID in output, got: {output}");
        assert!(!output.contains("Son of None"), "should not show 'None' when parent exists, got: {output}");
    }

    #[test]
    fn test_format_log_parent_none_when_absent() {
        let log = make_log::<32>(LogLevel::Info, "root log");
        let output = format_log(&log, false, false);

        assert!(output.contains("Son of None"), "expected 'Son of None' for parentless log, got: {output}");
    }
}