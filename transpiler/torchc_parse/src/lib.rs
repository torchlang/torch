use torchc_diagnosis::Diagnosis;
use torchc_script::Script;

pub async fn parser(script: &mut Script, diagnosis: &mut Diagnosis<'_>) {
    while let Some(token) = script.next_token().await {
        diagnosis.diagnosis("illegal", token.pos, script).await;
        //print!("[{}] ", token.lit().await.unwrap());
    }
}
