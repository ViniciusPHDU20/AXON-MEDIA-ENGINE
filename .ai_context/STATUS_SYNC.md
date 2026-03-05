# AI SYSTEM SYNC - 05/03/2026
## Estado Técnico Atual
- **GPU:** NVIDIA RTX 3060 Ti (Driver 590.48.01, CUDA 13.1).
- **Ambiente:** Hyprland v0.54.0 (Wayland).
- **Problema Crítico:** Waybar v0.15.0 e SwayNC não estão sendo renderizados nas camadas (layers) do Hyprland.
- **Causa Provável:** Bug de coordenadas no Hyprland 0.54.0. Monitores DP-2 e HDMI-A-1 estavam em posições sobrepostas (0,1860 e 0,780).
- **Correções Realizadas:**
  - Substituição de `$HOME` por caminhos absolutos no JSON da Waybar.
  - Correção de `@import` de cores do Wallust (estavam apontando para pastas inexistentes).
  - Reposicionamento manual do monitor DP-2 para 0,0.

## Memória AI (God Mode)
Este diretório contém o `GEMINI.md` que define as diretrizes "God Mode / Deus Active". 
O usuário ViniciusPHDU20 prefere o tema "Purple Abyss" (Preto #1A0026, Roxo #8A2BE2) e foco em Zero Input Lag.

## Instruções Pós-Formatação
1. Restaurar o `TOKEN.gpg`.
2. Apontar o Gemini para o `GEMINI.md` neste diretório.
3. Verificar se o Hyprland 0.54.0 persiste com o erro de camadas (layers) em instalações limpas.
