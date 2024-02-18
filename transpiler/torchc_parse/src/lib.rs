use torchc_diagnosis::Diagnosis;
use torchc_script::{
    iter::{Feature, Mode::Next},
    Script,
};

pub async fn parser(script: &mut Script, diagnosis: &mut Diagnosis<'_>) {
    while let Some(token) = script.next_token(Next(Feature::Code)).await {
        diagnosis.diagnosis("illegal", token.pos, script).await;
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
