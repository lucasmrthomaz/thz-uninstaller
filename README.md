# THZ Uninstaller

> Desinstalador de programas e limpador de vestígios para desktop Windows e aplicativos da Microsoft Store, desenvolvido em Rust com interface gráfica moderna e nativa via [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui/tree/master/crates/eframe).

---

## 🚀 Funcionalidades

- **Varredura Completa de Programas:**
  - Programas Desktop de 64-bit e 32-bit (via WOW6432Node e chaves de registro do HKLM e HKCU).
  - Aplicativos modernos empacotados da **Microsoft Store** (AppX Packages via repositório de pacotes do Windows).
- **Remoção de Vestígios (*Leftover Traces*):**
  - Identificação e limpeza de chaves órfãs no Registro do Windows (`HKCU\Software`, `HKLM\Software`).
  - Localização e remoção de pastas residuais em `%APPDATA%`, `%LOCALAPPDATA%` e `%ProgramData%`.
- **Desinstalação e Remoção Forçada:**
  - Acionamento direto do desinstalador nativo do software.
  - Suporte à síntese de comandos MSI (`msiexec /X`, `msiexec /I`).
  - Remoção forçada com limpeza atômica de sobras para softwares corrompidos.
- **Interface Gráfica e Atalhos:**
  - Filtragem e busca em tempo real com digitação instantânea.
  - Ordenação por nome, tamanho e data de instalação.
  - Alternância entre aplicativos Desktop (`F9`) e Microsoft Store (`F1`).
  - Atalhos de navegação por teclado e contexto.
- **Exportação de Relatórios:**
  - Geração de relatório de inventário em HTML estruturado (`Ctrl+S`).

---

## 🏗️ Arquitetura e Estrutura do Projeto

O projeto segue os princípios de **Clean Architecture**, **SRP (Single Responsibility Principle)** e **SOLID**:

```
src/
├── main.rs                  # Ponto de entrada enxuto e bootstrap do runner eframe
├── lib.rs                   # Reexportação pública e módulos da biblioteca
├── constants.rs             # Constantes de engenharia reversa e especificações de UI
├── models.rs                # Modelos de domínio (ProgramEntry, TraceItem, ActiveModal)
│
├── utils/                   # Utilitários puros
│   └── formatting.rs        # Formatação de bytes, parsing de datas e sanitização HTML
│
├── services/                # Regras de negócio e integração de sistema
│   ├── cache.rs             # Carregamento e descompressão do cache local (cache.dat)
│   ├── scanner.rs           # Varredura do Registro (Win32/WOW64) e Store Apps
│   ├── leftover.rs          # Localizador de vestígios em pastas de sistema e registro
│   └── reporter.rs          # Montador e exportador de relatórios HTML
│
└── ui/                      # Camada de apresentação gráfica
    ├── app.rs               # Estado central UninstallerApp e loop eframe::App
    ├── components/          # Widgets visuais (tabela virtualizada, menus, busca, status, ícones)
    ├── dialogs/             # Caixas de diálogo e modais (confirmação, rastros, sobre)
    ├── handlers/            # Interações do usuário (atalhos de teclado, ações no SO)
    └── platform/            # Fontes de sistema Segoe UI e captura de tela headless
```

Decisões de design e histórico arquitetural estão documentados em:
- [`docs/adr/0001-modularizacao-srp-clean-code-solid.md`](docs/adr/0001-modularizacao-srp-clean-code-solid.md)

---

## 🛠️ Pré-requisitos e Como Executar

### Pré-requisitos
- **Rust & Cargo** (versão 1.75+ ou superior).
- Sistema Operacional **Windows 10 / 11**.

### Compilação e Execução
```powershell
# Executar a aplicação
cargo run --release

# Executar os testes unitários
cargo test

# Checagem de tipos e lint
cargo check
```

---

## 📄 Licença
Distribuído sob licença livre / código aberto.
