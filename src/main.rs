use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        println!("\n=========================================");
        println!("     YouTube Downloader Pro (Rust)         ");
        println!("=========================================");
        println!("1. Baixar Vídeo / Áudio");
        println!("2. Atualizar o yt-dlp");
        println!("3. Sair");
        print!("Escolha uma opção (1-3): ");
        io::stdout().flush().unwrap();

        let mut opcao_menu = String::new();
        io::stdin().read_line(&mut opcao_menu).expect("Falha ao ler opção");

        match opcao_menu.trim() {
            "1" => iniciar_download(),
            "2" => atualizar_ytdlp(),
            "3" => {
                println!("\nA encerrar o programa. Até logo!");
                break;
            }
            _ => println!("\n⚠️ Opção inválida! Tente novamente."),
        }
    }
}

fn iniciar_download() {
    print!("\nDigite ou cole a URL do vídeo: ");
    io::stdout().flush().unwrap();

    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Falha ao ler URL");
    let url = url.trim();

    if url.is_empty() {
        println!("❌ A URL não pode estar vazia!");
        return;
    }

    println!("\nEscolha o formato desejado:");
    println!("1. MP4  (Vídeo em alta qualidade)");
    println!("2. MKV  (Vídeo com suporte a múltiplas legendas/áudios)");
    println!("3. MP3  (Apenas Áudio com Metadados e Capa)");
    print!("Escolha o formato (1-3): ");
    io::stdout().flush().unwrap();

    let mut formato_escolha = String::new();
    io::stdin().read_line(&mut formato_escolha).expect("Falha ao ler formato");

    let mut cmd = Command::new("yt-dlp");

    // 📂 Tenta pegar a pasta oficial de Downloads do sistema operacional
    if let Some(pasta_downloads) = dirs::download_dir() {
        if let Some(caminho_str) = pasta_downloads.to_str() {
            cmd.arg("--paths").arg(caminho_str);
        }
    } else {
        // Fallback caso não encontre: usa o diretório atual
        cmd.arg("--paths").arg(".");
    }

    // Ativa o download acelerado em 8 conexões paralelas
    cmd.arg("-N").arg("8");

    match formato_escolha.trim() {
        "2" => {
            println!("\n🎥 Baixando vídeo no formato MKV...");
            cmd.arg("-f").arg("bv*+ba/b")
               .arg("--merge-output-format").arg("mkv");
        }
        "3" => {
            println!("\n🎵 Extraindo áudio no formato MP3 com metadados e capa...");
            cmd.arg("-x")
               .arg("--audio-format").arg("mp3")
               .arg("--audio-quality").arg("0")
               .arg("--embed-metadata")
               .arg("--embed-thumbnail");
        }
        _ => {
            println!("\n🎥 Baixando vídeo no formato MP4...");
            cmd.arg("-f").arg("bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4] / bv*+ba/b")
               .arg("--merge-output-format").arg("mp4");
        }
    }

    cmd.arg(url);

    match cmd.status() {
        Ok(status) if status.success() => {
            println!("\n✅ Download concluído! O arquivo foi salvo na sua pasta padrão de Downloads.");
        }
        _ => {
            println!("\n❌ Ocorreu um erro durante o download.");
            println!("Certifique-se de que o FFmpeg está instalado caso esteja convertendo para MP3/MKV.");
        }
    }
}

fn atualizar_ytdlp() {
    println!("\n🔄 A verificar atualizações do yt-dlp...");
    let status = Command::new("yt-dlp").arg("-U").status();

    match status {
        Ok(s) if s.success() => println!("\n✅ yt-dlp verificado/atualizado com sucesso!"),
        _ => println!("\n❌ Falha ao atualizar o yt-dlp."),
    }
}
