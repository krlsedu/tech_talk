#[derive(Debug, Clone)]
pub struct User {
    pub user_name: String
}

pub struct Ide {
    pub name: String
}

pub struct UserBestIde {
    pub user: User,
    pub ide: Ide,
}

impl UserBestIde {
    pub fn show_result(&self) {
        println!("A ide favorita de {} é {}", self.user.user_name, self.ide.name)
    }
}

pub struct Lang {
    pub name: String
}

pub struct UserLangs {
    pub user: User,
    pub langs: Vec<Lang>,
}

impl UserLangs {
    pub fn add_lang(&mut self, lang: Lang) {
        self.langs.push(lang);
    }

    pub fn show_langs(&self) {
        println!("As linguagens que {} programa são: {}", self.user.user_name, self.langs.iter().fold(String::new(), |acc, lang| acc + &lang.name + ", "))
    }
}