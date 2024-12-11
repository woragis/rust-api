# API in Rust

## Boas Praticas de Documentacao

- Criar documento no word e anexar como pdf
- Criar diagramas com as relacoes do Banco de Dados
- Criar graficos com os dados exatos

## Features

- [x] - Password Manager
- [ ] - Video Stream
- [ ] - Live Stream
- [ ] - Store
- [ ] - Blog
- [ ] - News
- [ ] - Social Media **(Instagram Copy)**
- [ ] - Social Media **(Pinterest Copy)**
- [ ] - Social Media **(X Copy)**
- [ ] - Chat Bot
- [ ] - Chat Bot with Anime audio and image generation
- [ ] - Message
- [ ] - Notes
- [ ] - Habit Tracker

## Projects that use this API

Here are the projects that I built or will build that use this rust_api as the backend  
These projects are mostly frontend projects, some will be built for mobile

- [ ] - Ecommerce _Store_
- [ ] - G1 _News_
- [ ] - Blog _Blog_
- [ ] - Password Manager _Password Manager_
- [ ] - Youtube _Video Stream_
- [ ] - Pinterest _Social Media_
- [ ] - Instagram _Social Media_
- [ ] - Kicks _Live Stream_
- [ ] - Twitch.tv _Live Stream_
- [ ] - Cos.Tv _Video Stream_
- [ ] - Notes _Notes_
- [ ] - Habit Tracker _Habit Tracker_
- [ ] - Notes _Notes_
- [ ] - Whatsapp _Message_
- [ ] - Telegram _Message_
- [ ] - Signal _Message_

## Project Folder Structure

### Config

- DB
- JWT

### Db

- CONNECTION
- Tables
  - News
  - Store
  - Users

### Handlers

- Auth
- News
- Store
- Stories
- Video Stream
- Live Stream
- Notes
- Password Manager

### Models

- blog_post
- blog_post_like
- news
- fanfic website
- weather updates
- ai chat
- anime character chat bot
- web scraper?
- instagram like social media (user with its posts and comments and likes and saveds)
- pinterest like social media (user with its posts and comments and pins and likes)
- youtube like social media (videos, creator (user(id)), likes, comments)

### Routes

### Tests

- faker _cria infinitos dados realistas_
- reqwest _me permite testar as rotas_ e _criar unit tests_
- criar rota que inicia ativa os tests feito pelo reqwest em parceria com o faker

## Packages Description

```toml
[dependencies]
actix-web = "4.9.0"                                     // web builder
aes = "0.8.4"                                           // password_manager encrypt
bcrypt = "0.16.0"                                       // user authentication
bytes = "1.9.0"                                         // user model to_sql function
chrono = { version = "0.4.38", features = ["serde"] }   // models date
colored = "2.1.0"                                       // coloring log
dotenv = "0.15.0"                                       // save private variables
faker_rand = "0.1.1"                                    // tests
fern = "0.7.0"                                          // logger
hex = "0.4.3"                                           // decoder from bytes to string
jsonwebtoken = "9.3.0"                                  // user authentication
lettre = "0.11.10"                                      // auth and email
log = "0.4.22"                                          // logger
postgres-types = "0.2.8"                                // help models enum
rand = "0.8.5"                                          // tests
reqwest = "0.12.9"                                      // tests
serde = { version = "1.0.215", features = ["derive"] }  // json http deserializer and serializer
serde_json = "1.0.133"                                  // json http deserializer and serializer
tokio = { version = "1.41.1", features = ["full"] }     // async
tokio-postgres = { version = "0.7.12", features = ["with-chrono-0_4"] } // database connection
```
