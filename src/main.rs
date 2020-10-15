use structs::structs::User;
use structs::structs::UserBestIde;
use structs::structs::UserLangs;
use structs::structs::Lang;

//mod basico;
mod structs;


fn main() {
    //basico::declaracao_basica::contem_warnings();
    let user = User { user_name: "Carlos Eduardo".to_string() };
    let ide = structs::structs::Ide { name: "Intellij".to_string() };
    let fav_user_ide = UserBestIde { user: user.clone(), ide };
    fav_user_ide.show_result();
    let mut user_langs = UserLangs {
        user: user.clone(),
        langs: vec![Lang { name: "Java".to_string() }]
    };
    user_langs.add_lang(Lang { name: "Rust".to_string() });
    user_langs.show_langs();
}
