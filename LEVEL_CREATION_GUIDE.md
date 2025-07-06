# Level Creation Guide

Este guia mostra como usar os novos sistemas de criação de levels implementados no jogo. Existem várias maneiras de criar levels, desde editores visuais até arquivos de texto simples.

## 🎮 1. Editor Visual In-Game

O editor visual permite criar levels diretamente no jogo usando mouse e teclado.

### Ativando o Editor

1. Execute o jogo
2. Pressione **F1** para ativar/desativar o editor
3. Quando ativo, você verá instruções no console

### Controles do Editor

| Tecla | Função |
|-------|--------|
| **F1** | Liga/desliga o editor |
| **Mouse Esquerdo** | Coloca tile atual |
| **Mouse Direito** | Remove tile |
| **1-9** | Seleciona tipo de tile |
| **[ / ]** | Diminui/aumenta tamanho do pincel |
| **S** | Salva level |
| **L** | Carrega level |
| **H** | Liga/desliga interface |

### Tiles Disponíveis

| Tecla | Tile | Descrição |
|-------|------|-----------|
| **1** | Grama (180) | Bloco sólido de grama |
| **2** | Pedra (176) | Bloco sólido de pedra |
| **3** | Tijolo (184) | Bloco sólido de tijolo |
| **4** | Plataforma (181) | Plataforma atravessável |
| **5** | Madeira (182) | Plataforma de madeira |
| **6** | Flor (183) | Decoração |
| **7** | Árvore (185) | Decoração |
| **8** | Cristal (187) | Decoração |
| **9** | Vazio (255) | Remove tile |

### Exemplo de Uso

```rust
// No seu main.rs, adicione os sistemas:
app.add_systems(Startup, setup_level_editor)
   .add_systems(Update, (
       toggle_level_editor,
       level_editor_input,
       level_editor_mouse,
       level_editor_save_load,
   ).run_if(in_state(GameState::Playing)));
```

## 📝 2. Criação com Símbolos (Texto)

Crie levels usando arquivos de texto simples com símbolos.

### Formato do Arquivo

Crie um arquivo `.txt` com símbolos representando diferentes tiles:

```
// Comentário - linhas começando com // são ignoradas
// Legenda de símbolos:
// . = Espaço vazio
// G = Grama
// S = Pedra
// P = Plataforma
// F = Flor
// T = Árvore

.....T.........F.........T.....
...............................
..........====.................
...............................
...#...............................
...#.....PPP.....====..............
...#...............................
GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG
SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
```

### Símbolos Disponíveis

| Símbolo | Tile | Descrição |
|---------|------|-----------|
| `.` | Vazio (255) | Espaço vazio |
| `G` | Grama (180) | Sua grama customizada |
| `S` | Pedra (176) | Pedra sólida |
| `B` | Tijolo (184) | Tijolo |
| `P` | Plataforma (181) | Plataforma |
| `W` | Madeira (182) | Madeira |
| `F` | Flor (183) | Flor decorativa |
| `T` | Árvore (185) | Árvore decorativa |
| `C` | Cristal (187) | Cristal decorativo |
| `#` | Parede (176) | Parede sólida |
| `=` | Plataforma H (181) | Plataforma horizontal |
| `^` | Espinhos (188) | Espinhos perigosos |
| `~` | Água (189) | Água |
| `*` | Especial (187) | Bloco especial |

### Carregando Levels com Símbolos

```rust
use crate::systems::level_parser::load_level_from_symbol_file;

// Carrega level do arquivo
match load_level_from_symbol_file("assets/levels/my_level.txt") {
    Ok(level_data) => {
        // Use level_data para spawnar tiles
        info!("Level carregado com sucesso!");
    },
    Err(e) => error!("Erro ao carregar level: {}", e),
}
```

## 🏗️ 3. Templates/Padrões

Use templates pré-definidos para criar estruturas comuns rapidamente.

### Templates Disponíveis

```rust
use crate::systems::level_templates::*;

// Cria templates comuns
let templates = create_common_templates();

// Plataforma de chão
let ground = LevelTemplate::ground_platform(10, 180, 176); // largura, grama, pedra

// Plataforma flutuante
let floating = LevelTemplate::floating_platform(5, 181); // largura, tile_plataforma

// Pilar
let pillar = LevelTemplate::pillar(6, 176); // altura, tile_pedra

// Escada
let stairs = LevelTemplate::staircase(5, 5, 176); // largura, altura, tile

// Sala
let room = LevelTemplate::room(8, 6, 176, 180); // largura, altura, parede, chão

// Ponte
let bridge = LevelTemplate::bridge(8, 181, 176); // largura, plataforma, suporte

// Torre
let tower = LevelTemplate::tower(4, 12, 176, 180); // largura, altura, parede, chão
```

### Usando Templates

```rust
// Cria level vazio
let mut level_data = LevelData {
    width: 50,
    height: 20,
    tiles: vec![vec![255; 50]; 20],
};

// Coloca templates
let ground_template = LevelTemplate::ground_platform(10, 180, 176);
place_template(&mut level_data, &ground_template, 0, 18); // x, y

let platform_template = LevelTemplate::floating_platform(5, 181);
place_template(&mut level_data, &platform_template, 15, 12);

// Ou cria level automático com templates
let template_level = create_template_level(50, 20);
```

