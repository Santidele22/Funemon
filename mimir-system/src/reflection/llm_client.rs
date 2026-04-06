pub fn call_ollama(prompt: &str) -> Result<String> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "llama3.2",
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .map_err(|e| RusqliteError::Other(format!("Error conectando a Ollama: {}", e)))?;

    let json: serde_json::Value = response
        .json()
        .map_err(|e| RusqliteError::Other(format!("Error parseando respuesta: {}", e)))?;

    let text = json
        .get("response")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            RusqliteError::Other("Respuesta de Ollama sin campo 'response'".to_string())
        })?;

    Ok(text.to_string())
}
