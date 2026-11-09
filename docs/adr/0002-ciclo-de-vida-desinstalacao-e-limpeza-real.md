# ADR 0002: Ciclo de Vida Real de Desinstalação, Ações Pré/Pós e Limpeza Recursiva de Registro e Disco

## Status
Aceito

## Contexto e Diagnóstico
A análise das rotinas anteriores de remoção evidenciou falhas estruturais críticas que impediam o funcionamento real da aplicação:
1. **Ausência de Exclusão Real de Registro:** O clique no botão "Remover Entrada" apenas removia o item do vetor em memória (`Vec<ProgramEntry>`) via `retain(|p| p.id != id)`. Nenhuma chamada às APIs Win32 ou manipulação de chaves no Registro do Windows (`RegDeleteKey` / `delete_subkey_all`) era executada.
2. **Ausência de Exclusão Real de Arquivos e Vestígios:** O botão "Finish" do scanner de sobras apenas contabilizava `traces.iter().filter(|t| t.checked).count()`, sem invocar `fs::remove_dir_all` ou exclusão de chaves.
3. **Ausência de Ciclo de Vida Pré e Pós-Desinstalação:** O desinstalador nativo era disparado de forma desvinculada via `cmd.exe /C ... spawn()`, e a tela de varredura de sobras era aberta instantaneamente no mesmo frame, antes mesmo que o instalador começasse a rodar ou encerrasse suas tarefas.
4. **Travamentos por Processos Bloqueados:** Softwares em execução no momento da desinstalação retinham arquivos em uso (DLLs, EXEs), levando a falhas silenciosas do desinstalador oficial.

## Decisões Arquiteturais

### 1. Camada de Limpeza Real (`src/services/cleaner.rs`)
- **Exclusão Recursiva no Registro (`delete_registry_path`):**
  - Parsing de hives (`HKEY_LOCAL_MACHINE`, `HKEY_CURRENT_USER`, etc.) e subchaves.
  - Suporte a arquiteturas 64-bit e 32-bit (`KEY_WOW64_64KEY` e `KEY_WOW64_32KEY` para WOW6432Node).
  - Remoção recursiva de todas as subchaves e valores via `winreg::RegKey::delete_subkey_all`.
- **Exclusão no Sistema de Arquivos (`delete_filesystem_path`):**
  - Limpeza forçada (`remove_dir_all_force`): remoção recursiva de sinalizadores *read-only* antes da exclusão de pastas e arquivos para evitar erros de `PermissionDenied`.
- **Exclusão de Entrada de Desinstalação (`delete_uninstall_entry`):**
  - Remove a chave sob `SOFTWARE\...\Uninstall` no hive correspondente.
  - Para aplicativos da Microsoft Store, invoca `Remove-AppxPackage`.
  - Atualiza o arquivo `%APPDATA%\Geek Uninstaller\cache.dat` removendo o item do cache local.

### 2. Ações Pré-Desinstalação e Gestão de Processos (`src/services/process.rs`)
- **Detecção de Processos Concorrentes:**
  - Consulta a Win32 ToolHelp32 API (`CreateToolhelp32Snapshot`, `Process32FirstW`, `Process32NextW`) de forma nativa e sem janelas de console.
  - Identifica processos ativos relacionados ao executável principal ou ao nome do software.
- **Encerramento de Processos:**
  - `kill_processes`: encerra processos bloqueadores via `OpenProcess(PROCESS_TERMINATE)` e `TerminateProcess` antes da desinstalação.

### 3. Execução e Monitoramento Durante o Evento (`src/services/uninstaller.rs`)
- `ActiveUninstaller`: encapsula o processo filho (`Child`), permitindo polling não-bloqueante a cada frame via `child.try_wait()`.
- Parsing de executáveis com aspas, argumentos e comandos `msiexec /X{GUID}`.
- Modal de progresso com tempo decorrido, permitindo que a interface permaneça responsiva e com opção de pular espera para instaladores silenciosos.

### 4. Ações Pós-Desinstalação
- Executadas estritamente após a conclusão comprovada do desinstalador nativo:
  - Varredura profunda atualizada (`scan_leftover_traces`), inspecionando HKCU, HKLM (64 e WOW6432), pastas de instalação, `%APPDATA%`, `%LOCALAPPDATA%`, `%ProgramData%`, e atalhos no Menu Iniciar e Desktop.
  - Se nenhum vestígio for detectado: mensagem de sucesso direto e limpeza do registro.
  - Se houver vestígios: exibição detalhada para confirmação e exclusão real e atômica com relatório discriminado de sucessos e falhas.

## Testes e Validação
- Testes unitários adicionados para:
  - Criação, leitura, exclusão recursiva e confirmação de chave no Registro (`test_delete_registry_path_roundtrip`).
  - Criação e remoção forçada de pastas aninhadas com arquivos (`test_delete_filesystem_path_file_and_dir`).
  - Parsing de argumentos com aspas e espaços (`test_split_arguments_quotes_and_spaces`).
- Resultado: 9/9 testes aprovados com 100% de sucesso.