## 🗺️ 4. Tiled Map Editor

Use o Tiled Map Editor para criar levels visualmente e importá-los.

### Instalação do Tiled

1. Baixe o Tiled: https://www.mapeditor.org/
2. Instale e abra o programa
3. Crie um novo mapa

### Configuração do Tiled

1. **Criar Novo Mapa**:
   - Orientação: Orthogonal
   - Tamanho do tile: 16x16
   - Tamanho do mapa: 50x20 (exemplo)

2. **Adicionar Tileset**:
   - Imagem: `assets/scene/tileset.png`
   - Tamanho do tile: 16x16

3. **Mapear Tiles**:
   - Lembre-se das posições dos seus tiles personalizados
   - Grama está na linha 11, coluna 4 (índice 180)

### Exportando do Tiled

1. Vá em **File → Export As**
2. Escolha **JSON map files (*.json)**
3. Salve em `assets/levels/`

### Carregando no Jogo

```rust
use crate::systems::tiled_loader::*;

// Carrega mapa do Tiled
match load_tiled_map("assets/levels/my_map.json") {
    Ok(tiled_map) => {
        // Cria mapeamento de tiles
        let tile_mapping = create_tile_mapping();
        
        // Converte para formato do jogo
        match tiled_map_to_level_data_with_mapping(&tiled_map, &tile_mapping) {
            Ok(level_data) => {
                info!("Mapa Tiled carregado com sucesso!");
                // Use level_data
            },
            Err(e) => error!("Erro na conversão: {}", e),
        }
    },
    Err(e) => error!("Erro ao carregar Tiled: {}", e),
}
```

### Mapeamento de Tiles

Personalize o mapeamento entre tiles do Tiled e do jogo:

```rust
// Em tiled_loader.rs, função create_tile_mapping()
let mut mapping = HashMap::new();

// Tiled ID -> Game ID
mapping.insert(0, 180);  // Primeiro tile do Tiled -> Sua grama
mapping.insert(1, 176);  // Segundo tile -> Pedra
mapping.insert(2, 184);  // Terceiro tile -> Tijolo
// ... adicione mais conforme necessário
```

## 🔧 Configuração nos Sistemas

### Adicionando ao main.rs

```rust
use bevy::prelude::*;
use crate::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            setup_graphics,
            setup_physics,
            setup_level_editor,
            // ... outros sistemas
        ))
        .add_systems(Update, (
            // Editor in-game
            toggle_level_editor,
            level_editor_input,
            level_editor_mouse,
            level_editor_save_load,
            
            // ... outros sistemas
        ))
        .run();
}
```

## 📁 Estrutura de Arquivos

```
bevy_sidescroller/
├── assets/
│   └── levels/
│       ├── sample_level.txt          # Exemplo com símbolos
│       ├── my_level.txt              # Seus levels
│       ├── tiled_map.json            # Mapas do Tiled
│       └── editor_level.csv          # Salvos do editor
├── src/
│   └── systems/
│       ├── level_editor.rs           # Editor visual
│       ├── level_parser.rs           # Parser de símbolos
│       ├── level_templates.rs        # Templates
│       ├── tiled_loader.rs           # Carregador Tiled
│       └── ...
└── LEVEL_CREATION_GUIDE.md          # Este guia
```

## 💡 Dicas e Truques

### Personalizando Tiles

Para mudar as posições dos tiles no tileset, edite as constantes em `level_loader.rs`:

```rust
// Sua grama está na linha 11, coluna 4
const GRASS_TILES: [u32; 4] = [180, 181, 182, 183]; // Índices calculados
```

### Criando Novos Símbolos

Para adicionar novos símbolos ao parser:

```rust
// Em level_parser.rs, função LevelSymbolMap::new()
symbols.insert('X', 200);  // Novo símbolo X para tile 200
```

### Salvando Levels

O editor salva automaticamente em formato CSV. Para converter:

```rust
// CSV -> Símbolos
let level_data = load_level_from_file("assets/levels/editor_level.csv")?;
save_level_to_symbol_file(&level_data, "assets/levels/my_level.txt")?;
```

### Debugando Levels

Use o sistema de debug para visualizar tiles:

```rust
// Pressione F3 no jogo para ver informações de tiles
// Pressione F4 para ver grid de tiles
// Pressione F5 para ver colisões
```

## 🐛 Solução de Problemas

### Tiles não aparecem
- Verifique se os índices estão corretos
- Confirme que o tileset está carregado
- Veja se as coordenadas não estão fora dos limites

### Editor não funciona
- Certifique-se de que o sistema está adicionado ao App
- Verifique se o estado do jogo permite o editor
- Teste com F1 para ativar

### Arquivos não carregam
- Confirme que o caminho está correto
- Verifique se o arquivo existe em `assets/levels/`
- Teste a validação do formato

### Tiled não importa
- Certifique-se de exportar como JSON
- Verifique se o mapeamento de tiles está correto
- Confirme que as dimensões estão corretas

## 🎯 Próximos Passos

1. **Experimente** cada método de criação
2. **Personalize** os tiles para seu tileset
3. **Crie** seus próprios templates
4. **Combine** métodos para máxima flexibilidade
5. **Compartilhe** seus levels com a comunidade!

---

**Divirta-se criando levels! 🎮**