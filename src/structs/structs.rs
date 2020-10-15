#[derive(Clone)]
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

pub fn struts_basic(){
    let user = User { user_name: "Carlos Eduardo".to_string() };

    let ide = Ide { name: "Intellij".to_string() };

    let fav_user_ide = UserBestIde { user: user.clone(), ide };
    fav_user_ide.show_result();

    let mut user_langs = UserLangs {
        user: user.clone(),
        langs: vec![Lang { name: "Java".to_string() }]
    };
    user_langs.add_lang(Lang { name: "Rust".to_string() });
    user_langs.show_langs();
}