# Rust Spider

Uma ferramenta para crawling e análise de sites.

## Instalação

1. Extraia o arquivo
2. Execute: ./install.sh

## Uso

web-crawler --url https://exemplo.com --depth 3 --output-dir results

## Opções

--url: URL inicial para crawling
--depth: Profundidade máxima (default: 5)
--timeout: Timeout em segundos (default: 1000)
--output-dir: Diretório para salvar resultados (default: results)

## Exemplos

Buscar formulários de login:
web-crawler --url https://exemplo.com --depth 2 --output-dir login_results

Análise completa:
web-crawler --url https://exemplo.com --depth 5 --timeout 30 --output-dir full_analysis