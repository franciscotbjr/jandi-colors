# Contribuindo para Jandí Colors

Obrigado pelo interesse em contribuir!

## Como contribuir

### Novos formatos de token
Se você precisa da paleta em um formato que ainda não existe (ex: Flutter/Dart, Figma plugin, CSS-in-JS), abra uma issue descrevendo o formato e, se possível, envie um PR com o arquivo de token.

### Correções de acessibilidade
Se encontrou uma combinação de cores que não atende WCAG ou um erro na tabela de contraste, por favor reporte.

### Documentação e tradução
O README principal está em português. Traduções para outros idiomas são bem-vindas como arquivos separados (`README.en.md`, `README.es.md`).

### Pesquisa etnobotânica
Se você tem referências acadêmicas adicionais sobre o uso do jenipapo como pigmento, especialmente de comunidades indígenas específicas, adoraríamos incluir na documentação (com os devidos créditos).

## Regras

- Mantenha os valores hex consistentes em todos os formatos
- Novos tokens devem incluir documentação inline
- Respeite a narrativa cultural — os nomes dos tons não devem ser alterados

## Estrutura

```
tokens/
├── css/         → CSS custom properties
├── scss/        → SCSS variables + mixins
├── json/        → W3C Design Tokens format
├── tailwind/    → Tailwind CSS plugin
├── swift/       → SwiftUI + UIKit
├── kotlin/      → Jetpack Compose
└── rust/        → Rust const
```

Cada novo formato segue o mesmo padrão: um arquivo autocontido com todos os 8 tons, documentação inline, e referência ao repositório.
