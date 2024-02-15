use torchc_diagnosis::Diagnosis;
use torchc_script::Script;

pub async fn parser(script: &mut Script, diagnosis: &mut Diagnosis<'_>) {
    while let Some(token) = script.next_token().await {
        print!("[{}] ", token.lit().await.unwrap());
        //diagnosis.diagnosis("illegal", token).await;
    }
}
