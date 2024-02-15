use torchc_diagnosis::Diagnosis;
use torchc_lex::Tokens;

pub async fn parser(tokens: &mut Tokens<'_>, diagnosis: &mut Diagnosis<'_>) {
    while let Some(token) = tokens.next_token().await {
        print!("[{}] ", token.lit().await.unwrap());
        //diagnosis.diagnosis("illegal", token).await;
    }
}
