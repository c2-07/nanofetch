use std::fs;
use std::path::PathBuf;

use crate::quotes::load_quotes;
use crate::random::Rng;

#[repr(C)]
struct Tm {
    tm_sec: i32,
    tm_min: i32,
    tm_hour: i32,
    tm_mday: i32,
    tm_mon: i32,
    tm_year: i32,
    tm_wday: i32,
    tm_yday: i32,
    tm_isdst: i32,
    tm_gmtoff: i64,
    tm_zone: *const u8,
}

extern "C" {
    fn time(t: *mut i64) -> i64;
    fn localtime_r(timep: *const i64, result: *mut Tm) -> *mut Tm;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DailyState {
    pub current_date: String,
    pub current_quote: String,
    pub previous_quote: String,
}

impl DailyState {
    pub fn new() -> Self {
        Self {
            current_date: String::new(),
            current_quote: String::new(),
            previous_quote: String::new(),
        }
    }
}

pub fn get_current_date() -> String {
    if let Ok(d) = std::env::var("NANOFETCH_DATE") {
        let trimmed = d.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    #[cfg(unix)]
    {
        let now = unsafe { time(std::ptr::null_mut()) };
        let mut tm: Tm = unsafe { std::mem::zeroed() };
        unsafe {
            localtime_r(&now, &mut tm);
        }
        format!(
            "{:04}-{:02}-{:02}",
            tm.tm_year + 1900,
            tm.tm_mon + 1,
            tm.tm_mday
        )
    }

    #[cfg(not(unix))]
    {
        "1970-01-01".to_string()
    }
}

pub fn get_cache_file_path() -> PathBuf {
    let dir = if let Ok(xdg) = std::env::var("XDG_CACHE_HOME") {
        if !xdg.is_empty() {
            PathBuf::from(xdg).join("nanofetch")
        } else {
            fallback_cache_dir()
        }
    } else {
        fallback_cache_dir()
    };

    let _ = fs::create_dir_all(&dir);
    dir.join("daily_quote.txt")
}

fn fallback_cache_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        if !home.is_empty() {
            return PathBuf::from(home).join(".cache").join("nanofetch");
        }
    }
    std::env::temp_dir().join("nanofetch")
}

pub fn escape_text(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\n', "\\n")
}

pub fn unescape_text(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('\\') => result.push('\\'),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
}

pub fn load_state(path: &PathBuf) -> DailyState {
    let mut state = DailyState::new();
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return state,
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some(val) = trimmed.strip_prefix("CURRENT_DATE=") {
            state.current_date = val.trim().to_string();
        } else if let Some(val) = trimmed.strip_prefix("CURRENT_QUOTE=") {
            state.current_quote = unescape_text(val);
        } else if let Some(val) = trimmed.strip_prefix("PREVIOUS_QUOTE=") {
            state.previous_quote = unescape_text(val);
        }
    }

    state
}

pub fn save_state(path: &PathBuf, state: &DailyState) {
    let mut out = String::new();
    out.push_str("# nanofetch daily quote state\n");
    out.push_str(&format!("CURRENT_DATE={}\n", state.current_date));
    out.push_str(&format!(
        "CURRENT_QUOTE={}\n",
        escape_text(&state.current_quote)
    ));
    out.push_str(&format!(
        "PREVIOUS_QUOTE={}\n",
        escape_text(&state.previous_quote)
    ));

    // Atomic write via temp file
    let tmp_path = path.with_extension(format!("tmp.{}", std::process::id()));
    if fs::write(&tmp_path, out).is_ok() {
        let _ = fs::rename(&tmp_path, path);
    }
}

pub fn get_or_select_daily_quote(force_refresh: bool) -> String {
    let today = get_current_date();
    let cache_path = get_cache_file_path();
    let mut state = load_state(&cache_path);

    if !force_refresh && state.current_date == today && !state.current_quote.is_empty() {
        return state.current_quote.clone();
    }

    let quotes = load_quotes();
    let mut rng = Rng::new();

    // If changing days, archive yesterday's quote into previous_quote
    if state.current_date != today && !state.current_quote.is_empty() {
        state.previous_quote = state.current_quote.clone();
    }

    let avoid_quote = if force_refresh && !state.current_quote.is_empty() {
        &state.current_quote
    } else {
        &state.previous_quote
    };

    // Filter candidate quotes to avoid repeating the previous quote
    let candidate_quotes: Vec<&String> = if quotes.len() > 1 && !avoid_quote.is_empty() {
        let filtered: Vec<&String> = quotes.iter().filter(|&q| q != avoid_quote).collect();
        if filtered.is_empty() {
            quotes.iter().collect()
        } else {
            filtered
        }
    } else {
        quotes.iter().collect()
    };

    let quote_idx = rng.choose_index(candidate_quotes.len());
    let selected_quote = candidate_quotes[quote_idx].clone();

    state.current_date = today;
    state.current_quote = selected_quote.clone();

    save_state(&cache_path, &state);

    selected_quote
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_unescape_roundtrip() {
        let original = "Line 1\nLine 2 with \\ backslashes and symbols";
        let escaped = escape_text(original);
        let unescaped = unescape_text(&escaped);
        assert_eq!(original, unescaped);
    }

    #[test]
    fn test_daily_state_serialization() {
        let temp_dir = std::env::temp_dir().join("nanofetch_test_state_daily");
        let _ = fs::create_dir_all(&temp_dir);
        let test_file = temp_dir.join("test_daily_state.txt");

        let mut state = DailyState::new();
        state.current_date = "2026-10-29".to_string();
        state.current_quote = "Quote Today\nLine 2".to_string();
        state.previous_quote = "Quote Yesterday".to_string();

        save_state(&test_file, &state);
        let loaded = load_state(&test_file);

        assert_eq!(loaded.current_date, state.current_date);
        assert_eq!(loaded.current_quote, state.current_quote);
        assert_eq!(loaded.previous_quote, state.previous_quote);

        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_get_current_date_format() {
        let date = get_current_date();
        // Format YYYY-MM-DD
        assert_eq!(date.len(), 10);
        let parts: Vec<&str> = date.split('-').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].len(), 4);
        assert_eq!(parts[1].len(), 2);
        assert_eq!(parts[2].len(), 2);
    }
}
