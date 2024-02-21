use torchc_diagnosis::Diagnosis;
use torchc_script::{
    iter::{Feature, Mode::Next},
    Script,
};

pub fn parser(script: &mut Script, diagnosis: &mut Diagnosis<'_>) {
    while let Some(token) = script.token(Next(Feature::Code)) {
        diagnosis.diagnosis("illegal", token.pos, script);
        /*
        print!(
            "[{}] ",
            match token.lit().await {
                Some(lit) => format!("{}", lit),
                None => String::new(),
            }
        );
        */
    }
}
