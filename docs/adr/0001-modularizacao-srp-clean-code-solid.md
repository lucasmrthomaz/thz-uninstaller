# ADR 0001: Refatoração Modular baseada em SRP, Clean Code e SOLID

## Status
Aceito e Atualizado (com Subpastas na Camada UI)

## Contexto
O projeto `geek-uninstaller-open` concentrava originalmente toda a lógica da aplicação em um único arquivo monolítico (`src/main.rs`) com mais de 2.500 linhas. Nele coexistiam constantes de engenharia reversa, modelos de domínio, utilitários, acesso ao Registro do Windows e todo o ecossistema visual de interface gráfica.

Após as etapas iniciais de separação, a camada de UI (`src/ui`) continha diversos arquivos soltos na raiz do módulo. Para atingir um design limpo, coeso e escalável seguindo os princípios de Domain-Driven Design e Clean Architecture, foi solicitada a estruturação da camada de UI em subpastas semânticas.

## Decisão Arquitetural
Organizar a camada `src/ui` em subdiretórios categorizados por responsabilidade técnica e domínio visual:

### Estrutura de Subpastas em `src/ui/`:
```
src/ui/
├── app.rs                   # Estado central (UninstallerApp) e implementação eframe::App
├── mod.rs                   # Ponto central de exportação pública da camada UI
├── components/              # Elementos visuais e painéis da interface principal
│   ├── icons.rs             # Renderização vetorial de ícones de programas (egui::Painter)
│   ├── menu.rs              # Barra de menus (Arquivo, Ação, Ver, Ajuda) e menus de ação
│   ├── search.rs            # Barra de busca rápida com digitação reativa
│   ├── status.rs            # Barra inferior de estatísticas e link para versão PRO
│   ├── table.rs             # Layout virtualizado, cabeçalhos clicáveis, linhas e scrollbar
│   ├── title_bar.rs         # Barra de título simulada e bordas externas para headless/testes
│   └── mod.rs
├── dialogs/                 # Janelas modais e caixas de diálogo
│   ├── modals.rs            # Modais de Sobre, Desinstalação, Remoção Forçada e Rastros
│   └── mod.rs
├── handlers/                # Tratamento de entradas do usuário e despacho de ações
│   ├── actions.rs           # Despacho de comandos do SO (Regedit, Explorer, Web, HTML)
│   ├── shortcuts.rs         # Atalhos de teclado globais e navegação por setas/scroll
│   └── mod.rs
└── platform/                # Integração com recursos nativos do sistema operacional
    ├── fonts.rs             # Carregamento dinâmico de fontes Segoe UI via $env:WINDIR
    ├── screenshot.rs        # Automação de screenshots via CLI (--screenshot)
    └── mod.rs
```

## Benefícios e Métricas
1. **Alta Coesão e Baixo Acoplamento:**
   - Componentes visuais (`components`) são independentes de chamadas de processo ou SO (`handlers`).
   - Modais (`dialogs`) possuem ciclo de vida e estado próprios, sem poluir os layouts da janela principal.
   - Recursos específicos do Windows (`platform`) ficam isolados, facilitando futuras adaptações cross-platform.
2. **Métricas de Linhas de Código:**
   - Nenhum arquivo em `src/ui/` excede 300 linhas de código (máximo de 294 linhas em `table.rs`).
   - A legibilidade e a facilidade de navegação entre arquivos foram expressivamente melhoradas.
3. **Qualidade e Estabilidade:**
   - Zero *warnings* na compilação (`cargo check`).
   - 100% dos testes unitários passando (`cargo test`).
