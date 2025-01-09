# TP2-Project
Projeto para o curso de Técnicas de Programação 2 da Universidade de Brasília.

## Estilo de programação passive aggressive (Rust)

- Vídeo para projeto: [https://youtu.be/xko9XKqME4U](https://youtu.be/xko9XKqME4U)
- [Instruções para o uso](##como-rodar)

O código desenvolvido para aplicar o estilo passive aggressive (em Rust) foi o Keyword in Context:

O algoritmo recebe uma lista de títulos ou frases (separados por linha) e gera uma lista alfabetizada de todas as palavras-chave com o seu contexto ao redor. Assim, dado um conjunto de frases em um arquivo:
```
The boy ran away
The cat ran after the dog
I like grapes
```
O código irá processar as frases, ignorando stop-words (palavras frequentes como "o", "a", "ele" etc. que não são importantes para serem contextualizadas) e deslocar circularmente tais frases, deixando a palavra-chave em contexto. A saída, então, seria:
```
away | The boy ran
boy ran away | The 
cat ran after the dog | The
dog | The cat ran after the 
grapes | I like
like grapes | I 
ran after the dog | The cat
ran away | The boy
```
em que o pipe ("|") indica o começo e fim da frase (para facilitar a leitura dos resultados). 
O código desenvolvido pelos alunos também:
- Permite que os usuários especifiquem sua própria lista de stop-words, ao digitar ```cargo run args``` seguido de flags ```-w``` ou ```--words``` (com o caminho para o arquivo de stop-words)
- Permite que os usuários especifiquem sua própria lista de stop-words, ao digitar ```cargo run args``` seguido de flags ```-p``` ou ```--phrases``` (com o caminho para o arquivo de frases)
- Implementa diferentes opções de ordenação, ao digitar ```cargo run args``` seguido de flags ```-s``` ou ```--sort``` é possível escolher um dos 4 tipos de ordenação:
	- ```alpha```: insensível a maiúsculas e minúsculas, ordem alfabética
  - ```sensitive```: sensível a maiúsculas e minúsculas, ordem alfabética
  - ```reverse_alpha```: insensível a maiúsculas e minúsculas, ordem inversa
  - ```reverse_sensitive```: sensível a maiúsculas e minúsculas, ordem inversa

- Permite que os usuários especifiquem o tamanho da janela de contexto ao redor da palavra-chave, ao digitar ```cargo run args``` seguido de flags ```-l``` ou ```--length``` e um inteiro positivo. Para as frases propostas acima, a saída para ```cargo run args -l 1``` é:
```
away The | ran
boy ran | The 
cat ran | The
dog The | the 
grapes I | like
like grapes | I 
ran after | cat
ran away | boy
```
em que o pipe ("|") agora indica a divisão entre palavras à direita e à esquerda da palavra-chave (considerando que a frase é circular). Vale notar que o uso de uma janela que cause repetições de palavras da frade resulta em erro. 

## Como rodar
Certifique-se que está na pasta keyword_in_context e use o comando ```cargo run``` para rodar sem argumentos ou ```cargo run args``` seguido de arumentos (use as flags ```-h``` ou ```--help``` para descobrir os possíveis argumentos)
- Use ```cargo test``` para rodar os testes unitários e de integração

### Uso em linux
Caso o código não funciona apenas com ```cargo run```, provavelmente houve um problema ao identificar o caminho do arquivo. Para isso use:
```cargo run args -p [caminho para o arquivo default sentences.txt (localizado em files)] -w [caminho para o arquivo default stop_words.txt (localizado em files)]```
