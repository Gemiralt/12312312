#[derive(Clone, Copy, PartialEq)]
pub enum Language {
    English,
    Russian,
}

impl Language {
    pub fn next(&self) -> Self {
        match self {
            Language::English => Language::Russian,
            Language::Russian => Language::English,
        }
    }
    
    pub fn full_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Russian => "Русский",
        }
    }
}

// Тексты интерфейса на разных языках
pub struct Texts {
    pub title: &'static str,
    pub settings_title: &'static str,
    pub theme_dark: &'static str,
    pub theme_blue: &'static str,
    pub theme_green: &'static str,
    pub theme_purple: &'static str,
    pub exit: &'static str,
    pub applied: &'static str,
    pub lang_toggle: &'static str,
}

impl Texts {
    pub fn for_lang(lang: Language) -> Texts {
        match lang {
            Language::English => Texts {
                title: "VERTEXOS",
                settings_title: "COLOR SETTINGS",
                theme_dark: "Dark Matrix",
                theme_blue: "Blue Ocean",
                theme_green: "Neon Green",
                theme_purple: "Purple Haze",
                exit: "Exit",
                applied: "theme applied!",
                lang_toggle: "Language",
            },
            Language::Russian => Texts {
                title: "ВЕРТЕКСОС",
                settings_title: "НАСТРОЙКИ ЦВЕТОВ",
                theme_dark: "Тёмная Матрица",
                theme_blue: "Синий Океан",
                theme_green: "Неоновый Зелёный",
                theme_purple: "Пурпурный Туман",
                exit: "Выход",
                applied: "тема применена!",
                lang_toggle: "Язык",
            },
        }
    }
}