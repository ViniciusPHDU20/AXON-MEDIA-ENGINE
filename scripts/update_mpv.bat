@echo off
TITLE AXON ENGINE - MPV UPDATER (GITHUB MIRROR)
COLOR 0A

:: Usando o GitHub do Shinchiro (Muito mais confiavel para curl)
SET URL=https://github.com/shinchiro/mpv-winbuild-cmake/releases/download/2026-03-01/mpv-x86_64-v3-20260301-git-1234abc.7z
:: Como as datas mudam, vamos usar uma URL de um build estavel conhecido ou instruir o manual de forma clara.

echo [*] Iniciando download do Motor MPV via GitHub Mirror...
echo.

:: Tentativa com GitHub (Build Estavel Recente)
:: Nota: Vou usar a URL do SourceForge formatada para pular os intermediarios
curl -L "https://downloads.sourceforge.net/project/mpv-player-windows/stable/mpv-x86_64-latest.7z" -o "mpv.7z"

if %ERRORLEVEL% NEQ 0 (
    echo [!] Erro no download automatico.
    goto manual
)

:: Verifica se o arquivo e muito pequeno (HTML de erro)
for %%I in (mpv.7z) do if %%~zI LSS 1000000 (
    echo [!] O arquivo baixado parece corrompido (muito pequeno).
    goto manual
)

echo [*] Download concluido com sucesso!
echo [*] Extraia o 'mpv.7z' e copie 'mpv-1.dll' para a raiz como 'mpv.dll'.
explorer .
pause
exit

:manual
echo.
echo [!!!] O SourceForge bloqueou o download automatico [!!!]
echo.
echo Por favor, siga estes 3 passos simples:
echo 1. Acesse: https://sourceforge.net/projects/mpv-player-windows/files/stable/
echo 2. Baixe o primeiro arquivo da lista (ex: mpv-x86-64-v3-latest.7z)
echo 3. Extraia e coloque a 'mpv-1.dll' na pasta raiz renomeada para 'mpv.dll'
echo.
echo Abrindo o site para voce...
start https://sourceforge.net/projects/mpv-player-windows/files/stable/
pause
