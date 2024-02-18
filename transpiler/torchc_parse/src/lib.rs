use torchc_diagnosis::Diagnosis;
use torchc_script::Script;

pub async fn parser(script: &mut Script, diagnosis: &mut Diagnosis<'_>) {
    while let Some(token) = script.next_raw_token().await {
        print!(
            "[{}] ",
            match token.lit().await {
                Some(lit) => format!("{}", lit),
                None => String::new(),
            }
        );
        //diagnosis.diagnosis("illegal", token.pos, script).await;
    }
}
