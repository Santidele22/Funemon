// src/reflection/llm_client.rs

pub fn call_ollama(prompt: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| format!("Error creando cliente: {}", e))?;

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "llama3.2:latest",
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .map_err(|e| format!("Error de red Ollama: {}", e))?;

    let json: serde_json::Value = response
        .json()
        .map_err(|e| format!("Error al leer JSON: {}", e))?;

    let text = json["response"]
        .as_str()
        .ok_or_else(|| "Ollama devolvió un formato inesperado".to_string())?;

    Ok(text.to_string())
}
